use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader};
use serde::Deserialize;
use chrono::DateTime;
use reqwest::Client;
use ethers::prelude::*;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use std::env;
use axum::{Router, routing::get, response::Json};
use serde_json::json;
use std::net::SocketAddr;

abigen!(
    SensorStorage,
    "./build/contracts/SensorStorage.json"
);

#[derive(Deserialize, Debug, Clone)]
struct SensorPayload {
    timestamp: String,
    sensor_id: String,
    location: String,
    stage: String,
    temperature: f32,
    humidity: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let rpc_url = env::var("RPC_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let contract_address: Address = env::var("CONTRACT_ADDRESS")?.parse()?;

    let provider = Provider::<Http>::try_from(rpc_url)?.interval(std::time::Duration::from_millis(10));
    let wallet = private_key.parse::<LocalWallet>()?;
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(1337u64));
    let client = Arc::new(client);
    let contract = SensorStorage::new(contract_address, client.clone());

    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    let client_http = Client::new();

    let influx_url = "http://localhost:8086/api/v2/write?org=yusron&bucket=yusrondb&precision=s";
    let token = "eUzG1LkEN9q5m7PHUnOVs543d1188paEBd7q_p4s8PpzdYiZlF65cc_0RK-kD74KMR7dYOJ0BsOW-k6slXlIGw==";

    let latest_data: Arc<Mutex<Option<SensorPayload>>> = Arc::new(Mutex::new(None));

    let api_data = latest_data.clone();
    let app = Router::new().route("/latest", get(move || {
        let data = api_data.lock().unwrap().clone();
        async move {
            if let Some(d) = data {
                Json(json!({
                    "timestamp": d.timestamp,
                    "sensor_id": d.sensor_id,
                    "location": d.location,
                    "stage": d.stage,
                    "temperature": d.temperature,
                    "humidity": d.humidity
                }))
            } else {
                Json(json!({"error": "No data yet"}))
            }
        }
    }));

    tokio::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
        println!("🌐 HTTP endpoint aktif di http://{}/latest", addr);
        axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
    });

    println!("🟢 Server TCP aktif di port 9000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("🔌 Koneksi dari: {}", addr);

        let client_http = client_http.clone();
        let influx_url = influx_url.to_string();
        let token = token.to_string();
        let contract = contract.clone();
        let latest_data = latest_data.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(socket);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                if let Ok(data) = serde_json::from_str::<SensorPayload>(&line) {
                    println!("📥 Data masuk: {:?}", data);

                    let timestamp = DateTime::parse_from_rfc3339(&data.timestamp).unwrap().timestamp();

                    let line_protocol = format!(
                        "sensor_data,sensor_id={},location={},stage={} temperature={},humidity={} {}",
                        data.sensor_id.replace(" ", "_"),
                        data.location.replace(" ", "_"),
                        data.stage.replace(" ", "_"),
                        data.temperature,
                        data.humidity,
                        timestamp
                    );

                    let res = client_http
                        .post(&influx_url)
                        .header("Authorization", format!("Token {}", token))
                        .header("Content-Type", "text/plain")
                        .body(line_protocol)
                        .send()
                        .await;

                    match res {
                        Ok(r) if r.status().is_success() => println!("✅ InfluxDB OK"),
                        Ok(r) => println!("⚠️ InfluxDB Gagal: {}", r.status()),
                        Err(e) => println!("❌ Error HTTP: {}", e),
                    }

                    match contract
                        .store_data(
                            data.sensor_id.clone(),
                            (data.temperature as i64).into(),
                            (data.humidity as i64).into(),
                        )
                        .send()
                        .await
                    {
                        Ok(pending_tx) => {
                            println!("✅ Blockchain TX sent: {:?}", pending_tx.tx_hash());
                        }
                        Err(e) => {
                            println!("❌ Gagal kirim ke blockchain: {}", e);
                        }
                    }

                    *latest_data.lock().unwrap() = Some(data);
                } else {
                    println!("❌ JSON invalid: {}", line);
                }
            }
        });
    }
}
