// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract SensorStorage {
    struct SensorData {
        uint256 timestamp;
        string sensorId;
        int256 temperature;
        int256 humidity;
    }

    SensorData[] public dataLog;

    event DataStored(
        uint256 indexed index,
        string sensorId,
        int256 temperature,
        int256 humidity,
        uint256 timestamp
    );

    function storeData(
        string memory sensorId,
        int256 temperature,
        int256 humidity
    ) public {
        SensorData memory entry = SensorData(
            block.timestamp,
            sensorId,
            temperature,
            humidity
        );
        dataLog.push(entry);
        emit DataStored(
            dataLog.length - 1,
            sensorId,
            temperature,
            humidity,
            block.timestamp
        );
    }

    function getLatest()
        public
        view
        returns (
            uint256 timestamp,
            string memory sensorId,
            int256 temperature,
            int256 humidity
        )
    {
        require(dataLog.length > 0, "No data");
        SensorData memory d = dataLog[dataLog.length - 1];
        return (d.timestamp, d.sensorId, d.temperature, d.humidity);
    }

    function getCount() public view returns (uint256) {
        return dataLog.length;
    }
}
