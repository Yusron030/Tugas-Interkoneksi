// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract IPFS {
    struct Record {
        address sender;
        string cid;
        uint256 timestamp;
    }

    Record[] public records;

    event CIDStored(address indexed sender, string cid, uint256 timestamp);

    function storeCID(string memory cid) public {
        records.push(Record(msg.sender, cid, block.timestamp));
        emit CIDStored(msg.sender, cid, block.timestamp);
    }

    function getCID(uint index) public view returns (address sender, string memory cid, uint256 timestamp) {
        require(index < records.length, "Index out of bounds");
        Record memory r = records[index];
        return (r.sender, r.cid, r.timestamp);
    }

    function getTotalCID() public view returns (uint) {
        return records.length;
    }
}
