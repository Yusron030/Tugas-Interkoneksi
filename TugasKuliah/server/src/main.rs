use std::{error::Error, sync::Arc};
use chrono::DateTime;
use ethers::{
    prelude::*,
    types::Address,
};
use reqwest::Client;
use sha2::{Sha256, Digest};
use tokio::{io::{AsyncBufReadExt, BufReader}, net::TcpListener};
use serde::{Deserialize, Serialize};

// 🧠 ABI Binding
abigen!(
    SensorDataContract,
    "abis/SensorData.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    IPFSContract,
    "abis/IPFS.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Deserialize, Serialize, Debug)]
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f32,
    humidity_percent: f32,
}

async fn upload_to_pinata(json_data: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = "https://api.pinata.cloud/pinning/pinJSONToIPFS";

    let api_key = "0288752368907263144b";
    let api_secret = "beac7f49a525f0e3e8f1bcfcabaa09b78e9d35ee2190280805882c7c598d2063";

    let payload = serde_json::json!({
        "pinataMetadata": {
            "name": "sensor.json"
        },
        "pinataContent": serde_json::from_str::<serde_json::Value>(json_data)?
    });

    let res = client.post(url)
        .header("Content-Type", "application/json")
        .header("pinata_api_key", api_key)
        .header("pinata_secret_api_key", api_secret)
        .json(&payload)
        .send()
        .await?;

    let text = res.text().await?;
    println!("📦 [Pinata Raw Response] {}", text);

    let json: serde_json::Value = serde_json::from_str(&text)?;
    let ipfs_hash = json["IpfsHash"].as_str().unwrap_or_default().to_string();

    Ok(ipfs_hash)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ✅ Web3 setup
    let provider = Provider::<Http>::try_from("http://127.0.0.1:7545")?;
    let wallet: LocalWallet = "980e4170bb060f00799b20c5490eeda3b66ffe95ac36b9a66b2a3dde48c8a43a"
        .parse::<LocalWallet>()?
        .with_chain_id(1337u64);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // ✅ Contract bindings
    let sensor_addr: Address = "0xD5784F849D4a718a66e51Cd9908087f653578C25".parse()?;
    let ipfs_addr: Address = "0xe1c05F7183735296887450154D9E4c91D08AB403".parse()?;
    let sensor_contract = Arc::new(SensorDataContract::new(sensor_addr, client.clone()));
    let ipfs_contract = Arc::new(IPFSContract::new(ipfs_addr, client.clone()));

    // ✅ InfluxDB config
    let infl_url = "http://localhost:8086/api/v2/write?org=yusron&bucket=yusron&precision=s";
    let token    = "xYTcBBRwn8tDObeuKmW3frew0nQ9WO3QefsPQ_RhC-eyswf2RGMjQL9_eH4NsTnL22BJ2xAvqY3BXY7Fcdh07Q==";
    let http_cli = Client::new();

    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    println!("🖥️ Server listening on :9000");

    loop {
        let (sock, addr) = listener.accept().await?;
        println!("🔗 Connected: {}", addr);

        let infl_url = infl_url.to_string();
        let token    = token.to_string();
        let http_cli = http_cli.clone();
        let sensor_contract = sensor_contract.clone();
        let ipfs_contract = ipfs_contract.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(sock);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                match serde_json::from_str::<SensorData>(&line) {
                    Ok(data) => {
                        println!("📥 {:?}", data);

                        // ✅ InfluxDB Write
                        let ts = DateTime::parse_from_rfc3339(&data.timestamp).unwrap().timestamp();
                        let lp = format!(
                            "monitoring,sensor_id={},location={},stage={} temperature={},humidity={} {}",
                            data.sensor_id.replace(" ", "\\ "),
                            data.location.replace(" ", "\\ "),
                            data.process_stage.replace(" ", "\\ "),
                            data.temperature_celsius,
                            data.humidity_percent,
                            ts
                        );
                        let resp = http_cli.post(&infl_url)
                            .header("Authorization", format!("Token {}", token))
                            .body(lp)
                            .send().await;

                        match resp {
                            Ok(r) if r.status().is_success() => println!("✅ Ditulis ke InfluxDB"),
                            Ok(r) => eprintln!("⚠️ InfluxDB error: {}", r.status()),
                            Err(e) => eprintln!("❌ Reqwest error: {}", e),
                        }

                        // ✅ IPFS Upload
                        let json = serde_json::to_string(&data).unwrap();
                        match upload_to_pinata(&json).await {
                            Ok(cid) => {
                                println!("📦 IPFS CID: {}", cid);

                                // ✅ Optional: hash data (masih bisa disimpan juga)
                                let mut hasher = Sha256::new();
                                hasher.update(json.as_bytes());
                                let hash_bytes: [u8; 32] = hasher.finalize().into();

                                // ✅ Simpan ke contract SensorData
                                let temp_scaled = (data.temperature_celsius * 10.0) as u16;
                                let hum_scaled  = (data.humidity_percent * 10.0) as u16;

                                match sensor_contract
                                    .publish(
                                        data.sensor_id.clone(),
                                        temp_scaled,
                                        hum_scaled,
                                        data.timestamp.clone(),
                                        hash_bytes.into(), // atau cid.as_bytes() jika kontrak mendukung string
                                    )
                                    .send()
                                    .await
                                {
                                    Ok(tx) => println!("🔗 On-chain tx hash: {:?}", tx.tx_hash()),
                                    Err(e) => eprintln!("❌ SensorData contract error: {:?}", e),
                                }

                                // ✅ Simpan CID ke IPFSContract
                                match ipfs_contract
                                    .store_cid(cid.clone())
                                    .send()
                                    .await
                                {
                                    Ok(tx) => println!("🔗 CID stored on-chain: {:?}", tx.tx_hash()),
                                    Err(e) => eprintln!("❌ IPFS contract error: {:?}", e),
                                }
                            }
                            Err(e) => eprintln!("❌ IPFS Upload error: {:?}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ JSON parse error: {}", e),
                }
            }
        });
    }
}
