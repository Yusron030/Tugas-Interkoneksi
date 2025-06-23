import React, { useEffect, useState } from "react";
import Web3 from "web3";
import {
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer,
} from "recharts";
import IPFSContract from "./abis/IPFS.json";
import SensorDataContract from "./abis/SensorData.json";
import "./style.css";

// ✅ Alamat kontrak
const IPFS_ADDRESS = "0xe1c05F7183735296887450154D9E4c91D08AB403";
const SENSOR_ADDRESS = "0xD5784F849D4a718a66e51Cd9908087f653578C25";

function App() {
  const [account, setAccount] = useState("");
  const [cidList, setCidList] = useState([]);
  const [sensorList, setSensorList] = useState([]);

  useEffect(() => {
    async function loadBlockchain() {
      if (!window.ethereum) return alert("🦊 Install MetaMask!");

      const web3 = new Web3(window.ethereum);
      const existing = await web3.eth.getAccounts();
      if (!existing.length) await window.ethereum.request({ method: "eth_requestAccounts" });

      const accounts = await web3.eth.getAccounts();
      setAccount(accounts[0]);

      const ipfsContract = new web3.eth.Contract(IPFSContract.abi, IPFS_ADDRESS);
      const sensorContract = new web3.eth.Contract(SensorDataContract.abi, SENSOR_ADDRESS);

      const cidEvents = await ipfsContract.getPastEvents("CIDStored", { fromBlock: 0, toBlock: "latest" });
      setCidList(cidEvents.map(e => e.returnValues.cid));

      const events = await sensorContract.getPastEvents("DataPublished", { fromBlock: 0, toBlock: "latest" });
const sensors = events.map((e, i) => ({
  sensorId: e.returnValues.sensorId,
  temperature: Number(e.returnValues.temperature) / 10,
  humidity: Number(e.returnValues.humidity) / 10,
  index: i, // ← pakai index buat grafik
  timestamp: e.returnValues.timestamp, // tetap string, misalnya: "2025-06-23T12:34:56Z"
  hash: e.returnValues.dataHash,
}));

      setSensorList(sensors);
    }

    loadBlockchain();
  }, []);

return (
  <div className="container">
    <header>
      <h2>🌐 Web3 Sensor Dashboard</h2>
      <p><strong>Connected Wallet:</strong> {account || "Not connected"}</p>
    </header>

    <section className="card">
      <h3>📦 IPFS CIDs</h3>
      {cidList.length ? (
        <div className="scroll-box">
          <ul>
            {cidList.map((cid, i) => (
              <li key={i}>
                <a href={`https://gateway.pinata.cloud/ipfs/${cid}`} target="_blank" rel="noopener noreferrer">
                  {cid}
                </a>
              </li>
            ))}
          </ul>
        </div>
      ) : <p>No CIDs stored yet.</p>}
    </section>

    <section className="card">
      <h3>📊 Sensor Data</h3>
      {sensorList.length ? (
        <>
<ResponsiveContainer width="100%" height={300}>
  <LineChart data={sensorList}>
    <CartesianGrid strokeDasharray="3 3" />
    <XAxis
      dataKey="index"
      tick={{ fontSize: 10 }}
      tickFormatter={(i) => {
        const d = sensorList[i];
        return d ? d.timestamp.slice(11, 19) : "";
      }}
    />
    <YAxis />
    <Tooltip
      labelFormatter={(i) => sensorList[i]?.timestamp || ""}
    />
    <Legend />
    <Line type="monotone" dataKey="temperature" name="Temp °C" stroke="#ff6b6b" />
    <Line type="monotone" dataKey="humidity" name="Humidity %" stroke="#1e90ff" />
  </LineChart>
</ResponsiveContainer>


          <div className="scroll-box">
            <table className="sensor-table">
              <thead>
                <tr>
                  <th>ID</th><th>Temp (°C)</th><th>Hum (%)</th><th>Time</th><th>Hash</th>
                </tr>
              </thead>
              <tbody>
                {sensorList.map((d, i) => (
                  <tr key={i}>
                    <td>{d.sensorId}</td>
                    <td>{d.temperature}</td>
                    <td>{d.humidity}</td>
                    <td>{d.timestamp}</td>
                    <td><code>{d.hash && d.hash !== `0x${"0".repeat(64)}` ? d.hash.slice(0, 12)+'...' : ''}</code></td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </>
      ) : <p>No sensor data recorded yet.</p>}
    </section>
  </div>
);
}

export default App;
