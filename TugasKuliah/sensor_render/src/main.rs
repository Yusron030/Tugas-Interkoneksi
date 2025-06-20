use tokio_serial::{SerialPortBuilderExt, DataBits, Parity, StopBits};
use tokio_modbus::prelude::*;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde::Serialize;
use chrono::Utc;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[derive(Serialize, Debug)]
struct SensorPayload {
    timestamp: String,
    sensor_id: String,
    location: String,
    stage: String,
    temperature: f32,
    humidity: f32,
}

async fn read_sht20_rs485(slave_id: u8) -> Result<(f32, f32), Box<dyn Error>> {
    let serial = tokio_serial::new("/dev/ttyUSB0", 9600)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(std::time::Duration::from_secs(1))
        .open_native_async()?;

    let mut ctx = rtu::connect_slave(serial, Slave(slave_id)).await?;
    let raw = ctx.read_input_registers(1, 2).await?;

    let temperature = raw[0] as f32 / 10.0;
    let humidity = raw[1] as f32 / 10.0;

    Ok((temperature, humidity))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        match read_sht20_rs485(1).await {
            Ok((temp, hum)) => {
                let data = SensorPayload {
                    timestamp: Utc::now().to_rfc3339(),
                    sensor_id: "SHT20-001".to_string(),
                    location: "Gudang-A".to_string(),
                    stage: "Fermentasi".to_string(),
                    temperature: temp,
                    humidity: hum,
                };

                let json = serde_json::to_string(&data)?;

                match TcpStream::connect("127.0.0.1:9000").await {
                    Ok(mut stream) => {
                        stream.write_all(json.as_bytes()).await?;
                        stream.write_all(b"\n").await?;
                        println!("✅ Data terkirim: {:?}", data);
                    }
                    Err(e) => {
                        eprintln!("❌ TCP gagal: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Gagal baca sensor: {}", e);
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}
