#!/usr/bin/env python3
import sys
import requests
import threading
import time
import csv
from io import StringIO
from collections import deque

from PyQt5.QtWidgets import (
    QApplication, QWidget, QLabel, QVBoxLayout, QHBoxLayout, QGroupBox
)
from PyQt5.QtCore import Qt, QTimer
from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg as FigureCanvas
from matplotlib.figure import Figure

# ⚙️ Konfigurasi InfluxDB
INFLUX_QUERY_URL = "http://localhost:8086/api/v2/query"
ORG    = "yusron"
BUCKET = "yusron"
TOKEN  = "xYTcBBRwn8tDObeuKmW3frew0nQ9WO3QefsPQ_RhC-eyswf2RGMjQL9_eH4NsTnL22BJ2xAvqY3BXY7Fcdh07Q=="

# Filter Sensor
data_measurement = "monitoring"
SENSOR_ID = "SHT20-001"
LOCATION  = "Gudang Fermentasi"
STAGE     = "Fermentasi"

# History max 50 data
temp_history = deque(maxlen=50)
hum_history  = deque(maxlen=50)
time_history = deque(maxlen=50)

def get_latest():
    query = f'''
from(bucket: "{BUCKET}")
  |> range(start: -10m)
  |> filter(fn: (r) => r._measurement == "{data_measurement}")
  |> filter(fn: (r) => r["sensor_id"] == "{SENSOR_ID}")
  |> filter(fn: (r) => r["location"] == "{LOCATION}")
  |> filter(fn: (r) => r["stage"] == "{STAGE}")
  |> filter(fn: (r) => r._field == "temperature" or r._field == "humidity")
  |> last()
'''
    headers = {
        "Authorization": f"Token {TOKEN}",
        "Content-Type":  "application/vnd.flux",
        "Accept":        "application/csv"
    }
    try:
        res = requests.post(
            INFLUX_QUERY_URL,
            params={"org": ORG},
            headers=headers,
            data=query
        )
        res.raise_for_status()
        reader = csv.DictReader(StringIO(res.text))
        data = {}
        for row in reader:
            if "_field" in row and "_value" in row:
                data[row["_field"]] = float(row["_value"])
        return data.get("temperature"), data.get("humidity")
    except Exception as e:
        print(f"Error fetching data: {e}")
        return None, None

class MonitoringApp(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("🌡️ Monitoring Suhu & Kelembaban")
        self.setGeometry(100, 100, 900, 600)
        self.initUI()
        self.start_update_thread()

    def initUI(self):
        layout = QVBoxLayout()

        # === Informasi ===
        info_box = QGroupBox("Informasi Sensor")
        info_layout = QHBoxLayout()

        self.temp_label = QLabel("Suhu: -- °C")
        self.hum_label = QLabel("Kelembaban: -- %")
        self.status_label = QLabel("Status: ...")

        for label in [self.temp_label, self.hum_label, self.status_label]:
            label.setStyleSheet("font-size: 16px; padding: 4px;")

        info_layout.addWidget(self.temp_label)
        info_layout.addWidget(self.hum_label)
        info_layout.addWidget(self.status_label)
        info_box.setLayout(info_layout)

        # === Grafik ===
        self.fig = Figure(figsize=(5, 4), dpi=100)
        self.ax1 = self.fig.add_subplot(211)
        self.ax2 = self.fig.add_subplot(212)
        self.canvas = FigureCanvas(self.fig)

        # === Gabung Layout ===
        layout.addWidget(info_box)
        layout.addWidget(self.canvas)
        self.setLayout(layout)

    def start_update_thread(self):
        self.timer = QTimer()
        self.timer.timeout.connect(self.update_data)
        self.timer.start(2000)  # 2 detik

    def update_data(self):
        temp, hum = get_latest()
        now = time.strftime("%H:%M:%S")
        if temp is not None and hum is not None:
            self.temp_label.setText(f"Suhu: {temp:.1f} °C")
            self.hum_label.setText(f"Kelembaban: {hum:.1f} %")
            self.status_label.setText("✅ Data OK")
            temp_history.append(temp)
            hum_history.append(hum)
            time_history.append(now)
            self.plot_data()
        else:
            self.status_label.setText("❌ Tidak ada data")

    def plot_data(self):
        self.ax1.clear(); self.ax2.clear()
        self.ax1.plot(time_history, temp_history, marker="o", color="tomato")
        self.ax2.plot(time_history, hum_history, marker="x", color="skyblue")
        self.ax1.set_title("Suhu (°C)")
        self.ax2.set_title("Kelembaban (%)")
        self.ax1.tick_params(axis='x', rotation=45)
        self.ax2.tick_params(axis='x', rotation=45)
        self.fig.tight_layout()
        self.canvas.draw()

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = MonitoringApp()
    window.show()
    sys.exit(app.exec_())
