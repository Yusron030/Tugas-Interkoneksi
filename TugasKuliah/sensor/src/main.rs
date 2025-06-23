use tokio_modbus::{client::rtu, prelude::*};
use tokio_serial::{SerialPortBuilderExt, Parity, StopBits, DataBits};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f32,
    humidity_percent: f32,
}

async fn read_sensor_data() -> Result<SensorData, Box<dyn Error>> {
    let builder = tokio_serial::new("/dev/ttyUSB0", 9600)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight)
        .timeout(std::time::Duration::from_secs(2));

    let port = builder.open_native_async()?;
    let mut ctx = rtu::connect_slave(port, Slave(1)).await?;
    let regs = ctx.read_input_registers(1, 2).await?;

    let temp = regs[0] as f32 / 10.0;
    let hum  = regs[1] as f32 / 10.0;

    Ok(SensorData {
        timestamp: Utc::now().to_rfc3339(),
        sensor_id: "SHT20-001".into(),
        location: "Gudang Fermentasi".into(),
        process_stage: "Fermentasi".into(),
        temperature_celsius: temp,
        humidity_percent: hum,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let data = read_sensor_data().await?;
        let mut json = serde_json::to_string(&data)?;
        json.push('\n'); // penting: newline

        match TcpStream::connect("127.0.0.1:9000").await {
            Ok(mut s) => {
                s.write_all(json.as_bytes()).await?;
                println!("✅ Kirim: {}", json.trim());
            }
            Err(e) => eprintln!("❌ Gagal connect ke server: {}", e),
        }

        sleep(Duration::from_secs(2)).await;
    }
}
