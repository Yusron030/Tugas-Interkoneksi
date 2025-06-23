// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract SensorData {
    struct Record {
        string sensorId;
        uint16 temperature;
        uint16 humidity;
        string timestamp;
        bytes32 dataHash;
    }

    Record[] private records;

    event DataPublished(
        string sensorId,
        uint16 temperature,
        uint16 humidity,
        string timestamp,
        bytes32 dataHash
    );

    function publish(
        string memory sensorId,
        uint16 temperature,
        uint16 humidity,
        string memory timestamp,
        bytes32 dataHash
    ) public {
        records.push(Record(sensorId, temperature, humidity, timestamp, dataHash));
        emit DataPublished(sensorId, temperature, humidity, timestamp, dataHash);
    }

    function getRecord(uint index) public view returns (
        string memory sensorId,
        uint16 temperature,
        uint16 humidity,
        string memory timestamp,
        bytes32 dataHash
    ) {
        require(index < records.length, "Invalid index");
        Record storage rec = records[index];
        return (
            rec.sensorId,
            rec.temperature,
            rec.humidity,
            rec.timestamp,
            rec.dataHash
        );
    }

    function getCount() public view returns (uint) {
        return records.length;
    }
}
