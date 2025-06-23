{
  "contractName": "SensorData",
  "abi": [
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "string",
          "name": "sensorId",
          "type": "string"
        },
        {
          "indexed": false,
          "internalType": "uint16",
          "name": "temperature",
          "type": "uint16"
        },
        {
          "indexed": false,
          "internalType": "uint16",
          "name": "humidity",
          "type": "uint16"
        },
        {
          "indexed": false,
          "internalType": "string",
          "name": "timestamp",
          "type": "string"
        },
        {
          "indexed": false,
          "internalType": "bytes32",
          "name": "dataHash",
          "type": "bytes32"
        }
      ],
      "name": "DataPublished",
      "type": "event"
    },
    {
      "inputs": [
        {
          "internalType": "string",
          "name": "sensorId",
          "type": "string"
        },
        {
          "internalType": "uint16",
          "name": "temperature",
          "type": "uint16"
        },
        {
          "internalType": "uint16",
          "name": "humidity",
          "type": "uint16"
        },
        {
          "internalType": "string",
          "name": "timestamp",
          "type": "string"
        },
        {
          "internalType": "bytes32",
          "name": "dataHash",
          "type": "bytes32"
        }
      ],
      "name": "publish",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256",
          "name": "index",
          "type": "uint256"
        }
      ],
      "name": "getRecord",
      "outputs": [
        {
          "internalType": "string",
          "name": "sensorId",
          "type": "string"
        },
        {
          "internalType": "uint16",
          "name": "temperature",
          "type": "uint16"
        },
        {
          "internalType": "uint16",
          "name": "humidity",
          "type": "uint16"
        },
        {
          "internalType": "string",
          "name": "timestamp",
          "type": "string"
        },
        {
          "internalType": "bytes32",
          "name": "dataHash",
          "type": "bytes32"
        }
      ],
      "stateMutability": "view",
      "type": "function",
      "constant": true
    },
    {
      "inputs": [],
      "name": "getCount",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "",
          "type": "uint256"
        }
      ],
      "stateMutability": "view",
      "type": "function",
      "constant": true
    }
  ],
  "metadata": "{\"compiler\":{\"version\":\"0.8.19+commit.7dd6d404\"},\"language\":\"Solidity\",\"output\":{\"abi\":[{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"string\",\"name\":\"sensorId\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"uint16\",\"name\":\"temperature\",\"type\":\"uint16\"},{\"indexed\":false,\"internalType\":\"uint16\",\"name\":\"humidity\",\"type\":\"uint16\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"timestamp\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"DataPublished\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"getCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"getRecord\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"sensorId\",\"type\":\"string\"},{\"internalType\":\"uint16\",\"name\":\"temperature\",\"type\":\"uint16\"},{\"internalType\":\"uint16\",\"name\":\"humidity\",\"type\":\"uint16\"},{\"internalType\":\"string\",\"name\":\"timestamp\",\"type\":\"string\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"sensorId\",\"type\":\"string\"},{\"internalType\":\"uint16\",\"name\":\"temperature\",\"type\":\"uint16\"},{\"internalType\":\"uint16\",\"name\":\"humidity\",\"type\":\"uint16\"},{\"internalType\":\"string\",\"name\":\"timestamp\",\"type\":\"string\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"publish\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}],\"devdoc\":{\"kind\":\"dev\",\"methods\":{},\"version\":1},\"userdoc\":{\"kind\":\"user\",\"methods\":{},\"version\":1}},\"settings\":{\"compilationTarget\":{\"project:/contracts/SensorData.sol\":\"SensorData\"},\"evmVersion\":\"paris\",\"libraries\":{},\"metadata\":{\"bytecodeHash\":\"ipfs\"},\"optimizer\":{\"enabled\":false,\"runs\":200},\"remappings\":[]},\"sources\":{\"project:/contracts/SensorData.sol\":{\"keccak256\":\"0x8ef0c0e7e2aed0e8ac53d1807d9bd7c7833891a4630a80d206c9f88beb5fc9c6\",\"license\":\"MIT\",\"urls\":[\"bzz-raw://38b98ee34387a43e4bad9f5ef1f61d9400cb2b3da5082e31cea33e5584feefae\",\"dweb:/ipfs/QmcqPV5ct5hLjFq7NtW47s5zrkZomDzpwp9QfceJ8b1i7A\"]}},\"version\":1}",
  "bytecode": "0x608060405234801561001057600080fd5b50610b68806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806303e9e6091461004657806364dc2c111461007a578063a87d942c14610096575b600080fd5b610060600480360381019061005b91906103fb565b6100b4565b6040516100719594939291906104ee565b60405180910390f35b610094600480360381019061008f91906106dc565b610287565b005b61009e6103a5565b6040516100ab919061079e565b60405180910390f35b60606000806060600080805490508610610103576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100fa90610805565b60405180910390fd5b600080878154811061011857610117610825565b5b90600052602060002090600402019050806000018160010160009054906101000a900461ffff168260010160029054906101000a900461ffff1683600201846003015484805461016790610883565b80601f016020809104026020016040519081016040528092919081815260200182805461019390610883565b80156101e05780601f106101b5576101008083540402835291602001916101e0565b820191906000526020600020905b8154815290600101906020018083116101c357829003601f168201915b505050505094508180546101f390610883565b80601f016020809104026020016040519081016040528092919081815260200182805461021f90610883565b801561026c5780601f106102415761010080835404028352916020019161026c565b820191906000526020600020905b81548152906001019060200180831161024f57829003601f168201915b50505050509150955095509550955095505091939590929450565b60006040518060a001604052808781526020018661ffff1681526020018561ffff16815260200184815260200183815250908060018154018082558091505060019003906000526020600020906004020160009091909190915060008201518160000190816102f69190610a60565b5060208201518160010160006101000a81548161ffff021916908361ffff16021790555060408201518160010160026101000a81548161ffff021916908361ffff16021790555060608201518160020190816103529190610a60565b506080820151816003015550507f6eb0565dcbe8aa0169af5bd5f4d41002bc8b3990be43b9649df2d48d330bc9c585858585856040516103969594939291906104ee565b60405180910390a15050505050565b60008080549050905090565b6000604051905090565b600080fd5b600080fd5b6000819050919050565b6103d8816103c5565b81146103e357600080fd5b50565b6000813590506103f5816103cf565b92915050565b600060208284031215610411576104106103bb565b5b600061041f848285016103e6565b91505092915050565b600081519050919050565b600082825260208201905092915050565b60005b83811015610462578082015181840152602081019050610447565b60008484015250505050565b6000601f19601f8301169050919050565b600061048a82610428565b6104948185610433565b93506104a4818560208601610444565b6104ad8161046e565b840191505092915050565b600061ffff82169050919050565b6104cf816104b8565b82525050565b6000819050919050565b6104e8816104d5565b82525050565b600060a0820190508181036000830152610508818861047f565b905061051760208301876104c6565b61052460408301866104c6565b8181036060830152610536818561047f565b905061054560808301846104df565b9695505050505050565b600080fd5b600080fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6105918261046e565b810181811067ffffffffffffffff821117156105b0576105af610559565b5b80604052505050565b60006105c36103b1565b90506105cf8282610588565b919050565b600067ffffffffffffffff8211156105ef576105ee610559565b5b6105f88261046e565b9050602081019050919050565b82818337600083830152505050565b6000610627610622846105d4565b6105b9565b90508281526020810184848401111561064357610642610554565b5b61064e848285610605565b509392505050565b600082601f83011261066b5761066a61054f565b5b813561067b848260208601610614565b91505092915050565b61068d816104b8565b811461069857600080fd5b50565b6000813590506106aa81610684565b92915050565b6106b9816104d5565b81146106c457600080fd5b50565b6000813590506106d6816106b0565b92915050565b600080600080600060a086880312156106f8576106f76103bb565b5b600086013567ffffffffffffffff811115610716576107156103c0565b5b61072288828901610656565b95505060206107338882890161069b565b94505060406107448882890161069b565b935050606086013567ffffffffffffffff811115610765576107646103c0565b5b61077188828901610656565b9250506080610782888289016106c7565b9150509295509295909350565b610798816103c5565b82525050565b60006020820190506107b3600083018461078f565b92915050565b7f496e76616c696420696e64657800000000000000000000000000000000000000600082015250565b60006107ef600d83610433565b91506107fa826107b9565b602082019050919050565b6000602082019050818103600083015261081e816107e2565b9050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b6000600282049050600182168061089b57607f821691505b6020821081036108ae576108ad610854565b5b50919050565b60008190508160005260206000209050919050565b60006020601f8301049050919050565b600082821b905092915050565b6000600883026109167fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826108d9565b61092086836108d9565b95508019841693508086168417925050509392505050565b6000819050919050565b600061095d610958610953846103c5565b610938565b6103c5565b9050919050565b6000819050919050565b61097783610942565b61098b61098382610964565b8484546108e6565b825550505050565b600090565b6109a0610993565b6109ab81848461096e565b505050565b5b818110156109cf576109c4600082610998565b6001810190506109b1565b5050565b601f821115610a14576109e5816108b4565b6109ee846108c9565b810160208510156109fd578190505b610a11610a09856108c9565b8301826109b0565b50505b505050565b600082821c905092915050565b6000610a3760001984600802610a19565b1980831691505092915050565b6000610a508383610a26565b9150826002028217905092915050565b610a6982610428565b67ffffffffffffffff811115610a8257610a81610559565b5b610a8c8254610883565b610a978282856109d3565b600060209050601f831160018114610aca5760008415610ab8578287015190505b610ac28582610a44565b865550610b2a565b601f198416610ad8866108b4565b60005b82811015610b0057848901518255600182019150602085019450602081019050610adb565b86831015610b1d5784890151610b19601f891682610a26565b8355505b6001600288020188555050505b50505050505056fea26469706673582212206892cc5a6319a5eefb77c82ab039fa9a3b3e487ae4f5b2c45f8fec2f599866cb64736f6c63430008130033",
  "deployedBytecode": "0x608060405234801561001057600080fd5b50600436106100415760003560e01c806303e9e6091461004657806364dc2c111461007a578063a87d942c14610096575b600080fd5b610060600480360381019061005b91906103fb565b6100b4565b6040516100719594939291906104ee565b60405180910390f35b610094600480360381019061008f91906106dc565b610287565b005b61009e6103a5565b6040516100ab919061079e565b60405180910390f35b60606000806060600080805490508610610103576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100fa90610805565b60405180910390fd5b600080878154811061011857610117610825565b5b90600052602060002090600402019050806000018160010160009054906101000a900461ffff168260010160029054906101000a900461ffff1683600201846003015484805461016790610883565b80601f016020809104026020016040519081016040528092919081815260200182805461019390610883565b80156101e05780601f106101b5576101008083540402835291602001916101e0565b820191906000526020600020905b8154815290600101906020018083116101c357829003601f168201915b505050505094508180546101f390610883565b80601f016020809104026020016040519081016040528092919081815260200182805461021f90610883565b801561026c5780601f106102415761010080835404028352916020019161026c565b820191906000526020600020905b81548152906001019060200180831161024f57829003601f168201915b50505050509150955095509550955095505091939590929450565b60006040518060a001604052808781526020018661ffff1681526020018561ffff16815260200184815260200183815250908060018154018082558091505060019003906000526020600020906004020160009091909190915060008201518160000190816102f69190610a60565b5060208201518160010160006101000a81548161ffff021916908361ffff16021790555060408201518160010160026101000a81548161ffff021916908361ffff16021790555060608201518160020190816103529190610a60565b506080820151816003015550507f6eb0565dcbe8aa0169af5bd5f4d41002bc8b3990be43b9649df2d48d330bc9c585858585856040516103969594939291906104ee565b60405180910390a15050505050565b60008080549050905090565b6000604051905090565b600080fd5b600080fd5b6000819050919050565b6103d8816103c5565b81146103e357600080fd5b50565b6000813590506103f5816103cf565b92915050565b600060208284031215610411576104106103bb565b5b600061041f848285016103e6565b91505092915050565b600081519050919050565b600082825260208201905092915050565b60005b83811015610462578082015181840152602081019050610447565b60008484015250505050565b6000601f19601f8301169050919050565b600061048a82610428565b6104948185610433565b93506104a4818560208601610444565b6104ad8161046e565b840191505092915050565b600061ffff82169050919050565b6104cf816104b8565b82525050565b6000819050919050565b6104e8816104d5565b82525050565b600060a0820190508181036000830152610508818861047f565b905061051760208301876104c6565b61052460408301866104c6565b8181036060830152610536818561047f565b905061054560808301846104df565b9695505050505050565b600080fd5b600080fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6105918261046e565b810181811067ffffffffffffffff821117156105b0576105af610559565b5b80604052505050565b60006105c36103b1565b90506105cf8282610588565b919050565b600067ffffffffffffffff8211156105ef576105ee610559565b5b6105f88261046e565b9050602081019050919050565b82818337600083830152505050565b6000610627610622846105d4565b6105b9565b90508281526020810184848401111561064357610642610554565b5b61064e848285610605565b509392505050565b600082601f83011261066b5761066a61054f565b5b813561067b848260208601610614565b91505092915050565b61068d816104b8565b811461069857600080fd5b50565b6000813590506106aa81610684565b92915050565b6106b9816104d5565b81146106c457600080fd5b50565b6000813590506106d6816106b0565b92915050565b600080600080600060a086880312156106f8576106f76103bb565b5b600086013567ffffffffffffffff811115610716576107156103c0565b5b61072288828901610656565b95505060206107338882890161069b565b94505060406107448882890161069b565b935050606086013567ffffffffffffffff811115610765576107646103c0565b5b61077188828901610656565b9250506080610782888289016106c7565b9150509295509295909350565b610798816103c5565b82525050565b60006020820190506107b3600083018461078f565b92915050565b7f496e76616c696420696e64657800000000000000000000000000000000000000600082015250565b60006107ef600d83610433565b91506107fa826107b9565b602082019050919050565b6000602082019050818103600083015261081e816107e2565b9050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b6000600282049050600182168061089b57607f821691505b6020821081036108ae576108ad610854565b5b50919050565b60008190508160005260206000209050919050565b60006020601f8301049050919050565b600082821b905092915050565b6000600883026109167fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826108d9565b61092086836108d9565b95508019841693508086168417925050509392505050565b6000819050919050565b600061095d610958610953846103c5565b610938565b6103c5565b9050919050565b6000819050919050565b61097783610942565b61098b61098382610964565b8484546108e6565b825550505050565b600090565b6109a0610993565b6109ab81848461096e565b505050565b5b818110156109cf576109c4600082610998565b6001810190506109b1565b5050565b601f821115610a14576109e5816108b4565b6109ee846108c9565b810160208510156109fd578190505b610a11610a09856108c9565b8301826109b0565b50505b505050565b600082821c905092915050565b6000610a3760001984600802610a19565b1980831691505092915050565b6000610a508383610a26565b9150826002028217905092915050565b610a6982610428565b67ffffffffffffffff811115610a8257610a81610559565b5b610a8c8254610883565b610a978282856109d3565b600060209050601f831160018114610aca5760008415610ab8578287015190505b610ac28582610a44565b865550610b2a565b601f198416610ad8866108b4565b60005b82811015610b0057848901518255600182019150602085019450602081019050610adb565b86831015610b1d5784890151610b19601f891682610a26565b8355505b6001600288020188555050505b50505050505056fea26469706673582212206892cc5a6319a5eefb77c82ab039fa9a3b3e487ae4f5b2c45f8fec2f599866cb64736f6c63430008130033",
  "immutableReferences": {},
  "generatedSources": [],
  "deployedGeneratedSources": [
    {
      "ast": {
        "nodeType": "YulBlock",
        "src": "0:13595:1",
        "statements": [
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "47:35:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "57:19:1",
                  "value": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "73:2:1",
                        "type": "",
                        "value": "64"
                      }
                    ],
                    "functionName": {
                      "name": "mload",
                      "nodeType": "YulIdentifier",
                      "src": "67:5:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "67:9:1"
                  },
                  "variableNames": [
                    {
                      "name": "memPtr",
                      "nodeType": "YulIdentifier",
                      "src": "57:6:1"
                    }
                  ]
                }
              ]
            },
            "name": "allocate_unbounded",
            "nodeType": "YulFunctionDefinition",
            "returnVariables": [
              {
                "name": "memPtr",
                "nodeType": "YulTypedName",
                "src": "40:6:1",
                "type": ""
              }
            ],
            "src": "7:75:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "177:28:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "194:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "197:1:1",
                        "type": "",
                        "value": "0"
                      }
                    ],
                    "functionName": {
                      "name": "revert",
                      "nodeType": "YulIdentifier",
                      "src": "187:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "187:12:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "187:12:1"
                }
              ]
            },
            "name": "revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b",
            "nodeType": "YulFunctionDefinition",
            "src": "88:117:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "300:28:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "317:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "320:1:1",
                        "type": "",
                        "value": "0"
                      }
                    ],
                    "functionName": {
                      "name": "revert",
                      "nodeType": "YulIdentifier",
                      "src": "310:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "310:12:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "310:12:1"
                }
              ]
            },
            "name": "revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db",
            "nodeType": "YulFunctionDefinition",
            "src": "211:117:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "379:32:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "389:16:1",
                  "value": {
                    "name": "value",
                    "nodeType": "YulIdentifier",
                    "src": "400:5:1"
                  },
                  "variableNames": [
                    {
                      "name": "cleaned",
                      "nodeType": "YulIdentifier",
                      "src": "389:7:1"
                    }
                  ]
                }
              ]
            },
            "name": "cleanup_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "361:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "cleaned",
                "nodeType": "YulTypedName",
                "src": "371:7:1",
                "type": ""
              }
            ],
            "src": "334:77:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "460:79:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "517:16:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "526:1:1",
                              "type": "",
                              "value": "0"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "529:1:1",
                              "type": "",
                              "value": "0"
                            }
                          ],
                          "functionName": {
                            "name": "revert",
                            "nodeType": "YulIdentifier",
                            "src": "519:6:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "519:12:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "519:12:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "483:5:1"
                          },
                          {
                            "arguments": [
                              {
                                "name": "value",
                                "nodeType": "YulIdentifier",
                                "src": "508:5:1"
                              }
                            ],
                            "functionName": {
                              "name": "cleanup_t_uint256",
                              "nodeType": "YulIdentifier",
                              "src": "490:17:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "490:24:1"
                          }
                        ],
                        "functionName": {
                          "name": "eq",
                          "nodeType": "YulIdentifier",
                          "src": "480:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "480:35:1"
                      }
                    ],
                    "functionName": {
                      "name": "iszero",
                      "nodeType": "YulIdentifier",
                      "src": "473:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "473:43:1"
                  },
                  "nodeType": "YulIf",
                  "src": "470:63:1"
                }
              ]
            },
            "name": "validator_revert_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "453:5:1",
                "type": ""
              }
            ],
            "src": "417:122:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "597:87:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "607:29:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "offset",
                        "nodeType": "YulIdentifier",
                        "src": "629:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "calldataload",
                      "nodeType": "YulIdentifier",
                      "src": "616:12:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "616:20:1"
                  },
                  "variableNames": [
                    {
                      "name": "value",
                      "nodeType": "YulIdentifier",
                      "src": "607:5:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "672:5:1"
                      }
                    ],
                    "functionName": {
                      "name": "validator_revert_t_uint256",
                      "nodeType": "YulIdentifier",
                      "src": "645:26:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "645:33:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "645:33:1"
                }
              ]
            },
            "name": "abi_decode_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "offset",
                "nodeType": "YulTypedName",
                "src": "575:6:1",
                "type": ""
              },
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "583:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "591:5:1",
                "type": ""
              }
            ],
            "src": "545:139:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "756:263:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "802:83:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b",
                            "nodeType": "YulIdentifier",
                            "src": "804:77:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "804:79:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "804:79:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "777:7:1"
                          },
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "786:9:1"
                          }
                        ],
                        "functionName": {
                          "name": "sub",
                          "nodeType": "YulIdentifier",
                          "src": "773:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "773:23:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "798:2:1",
                        "type": "",
                        "value": "32"
                      }
                    ],
                    "functionName": {
                      "name": "slt",
                      "nodeType": "YulIdentifier",
                      "src": "769:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "769:32:1"
                  },
                  "nodeType": "YulIf",
                  "src": "766:119:1"
                },
                {
                  "nodeType": "YulBlock",
                  "src": "895:117:1",
                  "statements": [
                    {
                      "nodeType": "YulVariableDeclaration",
                      "src": "910:15:1",
                      "value": {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "924:1:1",
                        "type": "",
                        "value": "0"
                      },
                      "variables": [
                        {
                          "name": "offset",
                          "nodeType": "YulTypedName",
                          "src": "914:6:1",
                          "type": ""
                        }
                      ]
                    },
                    {
                      "nodeType": "YulAssignment",
                      "src": "939:63:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "974:9:1"
                              },
                              {
                                "name": "offset",
                                "nodeType": "YulIdentifier",
                                "src": "985:6:1"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "970:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "970:22:1"
                          },
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "994:7:1"
                          }
                        ],
                        "functionName": {
                          "name": "abi_decode_t_uint256",
                          "nodeType": "YulIdentifier",
                          "src": "949:20:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "949:53:1"
                      },
                      "variableNames": [
                        {
                          "name": "value0",
                          "nodeType": "YulIdentifier",
                          "src": "939:6:1"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            "name": "abi_decode_tuple_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "headStart",
                "nodeType": "YulTypedName",
                "src": "726:9:1",
                "type": ""
              },
              {
                "name": "dataEnd",
                "nodeType": "YulTypedName",
                "src": "737:7:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "value0",
                "nodeType": "YulTypedName",
                "src": "749:6:1",
                "type": ""
              }
            ],
            "src": "690:329:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "1084:40:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "1095:22:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "1111:5:1"
                      }
                    ],
                    "functionName": {
                      "name": "mload",
                      "nodeType": "YulIdentifier",
                      "src": "1105:5:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1105:12:1"
                  },
                  "variableNames": [
                    {
                      "name": "length",
                      "nodeType": "YulIdentifier",
                      "src": "1095:6:1"
                    }
                  ]
                }
              ]
            },
            "name": "array_length_t_string_memory_ptr",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "1067:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "length",
                "nodeType": "YulTypedName",
                "src": "1077:6:1",
                "type": ""
              }
            ],
            "src": "1025:99:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "1226:73:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "1243:3:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "1248:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "1236:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1236:19:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "1236:19:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "1264:29:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "1283:3:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "1288:4:1",
                        "type": "",
                        "value": "0x20"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "1279:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1279:14:1"
                  },
                  "variableNames": [
                    {
                      "name": "updated_pos",
                      "nodeType": "YulIdentifier",
                      "src": "1264:11:1"
                    }
                  ]
                }
              ]
            },
            "name": "array_storeLengthForEncoding_t_string_memory_ptr_fromStack",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "pos",
                "nodeType": "YulTypedName",
                "src": "1198:3:1",
                "type": ""
              },
              {
                "name": "length",
                "nodeType": "YulTypedName",
                "src": "1203:6:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "updated_pos",
                "nodeType": "YulTypedName",
                "src": "1214:11:1",
                "type": ""
              }
            ],
            "src": "1130:169:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "1367:184:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "1377:10:1",
                  "value": {
                    "kind": "number",
                    "nodeType": "YulLiteral",
                    "src": "1386:1:1",
                    "type": "",
                    "value": "0"
                  },
                  "variables": [
                    {
                      "name": "i",
                      "nodeType": "YulTypedName",
                      "src": "1381:1:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "1446:63:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [
                            {
                              "arguments": [
                                {
                                  "name": "dst",
                                  "nodeType": "YulIdentifier",
                                  "src": "1471:3:1"
                                },
                                {
                                  "name": "i",
                                  "nodeType": "YulIdentifier",
                                  "src": "1476:1:1"
                                }
                              ],
                              "functionName": {
                                "name": "add",
                                "nodeType": "YulIdentifier",
                                "src": "1467:3:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "1467:11:1"
                            },
                            {
                              "arguments": [
                                {
                                  "arguments": [
                                    {
                                      "name": "src",
                                      "nodeType": "YulIdentifier",
                                      "src": "1490:3:1"
                                    },
                                    {
                                      "name": "i",
                                      "nodeType": "YulIdentifier",
                                      "src": "1495:1:1"
                                    }
                                  ],
                                  "functionName": {
                                    "name": "add",
                                    "nodeType": "YulIdentifier",
                                    "src": "1486:3:1"
                                  },
                                  "nodeType": "YulFunctionCall",
                                  "src": "1486:11:1"
                                }
                              ],
                              "functionName": {
                                "name": "mload",
                                "nodeType": "YulIdentifier",
                                "src": "1480:5:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "1480:18:1"
                            }
                          ],
                          "functionName": {
                            "name": "mstore",
                            "nodeType": "YulIdentifier",
                            "src": "1460:6:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "1460:39:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "1460:39:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "name": "i",
                        "nodeType": "YulIdentifier",
                        "src": "1407:1:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "1410:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "lt",
                      "nodeType": "YulIdentifier",
                      "src": "1404:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1404:13:1"
                  },
                  "nodeType": "YulForLoop",
                  "post": {
                    "nodeType": "YulBlock",
                    "src": "1418:19:1",
                    "statements": [
                      {
                        "nodeType": "YulAssignment",
                        "src": "1420:15:1",
                        "value": {
                          "arguments": [
                            {
                              "name": "i",
                              "nodeType": "YulIdentifier",
                              "src": "1429:1:1"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "1432:2:1",
                              "type": "",
                              "value": "32"
                            }
                          ],
                          "functionName": {
                            "name": "add",
                            "nodeType": "YulIdentifier",
                            "src": "1425:3:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "1425:10:1"
                        },
                        "variableNames": [
                          {
                            "name": "i",
                            "nodeType": "YulIdentifier",
                            "src": "1420:1:1"
                          }
                        ]
                      }
                    ]
                  },
                  "pre": {
                    "nodeType": "YulBlock",
                    "src": "1400:3:1",
                    "statements": []
                  },
                  "src": "1396:113:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "dst",
                            "nodeType": "YulIdentifier",
                            "src": "1529:3:1"
                          },
                          {
                            "name": "length",
                            "nodeType": "YulIdentifier",
                            "src": "1534:6:1"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "1525:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "1525:16:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "1543:1:1",
                        "type": "",
                        "value": "0"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "1518:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1518:27:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "1518:27:1"
                }
              ]
            },
            "name": "copy_memory_to_memory_with_cleanup",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "src",
                "nodeType": "YulTypedName",
                "src": "1349:3:1",
                "type": ""
              },
              {
                "name": "dst",
                "nodeType": "YulTypedName",
                "src": "1354:3:1",
                "type": ""
              },
              {
                "name": "length",
                "nodeType": "YulTypedName",
                "src": "1359:6:1",
                "type": ""
              }
            ],
            "src": "1305:246:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "1605:54:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "1615:38:1",
                  "value": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "1633:5:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "1640:2:1",
                            "type": "",
                            "value": "31"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "1629:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "1629:14:1"
                      },
                      {
                        "arguments": [
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "1649:2:1",
                            "type": "",
                            "value": "31"
                          }
                        ],
                        "functionName": {
                          "name": "not",
                          "nodeType": "YulIdentifier",
                          "src": "1645:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "1645:7:1"
                      }
                    ],
                    "functionName": {
                      "name": "and",
                      "nodeType": "YulIdentifier",
                      "src": "1625:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1625:28:1"
                  },
                  "variableNames": [
                    {
                      "name": "result",
                      "nodeType": "YulIdentifier",
                      "src": "1615:6:1"
                    }
                  ]
                }
              ]
            },
            "name": "round_up_to_mul_of_32",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "1588:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "result",
                "nodeType": "YulTypedName",
                "src": "1598:6:1",
                "type": ""
              }
            ],
            "src": "1557:102:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "1757:285:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "1767:53:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "1814:5:1"
                      }
                    ],
                    "functionName": {
                      "name": "array_length_t_string_memory_ptr",
                      "nodeType": "YulIdentifier",
                      "src": "1781:32:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1781:39:1"
                  },
                  "variables": [
                    {
                      "name": "length",
                      "nodeType": "YulTypedName",
                      "src": "1771:6:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "1829:78:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "1895:3:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "1900:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "array_storeLengthForEncoding_t_string_memory_ptr_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "1836:58:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1836:71:1"
                  },
                  "variableNames": [
                    {
                      "name": "pos",
                      "nodeType": "YulIdentifier",
                      "src": "1829:3:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "1955:5:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "1962:4:1",
                            "type": "",
                            "value": "0x20"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "1951:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "1951:16:1"
                      },
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "1969:3:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "1974:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "copy_memory_to_memory_with_cleanup",
                      "nodeType": "YulIdentifier",
                      "src": "1916:34:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1916:65:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "1916:65:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "1990:46:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "2001:3:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "length",
                            "nodeType": "YulIdentifier",
                            "src": "2028:6:1"
                          }
                        ],
                        "functionName": {
                          "name": "round_up_to_mul_of_32",
                          "nodeType": "YulIdentifier",
                          "src": "2006:21:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "2006:29:1"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "1997:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "1997:39:1"
                  },
                  "variableNames": [
                    {
                      "name": "end",
                      "nodeType": "YulIdentifier",
                      "src": "1990:3:1"
                    }
                  ]
                }
              ]
            },
            "name": "abi_encode_t_string_memory_ptr_to_t_string_memory_ptr_fromStack",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "1738:5:1",
                "type": ""
              },
              {
                "name": "pos",
                "nodeType": "YulTypedName",
                "src": "1745:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "1753:3:1",
                "type": ""
              }
            ],
            "src": "1665:377:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "2092:45:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "2102:29:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "2117:5:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "2124:6:1",
                        "type": "",
                        "value": "0xffff"
                      }
                    ],
                    "functionName": {
                      "name": "and",
                      "nodeType": "YulIdentifier",
                      "src": "2113:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2113:18:1"
                  },
                  "variableNames": [
                    {
                      "name": "cleaned",
                      "nodeType": "YulIdentifier",
                      "src": "2102:7:1"
                    }
                  ]
                }
              ]
            },
            "name": "cleanup_t_uint16",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "2074:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "cleaned",
                "nodeType": "YulTypedName",
                "src": "2084:7:1",
                "type": ""
              }
            ],
            "src": "2048:89:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "2206:52:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "2223:3:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "2245:5:1"
                          }
                        ],
                        "functionName": {
                          "name": "cleanup_t_uint16",
                          "nodeType": "YulIdentifier",
                          "src": "2228:16:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "2228:23:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "2216:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2216:36:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "2216:36:1"
                }
              ]
            },
            "name": "abi_encode_t_uint16_to_t_uint16_fromStack",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "2194:5:1",
                "type": ""
              },
              {
                "name": "pos",
                "nodeType": "YulTypedName",
                "src": "2201:3:1",
                "type": ""
              }
            ],
            "src": "2143:115:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "2309:32:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "2319:16:1",
                  "value": {
                    "name": "value",
                    "nodeType": "YulIdentifier",
                    "src": "2330:5:1"
                  },
                  "variableNames": [
                    {
                      "name": "cleaned",
                      "nodeType": "YulIdentifier",
                      "src": "2319:7:1"
                    }
                  ]
                }
              ]
            },
            "name": "cleanup_t_bytes32",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "2291:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "cleaned",
                "nodeType": "YulTypedName",
                "src": "2301:7:1",
                "type": ""
              }
            ],
            "src": "2264:77:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "2412:53:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "2429:3:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "2452:5:1"
                          }
                        ],
                        "functionName": {
                          "name": "cleanup_t_bytes32",
                          "nodeType": "YulIdentifier",
                          "src": "2434:17:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "2434:24:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "2422:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2422:37:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "2422:37:1"
                }
              ]
            },
            "name": "abi_encode_t_bytes32_to_t_bytes32_fromStack",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "2400:5:1",
                "type": ""
              },
              {
                "name": "pos",
                "nodeType": "YulTypedName",
                "src": "2407:3:1",
                "type": ""
              }
            ],
            "src": "2347:118:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "2717:592:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "2727:27:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "headStart",
                        "nodeType": "YulIdentifier",
                        "src": "2739:9:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "2750:3:1",
                        "type": "",
                        "value": "160"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "2735:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2735:19:1"
                  },
                  "variableNames": [
                    {
                      "name": "tail",
                      "nodeType": "YulIdentifier",
                      "src": "2727:4:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "2775:9:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "2786:1:1",
                            "type": "",
                            "value": "0"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "2771:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "2771:17:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "tail",
                            "nodeType": "YulIdentifier",
                            "src": "2794:4:1"
                          },
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "2800:9:1"
                          }
                        ],
                        "functionName": {
                          "name": "sub",
                          "nodeType": "YulIdentifier",
                          "src": "2790:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "2790:20:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "2764:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2764:47:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "2764:47:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "2820:86:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value0",
                        "nodeType": "YulIdentifier",
                        "src": "2892:6:1"
                      },
                      {
                        "name": "tail",
                        "nodeType": "YulIdentifier",
                        "src": "2901:4:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_encode_t_string_memory_ptr_to_t_string_memory_ptr_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "2828:63:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2828:78:1"
                  },
                  "variableNames": [
                    {
                      "name": "tail",
                      "nodeType": "YulIdentifier",
                      "src": "2820:4:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "value1",
                        "nodeType": "YulIdentifier",
                        "src": "2958:6:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "2971:9:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "2982:2:1",
                            "type": "",
                            "value": "32"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "2967:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "2967:18:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_encode_t_uint16_to_t_uint16_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "2916:41:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2916:70:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "2916:70:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "value2",
                        "nodeType": "YulIdentifier",
                        "src": "3038:6:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "3051:9:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "3062:2:1",
                            "type": "",
                            "value": "64"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "3047:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "3047:18:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_encode_t_uint16_to_t_uint16_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "2996:41:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "2996:70:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "2996:70:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "3087:9:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "3098:2:1",
                            "type": "",
                            "value": "96"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "3083:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "3083:18:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "tail",
                            "nodeType": "YulIdentifier",
                            "src": "3107:4:1"
                          },
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "3113:9:1"
                          }
                        ],
                        "functionName": {
                          "name": "sub",
                          "nodeType": "YulIdentifier",
                          "src": "3103:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "3103:20:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "3076:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3076:48:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "3076:48:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "3133:86:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value3",
                        "nodeType": "YulIdentifier",
                        "src": "3205:6:1"
                      },
                      {
                        "name": "tail",
                        "nodeType": "YulIdentifier",
                        "src": "3214:4:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_encode_t_string_memory_ptr_to_t_string_memory_ptr_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "3141:63:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3141:78:1"
                  },
                  "variableNames": [
                    {
                      "name": "tail",
                      "nodeType": "YulIdentifier",
                      "src": "3133:4:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "value4",
                        "nodeType": "YulIdentifier",
                        "src": "3273:6:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "3286:9:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "3297:3:1",
                            "type": "",
                            "value": "128"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "3282:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "3282:19:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_encode_t_bytes32_to_t_bytes32_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "3229:43:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3229:73:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "3229:73:1"
                }
              ]
            },
            "name": "abi_encode_tuple_t_string_memory_ptr_t_uint16_t_uint16_t_string_memory_ptr_t_bytes32__to_t_string_memory_ptr_t_uint16_t_uint16_t_string_memory_ptr_t_bytes32__fromStack_reversed",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "headStart",
                "nodeType": "YulTypedName",
                "src": "2657:9:1",
                "type": ""
              },
              {
                "name": "value4",
                "nodeType": "YulTypedName",
                "src": "2669:6:1",
                "type": ""
              },
              {
                "name": "value3",
                "nodeType": "YulTypedName",
                "src": "2677:6:1",
                "type": ""
              },
              {
                "name": "value2",
                "nodeType": "YulTypedName",
                "src": "2685:6:1",
                "type": ""
              },
              {
                "name": "value1",
                "nodeType": "YulTypedName",
                "src": "2693:6:1",
                "type": ""
              },
              {
                "name": "value0",
                "nodeType": "YulTypedName",
                "src": "2701:6:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "tail",
                "nodeType": "YulTypedName",
                "src": "2712:4:1",
                "type": ""
              }
            ],
            "src": "2471:838:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "3404:28:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3421:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3424:1:1",
                        "type": "",
                        "value": "0"
                      }
                    ],
                    "functionName": {
                      "name": "revert",
                      "nodeType": "YulIdentifier",
                      "src": "3414:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3414:12:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "3414:12:1"
                }
              ]
            },
            "name": "revert_error_1b9f4a0a5773e33b91aa01db23bf8c55fce1411167c872835e7fa00a4f17d46d",
            "nodeType": "YulFunctionDefinition",
            "src": "3315:117:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "3527:28:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3544:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3547:1:1",
                        "type": "",
                        "value": "0"
                      }
                    ],
                    "functionName": {
                      "name": "revert",
                      "nodeType": "YulIdentifier",
                      "src": "3537:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3537:12:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "3537:12:1"
                }
              ]
            },
            "name": "revert_error_987264b3b1d58a9c7f8255e93e81c77d86d6299019c33110a076957a3e06e2ae",
            "nodeType": "YulFunctionDefinition",
            "src": "3438:117:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "3589:152:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3606:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3609:77:1",
                        "type": "",
                        "value": "35408467139433450592217433187231851964531694900788300625387963629091585785856"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "3599:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3599:88:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "3599:88:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3703:1:1",
                        "type": "",
                        "value": "4"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3706:4:1",
                        "type": "",
                        "value": "0x41"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "3696:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3696:15:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "3696:15:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3727:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "3730:4:1",
                        "type": "",
                        "value": "0x24"
                      }
                    ],
                    "functionName": {
                      "name": "revert",
                      "nodeType": "YulIdentifier",
                      "src": "3720:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3720:15:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "3720:15:1"
                }
              ]
            },
            "name": "panic_error_0x41",
            "nodeType": "YulFunctionDefinition",
            "src": "3561:180:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "3790:238:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "3800:58:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "memPtr",
                        "nodeType": "YulIdentifier",
                        "src": "3822:6:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "size",
                            "nodeType": "YulIdentifier",
                            "src": "3852:4:1"
                          }
                        ],
                        "functionName": {
                          "name": "round_up_to_mul_of_32",
                          "nodeType": "YulIdentifier",
                          "src": "3830:21:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "3830:27:1"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "3818:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3818:40:1"
                  },
                  "variables": [
                    {
                      "name": "newFreePtr",
                      "nodeType": "YulTypedName",
                      "src": "3804:10:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "3969:22:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "panic_error_0x41",
                            "nodeType": "YulIdentifier",
                            "src": "3971:16:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "3971:18:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "3971:18:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "newFreePtr",
                            "nodeType": "YulIdentifier",
                            "src": "3912:10:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "3924:18:1",
                            "type": "",
                            "value": "0xffffffffffffffff"
                          }
                        ],
                        "functionName": {
                          "name": "gt",
                          "nodeType": "YulIdentifier",
                          "src": "3909:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "3909:34:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "newFreePtr",
                            "nodeType": "YulIdentifier",
                            "src": "3948:10:1"
                          },
                          {
                            "name": "memPtr",
                            "nodeType": "YulIdentifier",
                            "src": "3960:6:1"
                          }
                        ],
                        "functionName": {
                          "name": "lt",
                          "nodeType": "YulIdentifier",
                          "src": "3945:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "3945:22:1"
                      }
                    ],
                    "functionName": {
                      "name": "or",
                      "nodeType": "YulIdentifier",
                      "src": "3906:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "3906:62:1"
                  },
                  "nodeType": "YulIf",
                  "src": "3903:88:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "4007:2:1",
                        "type": "",
                        "value": "64"
                      },
                      {
                        "name": "newFreePtr",
                        "nodeType": "YulIdentifier",
                        "src": "4011:10:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "4000:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4000:22:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "4000:22:1"
                }
              ]
            },
            "name": "finalize_allocation",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "memPtr",
                "nodeType": "YulTypedName",
                "src": "3776:6:1",
                "type": ""
              },
              {
                "name": "size",
                "nodeType": "YulTypedName",
                "src": "3784:4:1",
                "type": ""
              }
            ],
            "src": "3747:281:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "4075:88:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "4085:30:1",
                  "value": {
                    "arguments": [],
                    "functionName": {
                      "name": "allocate_unbounded",
                      "nodeType": "YulIdentifier",
                      "src": "4095:18:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4095:20:1"
                  },
                  "variableNames": [
                    {
                      "name": "memPtr",
                      "nodeType": "YulIdentifier",
                      "src": "4085:6:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "memPtr",
                        "nodeType": "YulIdentifier",
                        "src": "4144:6:1"
                      },
                      {
                        "name": "size",
                        "nodeType": "YulIdentifier",
                        "src": "4152:4:1"
                      }
                    ],
                    "functionName": {
                      "name": "finalize_allocation",
                      "nodeType": "YulIdentifier",
                      "src": "4124:19:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4124:33:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "4124:33:1"
                }
              ]
            },
            "name": "allocate_memory",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "size",
                "nodeType": "YulTypedName",
                "src": "4059:4:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "memPtr",
                "nodeType": "YulTypedName",
                "src": "4068:6:1",
                "type": ""
              }
            ],
            "src": "4034:129:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "4236:241:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "4341:22:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "panic_error_0x41",
                            "nodeType": "YulIdentifier",
                            "src": "4343:16:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "4343:18:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "4343:18:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "4313:6:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "4321:18:1",
                        "type": "",
                        "value": "0xffffffffffffffff"
                      }
                    ],
                    "functionName": {
                      "name": "gt",
                      "nodeType": "YulIdentifier",
                      "src": "4310:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4310:30:1"
                  },
                  "nodeType": "YulIf",
                  "src": "4307:56:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "4373:37:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "4403:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "round_up_to_mul_of_32",
                      "nodeType": "YulIdentifier",
                      "src": "4381:21:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4381:29:1"
                  },
                  "variableNames": [
                    {
                      "name": "size",
                      "nodeType": "YulIdentifier",
                      "src": "4373:4:1"
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "4447:23:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "size",
                        "nodeType": "YulIdentifier",
                        "src": "4459:4:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "4465:4:1",
                        "type": "",
                        "value": "0x20"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "4455:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4455:15:1"
                  },
                  "variableNames": [
                    {
                      "name": "size",
                      "nodeType": "YulIdentifier",
                      "src": "4447:4:1"
                    }
                  ]
                }
              ]
            },
            "name": "array_allocation_size_t_string_memory_ptr",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "length",
                "nodeType": "YulTypedName",
                "src": "4220:6:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "size",
                "nodeType": "YulTypedName",
                "src": "4231:4:1",
                "type": ""
              }
            ],
            "src": "4169:308:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "4547:82:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "dst",
                        "nodeType": "YulIdentifier",
                        "src": "4570:3:1"
                      },
                      {
                        "name": "src",
                        "nodeType": "YulIdentifier",
                        "src": "4575:3:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "4580:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "calldatacopy",
                      "nodeType": "YulIdentifier",
                      "src": "4557:12:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4557:30:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "4557:30:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "dst",
                            "nodeType": "YulIdentifier",
                            "src": "4607:3:1"
                          },
                          {
                            "name": "length",
                            "nodeType": "YulIdentifier",
                            "src": "4612:6:1"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "4603:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "4603:16:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "4621:1:1",
                        "type": "",
                        "value": "0"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "4596:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4596:27:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "4596:27:1"
                }
              ]
            },
            "name": "copy_calldata_to_memory_with_cleanup",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "src",
                "nodeType": "YulTypedName",
                "src": "4529:3:1",
                "type": ""
              },
              {
                "name": "dst",
                "nodeType": "YulTypedName",
                "src": "4534:3:1",
                "type": ""
              },
              {
                "name": "length",
                "nodeType": "YulTypedName",
                "src": "4539:6:1",
                "type": ""
              }
            ],
            "src": "4483:146:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "4719:341:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "4729:75:1",
                  "value": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "length",
                            "nodeType": "YulIdentifier",
                            "src": "4796:6:1"
                          }
                        ],
                        "functionName": {
                          "name": "array_allocation_size_t_string_memory_ptr",
                          "nodeType": "YulIdentifier",
                          "src": "4754:41:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "4754:49:1"
                      }
                    ],
                    "functionName": {
                      "name": "allocate_memory",
                      "nodeType": "YulIdentifier",
                      "src": "4738:15:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4738:66:1"
                  },
                  "variableNames": [
                    {
                      "name": "array",
                      "nodeType": "YulIdentifier",
                      "src": "4729:5:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "array",
                        "nodeType": "YulIdentifier",
                        "src": "4820:5:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "4827:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "4813:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4813:21:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "4813:21:1"
                },
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "4843:27:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "array",
                        "nodeType": "YulIdentifier",
                        "src": "4858:5:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "4865:4:1",
                        "type": "",
                        "value": "0x20"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "4854:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4854:16:1"
                  },
                  "variables": [
                    {
                      "name": "dst",
                      "nodeType": "YulTypedName",
                      "src": "4847:3:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "4908:83:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "revert_error_987264b3b1d58a9c7f8255e93e81c77d86d6299019c33110a076957a3e06e2ae",
                            "nodeType": "YulIdentifier",
                            "src": "4910:77:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "4910:79:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "4910:79:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "src",
                            "nodeType": "YulIdentifier",
                            "src": "4889:3:1"
                          },
                          {
                            "name": "length",
                            "nodeType": "YulIdentifier",
                            "src": "4894:6:1"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "4885:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "4885:16:1"
                      },
                      {
                        "name": "end",
                        "nodeType": "YulIdentifier",
                        "src": "4903:3:1"
                      }
                    ],
                    "functionName": {
                      "name": "gt",
                      "nodeType": "YulIdentifier",
                      "src": "4882:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "4882:25:1"
                  },
                  "nodeType": "YulIf",
                  "src": "4879:112:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "src",
                        "nodeType": "YulIdentifier",
                        "src": "5037:3:1"
                      },
                      {
                        "name": "dst",
                        "nodeType": "YulIdentifier",
                        "src": "5042:3:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "5047:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "copy_calldata_to_memory_with_cleanup",
                      "nodeType": "YulIdentifier",
                      "src": "5000:36:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5000:54:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "5000:54:1"
                }
              ]
            },
            "name": "abi_decode_available_length_t_string_memory_ptr",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "src",
                "nodeType": "YulTypedName",
                "src": "4692:3:1",
                "type": ""
              },
              {
                "name": "length",
                "nodeType": "YulTypedName",
                "src": "4697:6:1",
                "type": ""
              },
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "4705:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "array",
                "nodeType": "YulTypedName",
                "src": "4713:5:1",
                "type": ""
              }
            ],
            "src": "4635:425:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "5142:278:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "5191:83:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "revert_error_1b9f4a0a5773e33b91aa01db23bf8c55fce1411167c872835e7fa00a4f17d46d",
                            "nodeType": "YulIdentifier",
                            "src": "5193:77:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "5193:79:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "5193:79:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "offset",
                                "nodeType": "YulIdentifier",
                                "src": "5170:6:1"
                              },
                              {
                                "kind": "number",
                                "nodeType": "YulLiteral",
                                "src": "5178:4:1",
                                "type": "",
                                "value": "0x1f"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "5166:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "5166:17:1"
                          },
                          {
                            "name": "end",
                            "nodeType": "YulIdentifier",
                            "src": "5185:3:1"
                          }
                        ],
                        "functionName": {
                          "name": "slt",
                          "nodeType": "YulIdentifier",
                          "src": "5162:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "5162:27:1"
                      }
                    ],
                    "functionName": {
                      "name": "iszero",
                      "nodeType": "YulIdentifier",
                      "src": "5155:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5155:35:1"
                  },
                  "nodeType": "YulIf",
                  "src": "5152:122:1"
                },
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "5283:34:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "offset",
                        "nodeType": "YulIdentifier",
                        "src": "5310:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "calldataload",
                      "nodeType": "YulIdentifier",
                      "src": "5297:12:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5297:20:1"
                  },
                  "variables": [
                    {
                      "name": "length",
                      "nodeType": "YulTypedName",
                      "src": "5287:6:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "5326:88:1",
                  "value": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "offset",
                            "nodeType": "YulIdentifier",
                            "src": "5387:6:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "5395:4:1",
                            "type": "",
                            "value": "0x20"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "5383:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "5383:17:1"
                      },
                      {
                        "name": "length",
                        "nodeType": "YulIdentifier",
                        "src": "5402:6:1"
                      },
                      {
                        "name": "end",
                        "nodeType": "YulIdentifier",
                        "src": "5410:3:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_decode_available_length_t_string_memory_ptr",
                      "nodeType": "YulIdentifier",
                      "src": "5335:47:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5335:79:1"
                  },
                  "variableNames": [
                    {
                      "name": "array",
                      "nodeType": "YulIdentifier",
                      "src": "5326:5:1"
                    }
                  ]
                }
              ]
            },
            "name": "abi_decode_t_string_memory_ptr",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "offset",
                "nodeType": "YulTypedName",
                "src": "5120:6:1",
                "type": ""
              },
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "5128:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "array",
                "nodeType": "YulTypedName",
                "src": "5136:5:1",
                "type": ""
              }
            ],
            "src": "5080:340:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "5468:78:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "5524:16:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "5533:1:1",
                              "type": "",
                              "value": "0"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "5536:1:1",
                              "type": "",
                              "value": "0"
                            }
                          ],
                          "functionName": {
                            "name": "revert",
                            "nodeType": "YulIdentifier",
                            "src": "5526:6:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "5526:12:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "5526:12:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "5491:5:1"
                          },
                          {
                            "arguments": [
                              {
                                "name": "value",
                                "nodeType": "YulIdentifier",
                                "src": "5515:5:1"
                              }
                            ],
                            "functionName": {
                              "name": "cleanup_t_uint16",
                              "nodeType": "YulIdentifier",
                              "src": "5498:16:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "5498:23:1"
                          }
                        ],
                        "functionName": {
                          "name": "eq",
                          "nodeType": "YulIdentifier",
                          "src": "5488:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "5488:34:1"
                      }
                    ],
                    "functionName": {
                      "name": "iszero",
                      "nodeType": "YulIdentifier",
                      "src": "5481:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5481:42:1"
                  },
                  "nodeType": "YulIf",
                  "src": "5478:62:1"
                }
              ]
            },
            "name": "validator_revert_t_uint16",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "5461:5:1",
                "type": ""
              }
            ],
            "src": "5426:120:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "5603:86:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "5613:29:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "offset",
                        "nodeType": "YulIdentifier",
                        "src": "5635:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "calldataload",
                      "nodeType": "YulIdentifier",
                      "src": "5622:12:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5622:20:1"
                  },
                  "variableNames": [
                    {
                      "name": "value",
                      "nodeType": "YulIdentifier",
                      "src": "5613:5:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "5677:5:1"
                      }
                    ],
                    "functionName": {
                      "name": "validator_revert_t_uint16",
                      "nodeType": "YulIdentifier",
                      "src": "5651:25:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5651:32:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "5651:32:1"
                }
              ]
            },
            "name": "abi_decode_t_uint16",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "offset",
                "nodeType": "YulTypedName",
                "src": "5581:6:1",
                "type": ""
              },
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "5589:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "5597:5:1",
                "type": ""
              }
            ],
            "src": "5552:137:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "5738:79:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "5795:16:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "5804:1:1",
                              "type": "",
                              "value": "0"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "5807:1:1",
                              "type": "",
                              "value": "0"
                            }
                          ],
                          "functionName": {
                            "name": "revert",
                            "nodeType": "YulIdentifier",
                            "src": "5797:6:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "5797:12:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "5797:12:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "5761:5:1"
                          },
                          {
                            "arguments": [
                              {
                                "name": "value",
                                "nodeType": "YulIdentifier",
                                "src": "5786:5:1"
                              }
                            ],
                            "functionName": {
                              "name": "cleanup_t_bytes32",
                              "nodeType": "YulIdentifier",
                              "src": "5768:17:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "5768:24:1"
                          }
                        ],
                        "functionName": {
                          "name": "eq",
                          "nodeType": "YulIdentifier",
                          "src": "5758:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "5758:35:1"
                      }
                    ],
                    "functionName": {
                      "name": "iszero",
                      "nodeType": "YulIdentifier",
                      "src": "5751:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5751:43:1"
                  },
                  "nodeType": "YulIf",
                  "src": "5748:63:1"
                }
              ]
            },
            "name": "validator_revert_t_bytes32",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "5731:5:1",
                "type": ""
              }
            ],
            "src": "5695:122:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "5875:87:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "5885:29:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "offset",
                        "nodeType": "YulIdentifier",
                        "src": "5907:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "calldataload",
                      "nodeType": "YulIdentifier",
                      "src": "5894:12:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5894:20:1"
                  },
                  "variableNames": [
                    {
                      "name": "value",
                      "nodeType": "YulIdentifier",
                      "src": "5885:5:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "5950:5:1"
                      }
                    ],
                    "functionName": {
                      "name": "validator_revert_t_bytes32",
                      "nodeType": "YulIdentifier",
                      "src": "5923:26:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "5923:33:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "5923:33:1"
                }
              ]
            },
            "name": "abi_decode_t_bytes32",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "offset",
                "nodeType": "YulTypedName",
                "src": "5853:6:1",
                "type": ""
              },
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "5861:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "5869:5:1",
                "type": ""
              }
            ],
            "src": "5823:139:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "6120:1115:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "6167:83:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b",
                            "nodeType": "YulIdentifier",
                            "src": "6169:77:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "6169:79:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "6169:79:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "6141:7:1"
                          },
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "6150:9:1"
                          }
                        ],
                        "functionName": {
                          "name": "sub",
                          "nodeType": "YulIdentifier",
                          "src": "6137:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6137:23:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "6162:3:1",
                        "type": "",
                        "value": "160"
                      }
                    ],
                    "functionName": {
                      "name": "slt",
                      "nodeType": "YulIdentifier",
                      "src": "6133:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "6133:33:1"
                  },
                  "nodeType": "YulIf",
                  "src": "6130:120:1"
                },
                {
                  "nodeType": "YulBlock",
                  "src": "6260:287:1",
                  "statements": [
                    {
                      "nodeType": "YulVariableDeclaration",
                      "src": "6275:45:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "6306:9:1"
                              },
                              {
                                "kind": "number",
                                "nodeType": "YulLiteral",
                                "src": "6317:1:1",
                                "type": "",
                                "value": "0"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "6302:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "6302:17:1"
                          }
                        ],
                        "functionName": {
                          "name": "calldataload",
                          "nodeType": "YulIdentifier",
                          "src": "6289:12:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6289:31:1"
                      },
                      "variables": [
                        {
                          "name": "offset",
                          "nodeType": "YulTypedName",
                          "src": "6279:6:1",
                          "type": ""
                        }
                      ]
                    },
                    {
                      "body": {
                        "nodeType": "YulBlock",
                        "src": "6367:83:1",
                        "statements": [
                          {
                            "expression": {
                              "arguments": [],
                              "functionName": {
                                "name": "revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db",
                                "nodeType": "YulIdentifier",
                                "src": "6369:77:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "6369:79:1"
                            },
                            "nodeType": "YulExpressionStatement",
                            "src": "6369:79:1"
                          }
                        ]
                      },
                      "condition": {
                        "arguments": [
                          {
                            "name": "offset",
                            "nodeType": "YulIdentifier",
                            "src": "6339:6:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "6347:18:1",
                            "type": "",
                            "value": "0xffffffffffffffff"
                          }
                        ],
                        "functionName": {
                          "name": "gt",
                          "nodeType": "YulIdentifier",
                          "src": "6336:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6336:30:1"
                      },
                      "nodeType": "YulIf",
                      "src": "6333:117:1"
                    },
                    {
                      "nodeType": "YulAssignment",
                      "src": "6464:73:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "6509:9:1"
                              },
                              {
                                "name": "offset",
                                "nodeType": "YulIdentifier",
                                "src": "6520:6:1"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "6505:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "6505:22:1"
                          },
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "6529:7:1"
                          }
                        ],
                        "functionName": {
                          "name": "abi_decode_t_string_memory_ptr",
                          "nodeType": "YulIdentifier",
                          "src": "6474:30:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6474:63:1"
                      },
                      "variableNames": [
                        {
                          "name": "value0",
                          "nodeType": "YulIdentifier",
                          "src": "6464:6:1"
                        }
                      ]
                    }
                  ]
                },
                {
                  "nodeType": "YulBlock",
                  "src": "6557:117:1",
                  "statements": [
                    {
                      "nodeType": "YulVariableDeclaration",
                      "src": "6572:16:1",
                      "value": {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "6586:2:1",
                        "type": "",
                        "value": "32"
                      },
                      "variables": [
                        {
                          "name": "offset",
                          "nodeType": "YulTypedName",
                          "src": "6576:6:1",
                          "type": ""
                        }
                      ]
                    },
                    {
                      "nodeType": "YulAssignment",
                      "src": "6602:62:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "6636:9:1"
                              },
                              {
                                "name": "offset",
                                "nodeType": "YulIdentifier",
                                "src": "6647:6:1"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "6632:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "6632:22:1"
                          },
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "6656:7:1"
                          }
                        ],
                        "functionName": {
                          "name": "abi_decode_t_uint16",
                          "nodeType": "YulIdentifier",
                          "src": "6612:19:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6612:52:1"
                      },
                      "variableNames": [
                        {
                          "name": "value1",
                          "nodeType": "YulIdentifier",
                          "src": "6602:6:1"
                        }
                      ]
                    }
                  ]
                },
                {
                  "nodeType": "YulBlock",
                  "src": "6684:117:1",
                  "statements": [
                    {
                      "nodeType": "YulVariableDeclaration",
                      "src": "6699:16:1",
                      "value": {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "6713:2:1",
                        "type": "",
                        "value": "64"
                      },
                      "variables": [
                        {
                          "name": "offset",
                          "nodeType": "YulTypedName",
                          "src": "6703:6:1",
                          "type": ""
                        }
                      ]
                    },
                    {
                      "nodeType": "YulAssignment",
                      "src": "6729:62:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "6763:9:1"
                              },
                              {
                                "name": "offset",
                                "nodeType": "YulIdentifier",
                                "src": "6774:6:1"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "6759:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "6759:22:1"
                          },
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "6783:7:1"
                          }
                        ],
                        "functionName": {
                          "name": "abi_decode_t_uint16",
                          "nodeType": "YulIdentifier",
                          "src": "6739:19:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6739:52:1"
                      },
                      "variableNames": [
                        {
                          "name": "value2",
                          "nodeType": "YulIdentifier",
                          "src": "6729:6:1"
                        }
                      ]
                    }
                  ]
                },
                {
                  "nodeType": "YulBlock",
                  "src": "6811:288:1",
                  "statements": [
                    {
                      "nodeType": "YulVariableDeclaration",
                      "src": "6826:46:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "6857:9:1"
                              },
                              {
                                "kind": "number",
                                "nodeType": "YulLiteral",
                                "src": "6868:2:1",
                                "type": "",
                                "value": "96"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "6853:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "6853:18:1"
                          }
                        ],
                        "functionName": {
                          "name": "calldataload",
                          "nodeType": "YulIdentifier",
                          "src": "6840:12:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6840:32:1"
                      },
                      "variables": [
                        {
                          "name": "offset",
                          "nodeType": "YulTypedName",
                          "src": "6830:6:1",
                          "type": ""
                        }
                      ]
                    },
                    {
                      "body": {
                        "nodeType": "YulBlock",
                        "src": "6919:83:1",
                        "statements": [
                          {
                            "expression": {
                              "arguments": [],
                              "functionName": {
                                "name": "revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db",
                                "nodeType": "YulIdentifier",
                                "src": "6921:77:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "6921:79:1"
                            },
                            "nodeType": "YulExpressionStatement",
                            "src": "6921:79:1"
                          }
                        ]
                      },
                      "condition": {
                        "arguments": [
                          {
                            "name": "offset",
                            "nodeType": "YulIdentifier",
                            "src": "6891:6:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "6899:18:1",
                            "type": "",
                            "value": "0xffffffffffffffff"
                          }
                        ],
                        "functionName": {
                          "name": "gt",
                          "nodeType": "YulIdentifier",
                          "src": "6888:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "6888:30:1"
                      },
                      "nodeType": "YulIf",
                      "src": "6885:117:1"
                    },
                    {
                      "nodeType": "YulAssignment",
                      "src": "7016:73:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "7061:9:1"
                              },
                              {
                                "name": "offset",
                                "nodeType": "YulIdentifier",
                                "src": "7072:6:1"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "7057:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "7057:22:1"
                          },
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "7081:7:1"
                          }
                        ],
                        "functionName": {
                          "name": "abi_decode_t_string_memory_ptr",
                          "nodeType": "YulIdentifier",
                          "src": "7026:30:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "7026:63:1"
                      },
                      "variableNames": [
                        {
                          "name": "value3",
                          "nodeType": "YulIdentifier",
                          "src": "7016:6:1"
                        }
                      ]
                    }
                  ]
                },
                {
                  "nodeType": "YulBlock",
                  "src": "7109:119:1",
                  "statements": [
                    {
                      "nodeType": "YulVariableDeclaration",
                      "src": "7124:17:1",
                      "value": {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "7138:3:1",
                        "type": "",
                        "value": "128"
                      },
                      "variables": [
                        {
                          "name": "offset",
                          "nodeType": "YulTypedName",
                          "src": "7128:6:1",
                          "type": ""
                        }
                      ]
                    },
                    {
                      "nodeType": "YulAssignment",
                      "src": "7155:63:1",
                      "value": {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "headStart",
                                "nodeType": "YulIdentifier",
                                "src": "7190:9:1"
                              },
                              {
                                "name": "offset",
                                "nodeType": "YulIdentifier",
                                "src": "7201:6:1"
                              }
                            ],
                            "functionName": {
                              "name": "add",
                              "nodeType": "YulIdentifier",
                              "src": "7186:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "7186:22:1"
                          },
                          {
                            "name": "dataEnd",
                            "nodeType": "YulIdentifier",
                            "src": "7210:7:1"
                          }
                        ],
                        "functionName": {
                          "name": "abi_decode_t_bytes32",
                          "nodeType": "YulIdentifier",
                          "src": "7165:20:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "7165:53:1"
                      },
                      "variableNames": [
                        {
                          "name": "value4",
                          "nodeType": "YulIdentifier",
                          "src": "7155:6:1"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            "name": "abi_decode_tuple_t_string_memory_ptrt_uint16t_uint16t_string_memory_ptrt_bytes32",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "headStart",
                "nodeType": "YulTypedName",
                "src": "6058:9:1",
                "type": ""
              },
              {
                "name": "dataEnd",
                "nodeType": "YulTypedName",
                "src": "6069:7:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "value0",
                "nodeType": "YulTypedName",
                "src": "6081:6:1",
                "type": ""
              },
              {
                "name": "value1",
                "nodeType": "YulTypedName",
                "src": "6089:6:1",
                "type": ""
              },
              {
                "name": "value2",
                "nodeType": "YulTypedName",
                "src": "6097:6:1",
                "type": ""
              },
              {
                "name": "value3",
                "nodeType": "YulTypedName",
                "src": "6105:6:1",
                "type": ""
              },
              {
                "name": "value4",
                "nodeType": "YulTypedName",
                "src": "6113:6:1",
                "type": ""
              }
            ],
            "src": "5968:1267:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "7306:53:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "7323:3:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "7346:5:1"
                          }
                        ],
                        "functionName": {
                          "name": "cleanup_t_uint256",
                          "nodeType": "YulIdentifier",
                          "src": "7328:17:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "7328:24:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "7316:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "7316:37:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "7316:37:1"
                }
              ]
            },
            "name": "abi_encode_t_uint256_to_t_uint256_fromStack",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "7294:5:1",
                "type": ""
              },
              {
                "name": "pos",
                "nodeType": "YulTypedName",
                "src": "7301:3:1",
                "type": ""
              }
            ],
            "src": "7241:118:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "7463:124:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "7473:26:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "headStart",
                        "nodeType": "YulIdentifier",
                        "src": "7485:9:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "7496:2:1",
                        "type": "",
                        "value": "32"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "7481:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "7481:18:1"
                  },
                  "variableNames": [
                    {
                      "name": "tail",
                      "nodeType": "YulIdentifier",
                      "src": "7473:4:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "value0",
                        "nodeType": "YulIdentifier",
                        "src": "7553:6:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "7566:9:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "7577:1:1",
                            "type": "",
                            "value": "0"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "7562:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "7562:17:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_encode_t_uint256_to_t_uint256_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "7509:43:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "7509:71:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "7509:71:1"
                }
              ]
            },
            "name": "abi_encode_tuple_t_uint256__to_t_uint256__fromStack_reversed",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "headStart",
                "nodeType": "YulTypedName",
                "src": "7435:9:1",
                "type": ""
              },
              {
                "name": "value0",
                "nodeType": "YulTypedName",
                "src": "7447:6:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "tail",
                "nodeType": "YulTypedName",
                "src": "7458:4:1",
                "type": ""
              }
            ],
            "src": "7365:222:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "7699:57:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "memPtr",
                            "nodeType": "YulIdentifier",
                            "src": "7721:6:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "7729:1:1",
                            "type": "",
                            "value": "0"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "7717:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "7717:14:1"
                      },
                      {
                        "hexValue": "496e76616c696420696e646578",
                        "kind": "string",
                        "nodeType": "YulLiteral",
                        "src": "7733:15:1",
                        "type": "",
                        "value": "Invalid index"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "7710:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "7710:39:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "7710:39:1"
                }
              ]
            },
            "name": "store_literal_in_memory_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "memPtr",
                "nodeType": "YulTypedName",
                "src": "7691:6:1",
                "type": ""
              }
            ],
            "src": "7593:163:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "7908:220:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "7918:74:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "7984:3:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "7989:2:1",
                        "type": "",
                        "value": "13"
                      }
                    ],
                    "functionName": {
                      "name": "array_storeLengthForEncoding_t_string_memory_ptr_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "7925:58:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "7925:67:1"
                  },
                  "variableNames": [
                    {
                      "name": "pos",
                      "nodeType": "YulIdentifier",
                      "src": "7918:3:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "8090:3:1"
                      }
                    ],
                    "functionName": {
                      "name": "store_literal_in_memory_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60",
                      "nodeType": "YulIdentifier",
                      "src": "8001:88:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8001:93:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8001:93:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "8103:19:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "pos",
                        "nodeType": "YulIdentifier",
                        "src": "8114:3:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8119:2:1",
                        "type": "",
                        "value": "32"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "8110:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8110:12:1"
                  },
                  "variableNames": [
                    {
                      "name": "end",
                      "nodeType": "YulIdentifier",
                      "src": "8103:3:1"
                    }
                  ]
                }
              ]
            },
            "name": "abi_encode_t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60_to_t_string_memory_ptr_fromStack",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "pos",
                "nodeType": "YulTypedName",
                "src": "7896:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "7904:3:1",
                "type": ""
              }
            ],
            "src": "7762:366:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "8305:248:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "8315:26:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "headStart",
                        "nodeType": "YulIdentifier",
                        "src": "8327:9:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8338:2:1",
                        "type": "",
                        "value": "32"
                      }
                    ],
                    "functionName": {
                      "name": "add",
                      "nodeType": "YulIdentifier",
                      "src": "8323:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8323:18:1"
                  },
                  "variableNames": [
                    {
                      "name": "tail",
                      "nodeType": "YulIdentifier",
                      "src": "8315:4:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "8362:9:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "8373:1:1",
                            "type": "",
                            "value": "0"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "8358:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "8358:17:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "tail",
                            "nodeType": "YulIdentifier",
                            "src": "8381:4:1"
                          },
                          {
                            "name": "headStart",
                            "nodeType": "YulIdentifier",
                            "src": "8387:9:1"
                          }
                        ],
                        "functionName": {
                          "name": "sub",
                          "nodeType": "YulIdentifier",
                          "src": "8377:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "8377:20:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "8351:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8351:47:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8351:47:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "8407:139:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "tail",
                        "nodeType": "YulIdentifier",
                        "src": "8541:4:1"
                      }
                    ],
                    "functionName": {
                      "name": "abi_encode_t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60_to_t_string_memory_ptr_fromStack",
                      "nodeType": "YulIdentifier",
                      "src": "8415:124:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8415:131:1"
                  },
                  "variableNames": [
                    {
                      "name": "tail",
                      "nodeType": "YulIdentifier",
                      "src": "8407:4:1"
                    }
                  ]
                }
              ]
            },
            "name": "abi_encode_tuple_t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60__to_t_string_memory_ptr__fromStack_reversed",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "headStart",
                "nodeType": "YulTypedName",
                "src": "8285:9:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "tail",
                "nodeType": "YulTypedName",
                "src": "8300:4:1",
                "type": ""
              }
            ],
            "src": "8134:419:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "8587:152:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8604:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8607:77:1",
                        "type": "",
                        "value": "35408467139433450592217433187231851964531694900788300625387963629091585785856"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "8597:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8597:88:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8597:88:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8701:1:1",
                        "type": "",
                        "value": "4"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8704:4:1",
                        "type": "",
                        "value": "0x32"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "8694:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8694:15:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8694:15:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8725:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8728:4:1",
                        "type": "",
                        "value": "0x24"
                      }
                    ],
                    "functionName": {
                      "name": "revert",
                      "nodeType": "YulIdentifier",
                      "src": "8718:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8718:15:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8718:15:1"
                }
              ]
            },
            "name": "panic_error_0x32",
            "nodeType": "YulFunctionDefinition",
            "src": "8559:180:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "8773:152:1",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8790:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8793:77:1",
                        "type": "",
                        "value": "35408467139433450592217433187231851964531694900788300625387963629091585785856"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "8783:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8783:88:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8783:88:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8887:1:1",
                        "type": "",
                        "value": "4"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8890:4:1",
                        "type": "",
                        "value": "0x22"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "8880:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8880:15:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8880:15:1"
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8911:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "8914:4:1",
                        "type": "",
                        "value": "0x24"
                      }
                    ],
                    "functionName": {
                      "name": "revert",
                      "nodeType": "YulIdentifier",
                      "src": "8904:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "8904:15:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "8904:15:1"
                }
              ]
            },
            "name": "panic_error_0x22",
            "nodeType": "YulFunctionDefinition",
            "src": "8745:180:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "8982:269:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "8992:22:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "data",
                        "nodeType": "YulIdentifier",
                        "src": "9006:4:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9012:1:1",
                        "type": "",
                        "value": "2"
                      }
                    ],
                    "functionName": {
                      "name": "div",
                      "nodeType": "YulIdentifier",
                      "src": "9002:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9002:12:1"
                  },
                  "variableNames": [
                    {
                      "name": "length",
                      "nodeType": "YulIdentifier",
                      "src": "8992:6:1"
                    }
                  ]
                },
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "9023:38:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "data",
                        "nodeType": "YulIdentifier",
                        "src": "9053:4:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9059:1:1",
                        "type": "",
                        "value": "1"
                      }
                    ],
                    "functionName": {
                      "name": "and",
                      "nodeType": "YulIdentifier",
                      "src": "9049:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9049:12:1"
                  },
                  "variables": [
                    {
                      "name": "outOfPlaceEncoding",
                      "nodeType": "YulTypedName",
                      "src": "9027:18:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "9100:51:1",
                    "statements": [
                      {
                        "nodeType": "YulAssignment",
                        "src": "9114:27:1",
                        "value": {
                          "arguments": [
                            {
                              "name": "length",
                              "nodeType": "YulIdentifier",
                              "src": "9128:6:1"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "9136:4:1",
                              "type": "",
                              "value": "0x7f"
                            }
                          ],
                          "functionName": {
                            "name": "and",
                            "nodeType": "YulIdentifier",
                            "src": "9124:3:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "9124:17:1"
                        },
                        "variableNames": [
                          {
                            "name": "length",
                            "nodeType": "YulIdentifier",
                            "src": "9114:6:1"
                          }
                        ]
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "name": "outOfPlaceEncoding",
                        "nodeType": "YulIdentifier",
                        "src": "9080:18:1"
                      }
                    ],
                    "functionName": {
                      "name": "iszero",
                      "nodeType": "YulIdentifier",
                      "src": "9073:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9073:26:1"
                  },
                  "nodeType": "YulIf",
                  "src": "9070:81:1"
                },
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "9203:42:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "panic_error_0x22",
                            "nodeType": "YulIdentifier",
                            "src": "9217:16:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "9217:18:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "9217:18:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "name": "outOfPlaceEncoding",
                        "nodeType": "YulIdentifier",
                        "src": "9167:18:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "length",
                            "nodeType": "YulIdentifier",
                            "src": "9190:6:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "9198:2:1",
                            "type": "",
                            "value": "32"
                          }
                        ],
                        "functionName": {
                          "name": "lt",
                          "nodeType": "YulIdentifier",
                          "src": "9187:2:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "9187:14:1"
                      }
                    ],
                    "functionName": {
                      "name": "eq",
                      "nodeType": "YulIdentifier",
                      "src": "9164:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9164:38:1"
                  },
                  "nodeType": "YulIf",
                  "src": "9161:84:1"
                }
              ]
            },
            "name": "extract_byte_array_length",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "data",
                "nodeType": "YulTypedName",
                "src": "8966:4:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "length",
                "nodeType": "YulTypedName",
                "src": "8975:6:1",
                "type": ""
              }
            ],
            "src": "8931:320:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "9311:87:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "9321:11:1",
                  "value": {
                    "name": "ptr",
                    "nodeType": "YulIdentifier",
                    "src": "9329:3:1"
                  },
                  "variableNames": [
                    {
                      "name": "data",
                      "nodeType": "YulIdentifier",
                      "src": "9321:4:1"
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9349:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "name": "ptr",
                        "nodeType": "YulIdentifier",
                        "src": "9352:3:1"
                      }
                    ],
                    "functionName": {
                      "name": "mstore",
                      "nodeType": "YulIdentifier",
                      "src": "9342:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9342:14:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "9342:14:1"
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "9365:26:1",
                  "value": {
                    "arguments": [
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9383:1:1",
                        "type": "",
                        "value": "0"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9386:4:1",
                        "type": "",
                        "value": "0x20"
                      }
                    ],
                    "functionName": {
                      "name": "keccak256",
                      "nodeType": "YulIdentifier",
                      "src": "9373:9:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9373:18:1"
                  },
                  "variableNames": [
                    {
                      "name": "data",
                      "nodeType": "YulIdentifier",
                      "src": "9365:4:1"
                    }
                  ]
                }
              ]
            },
            "name": "array_dataslot_t_string_storage",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "ptr",
                "nodeType": "YulTypedName",
                "src": "9298:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "data",
                "nodeType": "YulTypedName",
                "src": "9306:4:1",
                "type": ""
              }
            ],
            "src": "9257:141:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "9448:49:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "9458:33:1",
                  "value": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "value",
                            "nodeType": "YulIdentifier",
                            "src": "9476:5:1"
                          },
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "9483:2:1",
                            "type": "",
                            "value": "31"
                          }
                        ],
                        "functionName": {
                          "name": "add",
                          "nodeType": "YulIdentifier",
                          "src": "9472:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "9472:14:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9488:2:1",
                        "type": "",
                        "value": "32"
                      }
                    ],
                    "functionName": {
                      "name": "div",
                      "nodeType": "YulIdentifier",
                      "src": "9468:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9468:23:1"
                  },
                  "variableNames": [
                    {
                      "name": "result",
                      "nodeType": "YulIdentifier",
                      "src": "9458:6:1"
                    }
                  ]
                }
              ]
            },
            "name": "divide_by_32_ceil",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "9431:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "result",
                "nodeType": "YulTypedName",
                "src": "9441:6:1",
                "type": ""
              }
            ],
            "src": "9404:93:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "9556:54:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "9566:37:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "bits",
                        "nodeType": "YulIdentifier",
                        "src": "9591:4:1"
                      },
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "9597:5:1"
                      }
                    ],
                    "functionName": {
                      "name": "shl",
                      "nodeType": "YulIdentifier",
                      "src": "9587:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9587:16:1"
                  },
                  "variableNames": [
                    {
                      "name": "newValue",
                      "nodeType": "YulIdentifier",
                      "src": "9566:8:1"
                    }
                  ]
                }
              ]
            },
            "name": "shift_left_dynamic",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "bits",
                "nodeType": "YulTypedName",
                "src": "9531:4:1",
                "type": ""
              },
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "9537:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "newValue",
                "nodeType": "YulTypedName",
                "src": "9547:8:1",
                "type": ""
              }
            ],
            "src": "9503:107:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "9692:317:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "9702:35:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "shiftBytes",
                        "nodeType": "YulIdentifier",
                        "src": "9723:10:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9735:1:1",
                        "type": "",
                        "value": "8"
                      }
                    ],
                    "functionName": {
                      "name": "mul",
                      "nodeType": "YulIdentifier",
                      "src": "9719:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9719:18:1"
                  },
                  "variables": [
                    {
                      "name": "shiftBits",
                      "nodeType": "YulTypedName",
                      "src": "9706:9:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "9746:109:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "shiftBits",
                        "nodeType": "YulIdentifier",
                        "src": "9777:9:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "9788:66:1",
                        "type": "",
                        "value": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                      }
                    ],
                    "functionName": {
                      "name": "shift_left_dynamic",
                      "nodeType": "YulIdentifier",
                      "src": "9758:18:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9758:97:1"
                  },
                  "variables": [
                    {
                      "name": "mask",
                      "nodeType": "YulTypedName",
                      "src": "9750:4:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "9864:51:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "shiftBits",
                        "nodeType": "YulIdentifier",
                        "src": "9895:9:1"
                      },
                      {
                        "name": "toInsert",
                        "nodeType": "YulIdentifier",
                        "src": "9906:8:1"
                      }
                    ],
                    "functionName": {
                      "name": "shift_left_dynamic",
                      "nodeType": "YulIdentifier",
                      "src": "9876:18:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9876:39:1"
                  },
                  "variableNames": [
                    {
                      "name": "toInsert",
                      "nodeType": "YulIdentifier",
                      "src": "9864:8:1"
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "9924:30:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "9937:5:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "mask",
                            "nodeType": "YulIdentifier",
                            "src": "9948:4:1"
                          }
                        ],
                        "functionName": {
                          "name": "not",
                          "nodeType": "YulIdentifier",
                          "src": "9944:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "9944:9:1"
                      }
                    ],
                    "functionName": {
                      "name": "and",
                      "nodeType": "YulIdentifier",
                      "src": "9933:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9933:21:1"
                  },
                  "variableNames": [
                    {
                      "name": "value",
                      "nodeType": "YulIdentifier",
                      "src": "9924:5:1"
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "9963:40:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "9976:5:1"
                      },
                      {
                        "arguments": [
                          {
                            "name": "toInsert",
                            "nodeType": "YulIdentifier",
                            "src": "9987:8:1"
                          },
                          {
                            "name": "mask",
                            "nodeType": "YulIdentifier",
                            "src": "9997:4:1"
                          }
                        ],
                        "functionName": {
                          "name": "and",
                          "nodeType": "YulIdentifier",
                          "src": "9983:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "9983:19:1"
                      }
                    ],
                    "functionName": {
                      "name": "or",
                      "nodeType": "YulIdentifier",
                      "src": "9973:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "9973:30:1"
                  },
                  "variableNames": [
                    {
                      "name": "result",
                      "nodeType": "YulIdentifier",
                      "src": "9963:6:1"
                    }
                  ]
                }
              ]
            },
            "name": "update_byte_slice_dynamic32",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "9653:5:1",
                "type": ""
              },
              {
                "name": "shiftBytes",
                "nodeType": "YulTypedName",
                "src": "9660:10:1",
                "type": ""
              },
              {
                "name": "toInsert",
                "nodeType": "YulTypedName",
                "src": "9672:8:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "result",
                "nodeType": "YulTypedName",
                "src": "9685:6:1",
                "type": ""
              }
            ],
            "src": "9616:393:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "10047:28:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "10057:12:1",
                  "value": {
                    "name": "value",
                    "nodeType": "YulIdentifier",
                    "src": "10064:5:1"
                  },
                  "variableNames": [
                    {
                      "name": "ret",
                      "nodeType": "YulIdentifier",
                      "src": "10057:3:1"
                    }
                  ]
                }
              ]
            },
            "name": "identity",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "10033:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "ret",
                "nodeType": "YulTypedName",
                "src": "10043:3:1",
                "type": ""
              }
            ],
            "src": "10015:60:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "10141:82:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "10151:66:1",
                  "value": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "value",
                                "nodeType": "YulIdentifier",
                                "src": "10209:5:1"
                              }
                            ],
                            "functionName": {
                              "name": "cleanup_t_uint256",
                              "nodeType": "YulIdentifier",
                              "src": "10191:17:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "10191:24:1"
                          }
                        ],
                        "functionName": {
                          "name": "identity",
                          "nodeType": "YulIdentifier",
                          "src": "10182:8:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "10182:34:1"
                      }
                    ],
                    "functionName": {
                      "name": "cleanup_t_uint256",
                      "nodeType": "YulIdentifier",
                      "src": "10164:17:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "10164:53:1"
                  },
                  "variableNames": [
                    {
                      "name": "converted",
                      "nodeType": "YulIdentifier",
                      "src": "10151:9:1"
                    }
                  ]
                }
              ]
            },
            "name": "convert_t_uint256_to_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "10121:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "converted",
                "nodeType": "YulTypedName",
                "src": "10131:9:1",
                "type": ""
              }
            ],
            "src": "10081:142:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "10276:28:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "10286:12:1",
                  "value": {
                    "name": "value",
                    "nodeType": "YulIdentifier",
                    "src": "10293:5:1"
                  },
                  "variableNames": [
                    {
                      "name": "ret",
                      "nodeType": "YulIdentifier",
                      "src": "10286:3:1"
                    }
                  ]
                }
              ]
            },
            "name": "prepare_store_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "10262:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "ret",
                "nodeType": "YulTypedName",
                "src": "10272:3:1",
                "type": ""
              }
            ],
            "src": "10229:75:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "10386:193:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "10396:63:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "value_0",
                        "nodeType": "YulIdentifier",
                        "src": "10451:7:1"
                      }
                    ],
                    "functionName": {
                      "name": "convert_t_uint256_to_t_uint256",
                      "nodeType": "YulIdentifier",
                      "src": "10420:30:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "10420:39:1"
                  },
                  "variables": [
                    {
                      "name": "convertedValue_0",
                      "nodeType": "YulTypedName",
                      "src": "10400:16:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "slot",
                        "nodeType": "YulIdentifier",
                        "src": "10475:4:1"
                      },
                      {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "name": "slot",
                                "nodeType": "YulIdentifier",
                                "src": "10515:4:1"
                              }
                            ],
                            "functionName": {
                              "name": "sload",
                              "nodeType": "YulIdentifier",
                              "src": "10509:5:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "10509:11:1"
                          },
                          {
                            "name": "offset",
                            "nodeType": "YulIdentifier",
                            "src": "10522:6:1"
                          },
                          {
                            "arguments": [
                              {
                                "name": "convertedValue_0",
                                "nodeType": "YulIdentifier",
                                "src": "10554:16:1"
                              }
                            ],
                            "functionName": {
                              "name": "prepare_store_t_uint256",
                              "nodeType": "YulIdentifier",
                              "src": "10530:23:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "10530:41:1"
                          }
                        ],
                        "functionName": {
                          "name": "update_byte_slice_dynamic32",
                          "nodeType": "YulIdentifier",
                          "src": "10481:27:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "10481:91:1"
                      }
                    ],
                    "functionName": {
                      "name": "sstore",
                      "nodeType": "YulIdentifier",
                      "src": "10468:6:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "10468:105:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "10468:105:1"
                }
              ]
            },
            "name": "update_storage_value_t_uint256_to_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "slot",
                "nodeType": "YulTypedName",
                "src": "10363:4:1",
                "type": ""
              },
              {
                "name": "offset",
                "nodeType": "YulTypedName",
                "src": "10369:6:1",
                "type": ""
              },
              {
                "name": "value_0",
                "nodeType": "YulTypedName",
                "src": "10377:7:1",
                "type": ""
              }
            ],
            "src": "10310:269:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "10634:24:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "10644:8:1",
                  "value": {
                    "kind": "number",
                    "nodeType": "YulLiteral",
                    "src": "10651:1:1",
                    "type": "",
                    "value": "0"
                  },
                  "variableNames": [
                    {
                      "name": "ret",
                      "nodeType": "YulIdentifier",
                      "src": "10644:3:1"
                    }
                  ]
                }
              ]
            },
            "name": "zero_value_for_split_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "returnVariables": [
              {
                "name": "ret",
                "nodeType": "YulTypedName",
                "src": "10630:3:1",
                "type": ""
              }
            ],
            "src": "10585:73:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "10717:136:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "10727:46:1",
                  "value": {
                    "arguments": [],
                    "functionName": {
                      "name": "zero_value_for_split_t_uint256",
                      "nodeType": "YulIdentifier",
                      "src": "10741:30:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "10741:32:1"
                  },
                  "variables": [
                    {
                      "name": "zero_0",
                      "nodeType": "YulTypedName",
                      "src": "10731:6:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "slot",
                        "nodeType": "YulIdentifier",
                        "src": "10826:4:1"
                      },
                      {
                        "name": "offset",
                        "nodeType": "YulIdentifier",
                        "src": "10832:6:1"
                      },
                      {
                        "name": "zero_0",
                        "nodeType": "YulIdentifier",
                        "src": "10840:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "update_storage_value_t_uint256_to_t_uint256",
                      "nodeType": "YulIdentifier",
                      "src": "10782:43:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "10782:65:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "10782:65:1"
                }
              ]
            },
            "name": "storage_set_to_zero_t_uint256",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "slot",
                "nodeType": "YulTypedName",
                "src": "10703:4:1",
                "type": ""
              },
              {
                "name": "offset",
                "nodeType": "YulTypedName",
                "src": "10709:6:1",
                "type": ""
              }
            ],
            "src": "10664:189:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "10909:136:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "10976:63:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [
                            {
                              "name": "start",
                              "nodeType": "YulIdentifier",
                              "src": "11020:5:1"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "11027:1:1",
                              "type": "",
                              "value": "0"
                            }
                          ],
                          "functionName": {
                            "name": "storage_set_to_zero_t_uint256",
                            "nodeType": "YulIdentifier",
                            "src": "10990:29:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "10990:39:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "10990:39:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "name": "start",
                        "nodeType": "YulIdentifier",
                        "src": "10929:5:1"
                      },
                      {
                        "name": "end",
                        "nodeType": "YulIdentifier",
                        "src": "10936:3:1"
                      }
                    ],
                    "functionName": {
                      "name": "lt",
                      "nodeType": "YulIdentifier",
                      "src": "10926:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "10926:14:1"
                  },
                  "nodeType": "YulForLoop",
                  "post": {
                    "nodeType": "YulBlock",
                    "src": "10941:26:1",
                    "statements": [
                      {
                        "nodeType": "YulAssignment",
                        "src": "10943:22:1",
                        "value": {
                          "arguments": [
                            {
                              "name": "start",
                              "nodeType": "YulIdentifier",
                              "src": "10956:5:1"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "10963:1:1",
                              "type": "",
                              "value": "1"
                            }
                          ],
                          "functionName": {
                            "name": "add",
                            "nodeType": "YulIdentifier",
                            "src": "10952:3:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "10952:13:1"
                        },
                        "variableNames": [
                          {
                            "name": "start",
                            "nodeType": "YulIdentifier",
                            "src": "10943:5:1"
                          }
                        ]
                      }
                    ]
                  },
                  "pre": {
                    "nodeType": "YulBlock",
                    "src": "10923:2:1",
                    "statements": []
                  },
                  "src": "10919:120:1"
                }
              ]
            },
            "name": "clear_storage_range_t_bytes1",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "start",
                "nodeType": "YulTypedName",
                "src": "10897:5:1",
                "type": ""
              },
              {
                "name": "end",
                "nodeType": "YulTypedName",
                "src": "10904:3:1",
                "type": ""
              }
            ],
            "src": "10859:186:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "11130:464:1",
              "statements": [
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "11156:431:1",
                    "statements": [
                      {
                        "nodeType": "YulVariableDeclaration",
                        "src": "11170:54:1",
                        "value": {
                          "arguments": [
                            {
                              "name": "array",
                              "nodeType": "YulIdentifier",
                              "src": "11218:5:1"
                            }
                          ],
                          "functionName": {
                            "name": "array_dataslot_t_string_storage",
                            "nodeType": "YulIdentifier",
                            "src": "11186:31:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "11186:38:1"
                        },
                        "variables": [
                          {
                            "name": "dataArea",
                            "nodeType": "YulTypedName",
                            "src": "11174:8:1",
                            "type": ""
                          }
                        ]
                      },
                      {
                        "nodeType": "YulVariableDeclaration",
                        "src": "11237:63:1",
                        "value": {
                          "arguments": [
                            {
                              "name": "dataArea",
                              "nodeType": "YulIdentifier",
                              "src": "11260:8:1"
                            },
                            {
                              "arguments": [
                                {
                                  "name": "startIndex",
                                  "nodeType": "YulIdentifier",
                                  "src": "11288:10:1"
                                }
                              ],
                              "functionName": {
                                "name": "divide_by_32_ceil",
                                "nodeType": "YulIdentifier",
                                "src": "11270:17:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "11270:29:1"
                            }
                          ],
                          "functionName": {
                            "name": "add",
                            "nodeType": "YulIdentifier",
                            "src": "11256:3:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "11256:44:1"
                        },
                        "variables": [
                          {
                            "name": "deleteStart",
                            "nodeType": "YulTypedName",
                            "src": "11241:11:1",
                            "type": ""
                          }
                        ]
                      },
                      {
                        "body": {
                          "nodeType": "YulBlock",
                          "src": "11457:27:1",
                          "statements": [
                            {
                              "nodeType": "YulAssignment",
                              "src": "11459:23:1",
                              "value": {
                                "name": "dataArea",
                                "nodeType": "YulIdentifier",
                                "src": "11474:8:1"
                              },
                              "variableNames": [
                                {
                                  "name": "deleteStart",
                                  "nodeType": "YulIdentifier",
                                  "src": "11459:11:1"
                                }
                              ]
                            }
                          ]
                        },
                        "condition": {
                          "arguments": [
                            {
                              "name": "startIndex",
                              "nodeType": "YulIdentifier",
                              "src": "11441:10:1"
                            },
                            {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "11453:2:1",
                              "type": "",
                              "value": "32"
                            }
                          ],
                          "functionName": {
                            "name": "lt",
                            "nodeType": "YulIdentifier",
                            "src": "11438:2:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "11438:18:1"
                        },
                        "nodeType": "YulIf",
                        "src": "11435:49:1"
                      },
                      {
                        "expression": {
                          "arguments": [
                            {
                              "name": "deleteStart",
                              "nodeType": "YulIdentifier",
                              "src": "11526:11:1"
                            },
                            {
                              "arguments": [
                                {
                                  "name": "dataArea",
                                  "nodeType": "YulIdentifier",
                                  "src": "11543:8:1"
                                },
                                {
                                  "arguments": [
                                    {
                                      "name": "len",
                                      "nodeType": "YulIdentifier",
                                      "src": "11571:3:1"
                                    }
                                  ],
                                  "functionName": {
                                    "name": "divide_by_32_ceil",
                                    "nodeType": "YulIdentifier",
                                    "src": "11553:17:1"
                                  },
                                  "nodeType": "YulFunctionCall",
                                  "src": "11553:22:1"
                                }
                              ],
                              "functionName": {
                                "name": "add",
                                "nodeType": "YulIdentifier",
                                "src": "11539:3:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "11539:37:1"
                            }
                          ],
                          "functionName": {
                            "name": "clear_storage_range_t_bytes1",
                            "nodeType": "YulIdentifier",
                            "src": "11497:28:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "11497:80:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "11497:80:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "name": "len",
                        "nodeType": "YulIdentifier",
                        "src": "11147:3:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "11152:2:1",
                        "type": "",
                        "value": "31"
                      }
                    ],
                    "functionName": {
                      "name": "gt",
                      "nodeType": "YulIdentifier",
                      "src": "11144:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "11144:11:1"
                  },
                  "nodeType": "YulIf",
                  "src": "11141:446:1"
                }
              ]
            },
            "name": "clean_up_bytearray_end_slots_t_string_storage",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "array",
                "nodeType": "YulTypedName",
                "src": "11106:5:1",
                "type": ""
              },
              {
                "name": "len",
                "nodeType": "YulTypedName",
                "src": "11113:3:1",
                "type": ""
              },
              {
                "name": "startIndex",
                "nodeType": "YulTypedName",
                "src": "11118:10:1",
                "type": ""
              }
            ],
            "src": "11051:543:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "11663:54:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "11673:37:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "bits",
                        "nodeType": "YulIdentifier",
                        "src": "11698:4:1"
                      },
                      {
                        "name": "value",
                        "nodeType": "YulIdentifier",
                        "src": "11704:5:1"
                      }
                    ],
                    "functionName": {
                      "name": "shr",
                      "nodeType": "YulIdentifier",
                      "src": "11694:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "11694:16:1"
                  },
                  "variableNames": [
                    {
                      "name": "newValue",
                      "nodeType": "YulIdentifier",
                      "src": "11673:8:1"
                    }
                  ]
                }
              ]
            },
            "name": "shift_right_unsigned_dynamic",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "bits",
                "nodeType": "YulTypedName",
                "src": "11638:4:1",
                "type": ""
              },
              {
                "name": "value",
                "nodeType": "YulTypedName",
                "src": "11644:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "newValue",
                "nodeType": "YulTypedName",
                "src": "11654:8:1",
                "type": ""
              }
            ],
            "src": "11600:117:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "11774:118:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "11784:68:1",
                  "value": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "arguments": [
                              {
                                "kind": "number",
                                "nodeType": "YulLiteral",
                                "src": "11833:1:1",
                                "type": "",
                                "value": "8"
                              },
                              {
                                "name": "bytes",
                                "nodeType": "YulIdentifier",
                                "src": "11836:5:1"
                              }
                            ],
                            "functionName": {
                              "name": "mul",
                              "nodeType": "YulIdentifier",
                              "src": "11829:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "11829:13:1"
                          },
                          {
                            "arguments": [
                              {
                                "kind": "number",
                                "nodeType": "YulLiteral",
                                "src": "11848:1:1",
                                "type": "",
                                "value": "0"
                              }
                            ],
                            "functionName": {
                              "name": "not",
                              "nodeType": "YulIdentifier",
                              "src": "11844:3:1"
                            },
                            "nodeType": "YulFunctionCall",
                            "src": "11844:6:1"
                          }
                        ],
                        "functionName": {
                          "name": "shift_right_unsigned_dynamic",
                          "nodeType": "YulIdentifier",
                          "src": "11800:28:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "11800:51:1"
                      }
                    ],
                    "functionName": {
                      "name": "not",
                      "nodeType": "YulIdentifier",
                      "src": "11796:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "11796:56:1"
                  },
                  "variables": [
                    {
                      "name": "mask",
                      "nodeType": "YulTypedName",
                      "src": "11788:4:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "11861:25:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "data",
                        "nodeType": "YulIdentifier",
                        "src": "11875:4:1"
                      },
                      {
                        "name": "mask",
                        "nodeType": "YulIdentifier",
                        "src": "11881:4:1"
                      }
                    ],
                    "functionName": {
                      "name": "and",
                      "nodeType": "YulIdentifier",
                      "src": "11871:3:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "11871:15:1"
                  },
                  "variableNames": [
                    {
                      "name": "result",
                      "nodeType": "YulIdentifier",
                      "src": "11861:6:1"
                    }
                  ]
                }
              ]
            },
            "name": "mask_bytes_dynamic",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "data",
                "nodeType": "YulTypedName",
                "src": "11751:4:1",
                "type": ""
              },
              {
                "name": "bytes",
                "nodeType": "YulTypedName",
                "src": "11757:5:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "result",
                "nodeType": "YulTypedName",
                "src": "11767:6:1",
                "type": ""
              }
            ],
            "src": "11723:169:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "11978:214:1",
              "statements": [
                {
                  "nodeType": "YulAssignment",
                  "src": "12111:37:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "data",
                        "nodeType": "YulIdentifier",
                        "src": "12138:4:1"
                      },
                      {
                        "name": "len",
                        "nodeType": "YulIdentifier",
                        "src": "12144:3:1"
                      }
                    ],
                    "functionName": {
                      "name": "mask_bytes_dynamic",
                      "nodeType": "YulIdentifier",
                      "src": "12119:18:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "12119:29:1"
                  },
                  "variableNames": [
                    {
                      "name": "data",
                      "nodeType": "YulIdentifier",
                      "src": "12111:4:1"
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "12157:29:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "data",
                        "nodeType": "YulIdentifier",
                        "src": "12168:4:1"
                      },
                      {
                        "arguments": [
                          {
                            "kind": "number",
                            "nodeType": "YulLiteral",
                            "src": "12178:1:1",
                            "type": "",
                            "value": "2"
                          },
                          {
                            "name": "len",
                            "nodeType": "YulIdentifier",
                            "src": "12181:3:1"
                          }
                        ],
                        "functionName": {
                          "name": "mul",
                          "nodeType": "YulIdentifier",
                          "src": "12174:3:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "12174:11:1"
                      }
                    ],
                    "functionName": {
                      "name": "or",
                      "nodeType": "YulIdentifier",
                      "src": "12165:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "12165:21:1"
                  },
                  "variableNames": [
                    {
                      "name": "used",
                      "nodeType": "YulIdentifier",
                      "src": "12157:4:1"
                    }
                  ]
                }
              ]
            },
            "name": "extract_used_part_and_set_length_of_short_byte_array",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "data",
                "nodeType": "YulTypedName",
                "src": "11959:4:1",
                "type": ""
              },
              {
                "name": "len",
                "nodeType": "YulTypedName",
                "src": "11965:3:1",
                "type": ""
              }
            ],
            "returnVariables": [
              {
                "name": "used",
                "nodeType": "YulTypedName",
                "src": "11973:4:1",
                "type": ""
              }
            ],
            "src": "11897:295:1"
          },
          {
            "body": {
              "nodeType": "YulBlock",
              "src": "12289:1303:1",
              "statements": [
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "12300:51:1",
                  "value": {
                    "arguments": [
                      {
                        "name": "src",
                        "nodeType": "YulIdentifier",
                        "src": "12347:3:1"
                      }
                    ],
                    "functionName": {
                      "name": "array_length_t_string_memory_ptr",
                      "nodeType": "YulIdentifier",
                      "src": "12314:32:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "12314:37:1"
                  },
                  "variables": [
                    {
                      "name": "newLen",
                      "nodeType": "YulTypedName",
                      "src": "12304:6:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "body": {
                    "nodeType": "YulBlock",
                    "src": "12436:22:1",
                    "statements": [
                      {
                        "expression": {
                          "arguments": [],
                          "functionName": {
                            "name": "panic_error_0x41",
                            "nodeType": "YulIdentifier",
                            "src": "12438:16:1"
                          },
                          "nodeType": "YulFunctionCall",
                          "src": "12438:18:1"
                        },
                        "nodeType": "YulExpressionStatement",
                        "src": "12438:18:1"
                      }
                    ]
                  },
                  "condition": {
                    "arguments": [
                      {
                        "name": "newLen",
                        "nodeType": "YulIdentifier",
                        "src": "12408:6:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "12416:18:1",
                        "type": "",
                        "value": "0xffffffffffffffff"
                      }
                    ],
                    "functionName": {
                      "name": "gt",
                      "nodeType": "YulIdentifier",
                      "src": "12405:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "12405:30:1"
                  },
                  "nodeType": "YulIf",
                  "src": "12402:56:1"
                },
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "12468:52:1",
                  "value": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "name": "slot",
                            "nodeType": "YulIdentifier",
                            "src": "12514:4:1"
                          }
                        ],
                        "functionName": {
                          "name": "sload",
                          "nodeType": "YulIdentifier",
                          "src": "12508:5:1"
                        },
                        "nodeType": "YulFunctionCall",
                        "src": "12508:11:1"
                      }
                    ],
                    "functionName": {
                      "name": "extract_byte_array_length",
                      "nodeType": "YulIdentifier",
                      "src": "12482:25:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "12482:38:1"
                  },
                  "variables": [
                    {
                      "name": "oldLen",
                      "nodeType": "YulTypedName",
                      "src": "12472:6:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "expression": {
                    "arguments": [
                      {
                        "name": "slot",
                        "nodeType": "YulIdentifier",
                        "src": "12613:4:1"
                      },
                      {
                        "name": "oldLen",
                        "nodeType": "YulIdentifier",
                        "src": "12619:6:1"
                      },
                      {
                        "name": "newLen",
                        "nodeType": "YulIdentifier",
                        "src": "12627:6:1"
                      }
                    ],
                    "functionName": {
                      "name": "clean_up_bytearray_end_slots_t_string_storage",
                      "nodeType": "YulIdentifier",
                      "src": "12567:45:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "12567:67:1"
                  },
                  "nodeType": "YulExpressionStatement",
                  "src": "12567:67:1"
                },
                {
                  "nodeType": "YulVariableDeclaration",
                  "src": "12644:18:1",
                  "value": {
                    "kind": "number",
                    "nodeType": "YulLiteral",
                    "src": "12661:1:1",
                    "type": "",
                    "value": "0"
                  },
                  "variables": [
                    {
                      "name": "srcOffset",
                      "nodeType": "YulTypedName",
                      "src": "12648:9:1",
                      "type": ""
                    }
                  ]
                },
                {
                  "nodeType": "YulAssignment",
                  "src": "12672:17:1",
                  "value": {
                    "kind": "number",
                    "nodeType": "YulLiteral",
                    "src": "12685:4:1",
                    "type": "",
                    "value": "0x20"
                  },
                  "variableNames": [
                    {
                      "name": "srcOffset",
                      "nodeType": "YulIdentifier",
                      "src": "12672:9:1"
                    }
                  ]
                },
                {
                  "cases": [
                    {
                      "body": {
                        "nodeType": "YulBlock",
                        "src": "12736:611:1",
                        "statements": [
                          {
                            "nodeType": "YulVariableDeclaration",
                            "src": "12750:37:1",
                            "value": {
                              "arguments": [
                                {
                                  "name": "newLen",
                                  "nodeType": "YulIdentifier",
                                  "src": "12769:6:1"
                                },
                                {
                                  "arguments": [
                                    {
                                      "kind": "number",
                                      "nodeType": "YulLiteral",
                                      "src": "12781:4:1",
                                      "type": "",
                                      "value": "0x1f"
                                    }
                                  ],
                                  "functionName": {
                                    "name": "not",
                                    "nodeType": "YulIdentifier",
                                    "src": "12777:3:1"
                                  },
                                  "nodeType": "YulFunctionCall",
                                  "src": "12777:9:1"
                                }
                              ],
                              "functionName": {
                                "name": "and",
                                "nodeType": "YulIdentifier",
                                "src": "12765:3:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "12765:22:1"
                            },
                            "variables": [
                              {
                                "name": "loopEnd",
                                "nodeType": "YulTypedName",
                                "src": "12754:7:1",
                                "type": ""
                              }
                            ]
                          },
                          {
                            "nodeType": "YulVariableDeclaration",
                            "src": "12801:51:1",
                            "value": {
                              "arguments": [
                                {
                                  "name": "slot",
                                  "nodeType": "YulIdentifier",
                                  "src": "12847:4:1"
                                }
                              ],
                              "functionName": {
                                "name": "array_dataslot_t_string_storage",
                                "nodeType": "YulIdentifier",
                                "src": "12815:31:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "12815:37:1"
                            },
                            "variables": [
                              {
                                "name": "dstPtr",
                                "nodeType": "YulTypedName",
                                "src": "12805:6:1",
                                "type": ""
                              }
                            ]
                          },
                          {
                            "nodeType": "YulVariableDeclaration",
                            "src": "12865:10:1",
                            "value": {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "12874:1:1",
                              "type": "",
                              "value": "0"
                            },
                            "variables": [
                              {
                                "name": "i",
                                "nodeType": "YulTypedName",
                                "src": "12869:1:1",
                                "type": ""
                              }
                            ]
                          },
                          {
                            "body": {
                              "nodeType": "YulBlock",
                              "src": "12933:163:1",
                              "statements": [
                                {
                                  "expression": {
                                    "arguments": [
                                      {
                                        "name": "dstPtr",
                                        "nodeType": "YulIdentifier",
                                        "src": "12958:6:1"
                                      },
                                      {
                                        "arguments": [
                                          {
                                            "arguments": [
                                              {
                                                "name": "src",
                                                "nodeType": "YulIdentifier",
                                                "src": "12976:3:1"
                                              },
                                              {
                                                "name": "srcOffset",
                                                "nodeType": "YulIdentifier",
                                                "src": "12981:9:1"
                                              }
                                            ],
                                            "functionName": {
                                              "name": "add",
                                              "nodeType": "YulIdentifier",
                                              "src": "12972:3:1"
                                            },
                                            "nodeType": "YulFunctionCall",
                                            "src": "12972:19:1"
                                          }
                                        ],
                                        "functionName": {
                                          "name": "mload",
                                          "nodeType": "YulIdentifier",
                                          "src": "12966:5:1"
                                        },
                                        "nodeType": "YulFunctionCall",
                                        "src": "12966:26:1"
                                      }
                                    ],
                                    "functionName": {
                                      "name": "sstore",
                                      "nodeType": "YulIdentifier",
                                      "src": "12951:6:1"
                                    },
                                    "nodeType": "YulFunctionCall",
                                    "src": "12951:42:1"
                                  },
                                  "nodeType": "YulExpressionStatement",
                                  "src": "12951:42:1"
                                },
                                {
                                  "nodeType": "YulAssignment",
                                  "src": "13010:24:1",
                                  "value": {
                                    "arguments": [
                                      {
                                        "name": "dstPtr",
                                        "nodeType": "YulIdentifier",
                                        "src": "13024:6:1"
                                      },
                                      {
                                        "kind": "number",
                                        "nodeType": "YulLiteral",
                                        "src": "13032:1:1",
                                        "type": "",
                                        "value": "1"
                                      }
                                    ],
                                    "functionName": {
                                      "name": "add",
                                      "nodeType": "YulIdentifier",
                                      "src": "13020:3:1"
                                    },
                                    "nodeType": "YulFunctionCall",
                                    "src": "13020:14:1"
                                  },
                                  "variableNames": [
                                    {
                                      "name": "dstPtr",
                                      "nodeType": "YulIdentifier",
                                      "src": "13010:6:1"
                                    }
                                  ]
                                },
                                {
                                  "nodeType": "YulAssignment",
                                  "src": "13051:31:1",
                                  "value": {
                                    "arguments": [
                                      {
                                        "name": "srcOffset",
                                        "nodeType": "YulIdentifier",
                                        "src": "13068:9:1"
                                      },
                                      {
                                        "kind": "number",
                                        "nodeType": "YulLiteral",
                                        "src": "13079:2:1",
                                        "type": "",
                                        "value": "32"
                                      }
                                    ],
                                    "functionName": {
                                      "name": "add",
                                      "nodeType": "YulIdentifier",
                                      "src": "13064:3:1"
                                    },
                                    "nodeType": "YulFunctionCall",
                                    "src": "13064:18:1"
                                  },
                                  "variableNames": [
                                    {
                                      "name": "srcOffset",
                                      "nodeType": "YulIdentifier",
                                      "src": "13051:9:1"
                                    }
                                  ]
                                }
                              ]
                            },
                            "condition": {
                              "arguments": [
                                {
                                  "name": "i",
                                  "nodeType": "YulIdentifier",
                                  "src": "12899:1:1"
                                },
                                {
                                  "name": "loopEnd",
                                  "nodeType": "YulIdentifier",
                                  "src": "12902:7:1"
                                }
                              ],
                              "functionName": {
                                "name": "lt",
                                "nodeType": "YulIdentifier",
                                "src": "12896:2:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "12896:14:1"
                            },
                            "nodeType": "YulForLoop",
                            "post": {
                              "nodeType": "YulBlock",
                              "src": "12911:21:1",
                              "statements": [
                                {
                                  "nodeType": "YulAssignment",
                                  "src": "12913:17:1",
                                  "value": {
                                    "arguments": [
                                      {
                                        "name": "i",
                                        "nodeType": "YulIdentifier",
                                        "src": "12922:1:1"
                                      },
                                      {
                                        "kind": "number",
                                        "nodeType": "YulLiteral",
                                        "src": "12925:4:1",
                                        "type": "",
                                        "value": "0x20"
                                      }
                                    ],
                                    "functionName": {
                                      "name": "add",
                                      "nodeType": "YulIdentifier",
                                      "src": "12918:3:1"
                                    },
                                    "nodeType": "YulFunctionCall",
                                    "src": "12918:12:1"
                                  },
                                  "variableNames": [
                                    {
                                      "name": "i",
                                      "nodeType": "YulIdentifier",
                                      "src": "12913:1:1"
                                    }
                                  ]
                                }
                              ]
                            },
                            "pre": {
                              "nodeType": "YulBlock",
                              "src": "12892:3:1",
                              "statements": []
                            },
                            "src": "12888:208:1"
                          },
                          {
                            "body": {
                              "nodeType": "YulBlock",
                              "src": "13132:156:1",
                              "statements": [
                                {
                                  "nodeType": "YulVariableDeclaration",
                                  "src": "13150:43:1",
                                  "value": {
                                    "arguments": [
                                      {
                                        "arguments": [
                                          {
                                            "name": "src",
                                            "nodeType": "YulIdentifier",
                                            "src": "13177:3:1"
                                          },
                                          {
                                            "name": "srcOffset",
                                            "nodeType": "YulIdentifier",
                                            "src": "13182:9:1"
                                          }
                                        ],
                                        "functionName": {
                                          "name": "add",
                                          "nodeType": "YulIdentifier",
                                          "src": "13173:3:1"
                                        },
                                        "nodeType": "YulFunctionCall",
                                        "src": "13173:19:1"
                                      }
                                    ],
                                    "functionName": {
                                      "name": "mload",
                                      "nodeType": "YulIdentifier",
                                      "src": "13167:5:1"
                                    },
                                    "nodeType": "YulFunctionCall",
                                    "src": "13167:26:1"
                                  },
                                  "variables": [
                                    {
                                      "name": "lastValue",
                                      "nodeType": "YulTypedName",
                                      "src": "13154:9:1",
                                      "type": ""
                                    }
                                  ]
                                },
                                {
                                  "expression": {
                                    "arguments": [
                                      {
                                        "name": "dstPtr",
                                        "nodeType": "YulIdentifier",
                                        "src": "13217:6:1"
                                      },
                                      {
                                        "arguments": [
                                          {
                                            "name": "lastValue",
                                            "nodeType": "YulIdentifier",
                                            "src": "13244:9:1"
                                          },
                                          {
                                            "arguments": [
                                              {
                                                "name": "newLen",
                                                "nodeType": "YulIdentifier",
                                                "src": "13259:6:1"
                                              },
                                              {
                                                "kind": "number",
                                                "nodeType": "YulLiteral",
                                                "src": "13267:4:1",
                                                "type": "",
                                                "value": "0x1f"
                                              }
                                            ],
                                            "functionName": {
                                              "name": "and",
                                              "nodeType": "YulIdentifier",
                                              "src": "13255:3:1"
                                            },
                                            "nodeType": "YulFunctionCall",
                                            "src": "13255:17:1"
                                          }
                                        ],
                                        "functionName": {
                                          "name": "mask_bytes_dynamic",
                                          "nodeType": "YulIdentifier",
                                          "src": "13225:18:1"
                                        },
                                        "nodeType": "YulFunctionCall",
                                        "src": "13225:48:1"
                                      }
                                    ],
                                    "functionName": {
                                      "name": "sstore",
                                      "nodeType": "YulIdentifier",
                                      "src": "13210:6:1"
                                    },
                                    "nodeType": "YulFunctionCall",
                                    "src": "13210:64:1"
                                  },
                                  "nodeType": "YulExpressionStatement",
                                  "src": "13210:64:1"
                                }
                              ]
                            },
                            "condition": {
                              "arguments": [
                                {
                                  "name": "loopEnd",
                                  "nodeType": "YulIdentifier",
                                  "src": "13115:7:1"
                                },
                                {
                                  "name": "newLen",
                                  "nodeType": "YulIdentifier",
                                  "src": "13124:6:1"
                                }
                              ],
                              "functionName": {
                                "name": "lt",
                                "nodeType": "YulIdentifier",
                                "src": "13112:2:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "13112:19:1"
                            },
                            "nodeType": "YulIf",
                            "src": "13109:179:1"
                          },
                          {
                            "expression": {
                              "arguments": [
                                {
                                  "name": "slot",
                                  "nodeType": "YulIdentifier",
                                  "src": "13308:4:1"
                                },
                                {
                                  "arguments": [
                                    {
                                      "arguments": [
                                        {
                                          "name": "newLen",
                                          "nodeType": "YulIdentifier",
                                          "src": "13322:6:1"
                                        },
                                        {
                                          "kind": "number",
                                          "nodeType": "YulLiteral",
                                          "src": "13330:1:1",
                                          "type": "",
                                          "value": "2"
                                        }
                                      ],
                                      "functionName": {
                                        "name": "mul",
                                        "nodeType": "YulIdentifier",
                                        "src": "13318:3:1"
                                      },
                                      "nodeType": "YulFunctionCall",
                                      "src": "13318:14:1"
                                    },
                                    {
                                      "kind": "number",
                                      "nodeType": "YulLiteral",
                                      "src": "13334:1:1",
                                      "type": "",
                                      "value": "1"
                                    }
                                  ],
                                  "functionName": {
                                    "name": "add",
                                    "nodeType": "YulIdentifier",
                                    "src": "13314:3:1"
                                  },
                                  "nodeType": "YulFunctionCall",
                                  "src": "13314:22:1"
                                }
                              ],
                              "functionName": {
                                "name": "sstore",
                                "nodeType": "YulIdentifier",
                                "src": "13301:6:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "13301:36:1"
                            },
                            "nodeType": "YulExpressionStatement",
                            "src": "13301:36:1"
                          }
                        ]
                      },
                      "nodeType": "YulCase",
                      "src": "12729:618:1",
                      "value": {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "12734:1:1",
                        "type": "",
                        "value": "1"
                      }
                    },
                    {
                      "body": {
                        "nodeType": "YulBlock",
                        "src": "13364:222:1",
                        "statements": [
                          {
                            "nodeType": "YulVariableDeclaration",
                            "src": "13378:14:1",
                            "value": {
                              "kind": "number",
                              "nodeType": "YulLiteral",
                              "src": "13391:1:1",
                              "type": "",
                              "value": "0"
                            },
                            "variables": [
                              {
                                "name": "value",
                                "nodeType": "YulTypedName",
                                "src": "13382:5:1",
                                "type": ""
                              }
                            ]
                          },
                          {
                            "body": {
                              "nodeType": "YulBlock",
                              "src": "13415:67:1",
                              "statements": [
                                {
                                  "nodeType": "YulAssignment",
                                  "src": "13433:35:1",
                                  "value": {
                                    "arguments": [
                                      {
                                        "arguments": [
                                          {
                                            "name": "src",
                                            "nodeType": "YulIdentifier",
                                            "src": "13452:3:1"
                                          },
                                          {
                                            "name": "srcOffset",
                                            "nodeType": "YulIdentifier",
                                            "src": "13457:9:1"
                                          }
                                        ],
                                        "functionName": {
                                          "name": "add",
                                          "nodeType": "YulIdentifier",
                                          "src": "13448:3:1"
                                        },
                                        "nodeType": "YulFunctionCall",
                                        "src": "13448:19:1"
                                      }
                                    ],
                                    "functionName": {
                                      "name": "mload",
                                      "nodeType": "YulIdentifier",
                                      "src": "13442:5:1"
                                    },
                                    "nodeType": "YulFunctionCall",
                                    "src": "13442:26:1"
                                  },
                                  "variableNames": [
                                    {
                                      "name": "value",
                                      "nodeType": "YulIdentifier",
                                      "src": "13433:5:1"
                                    }
                                  ]
                                }
                              ]
                            },
                            "condition": {
                              "name": "newLen",
                              "nodeType": "YulIdentifier",
                              "src": "13408:6:1"
                            },
                            "nodeType": "YulIf",
                            "src": "13405:77:1"
                          },
                          {
                            "expression": {
                              "arguments": [
                                {
                                  "name": "slot",
                                  "nodeType": "YulIdentifier",
                                  "src": "13502:4:1"
                                },
                                {
                                  "arguments": [
                                    {
                                      "name": "value",
                                      "nodeType": "YulIdentifier",
                                      "src": "13561:5:1"
                                    },
                                    {
                                      "name": "newLen",
                                      "nodeType": "YulIdentifier",
                                      "src": "13568:6:1"
                                    }
                                  ],
                                  "functionName": {
                                    "name": "extract_used_part_and_set_length_of_short_byte_array",
                                    "nodeType": "YulIdentifier",
                                    "src": "13508:52:1"
                                  },
                                  "nodeType": "YulFunctionCall",
                                  "src": "13508:67:1"
                                }
                              ],
                              "functionName": {
                                "name": "sstore",
                                "nodeType": "YulIdentifier",
                                "src": "13495:6:1"
                              },
                              "nodeType": "YulFunctionCall",
                              "src": "13495:81:1"
                            },
                            "nodeType": "YulExpressionStatement",
                            "src": "13495:81:1"
                          }
                        ]
                      },
                      "nodeType": "YulCase",
                      "src": "13356:230:1",
                      "value": "default"
                    }
                  ],
                  "expression": {
                    "arguments": [
                      {
                        "name": "newLen",
                        "nodeType": "YulIdentifier",
                        "src": "12709:6:1"
                      },
                      {
                        "kind": "number",
                        "nodeType": "YulLiteral",
                        "src": "12717:2:1",
                        "type": "",
                        "value": "31"
                      }
                    ],
                    "functionName": {
                      "name": "gt",
                      "nodeType": "YulIdentifier",
                      "src": "12706:2:1"
                    },
                    "nodeType": "YulFunctionCall",
                    "src": "12706:14:1"
                  },
                  "nodeType": "YulSwitch",
                  "src": "12699:887:1"
                }
              ]
            },
            "name": "copy_byte_array_to_storage_from_t_string_memory_ptr_to_t_string_storage",
            "nodeType": "YulFunctionDefinition",
            "parameters": [
              {
                "name": "slot",
                "nodeType": "YulTypedName",
                "src": "12278:4:1",
                "type": ""
              },
              {
                "name": "src",
                "nodeType": "YulTypedName",
                "src": "12284:3:1",
                "type": ""
              }
            ],
            "src": "12197:1395:1"
          }
        ]
      },
      "contents": "{\n\n    function allocate_unbounded() -> memPtr {\n        memPtr := mload(64)\n    }\n\n    function revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() {\n        revert(0, 0)\n    }\n\n    function revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() {\n        revert(0, 0)\n    }\n\n    function cleanup_t_uint256(value) -> cleaned {\n        cleaned := value\n    }\n\n    function validator_revert_t_uint256(value) {\n        if iszero(eq(value, cleanup_t_uint256(value))) { revert(0, 0) }\n    }\n\n    function abi_decode_t_uint256(offset, end) -> value {\n        value := calldataload(offset)\n        validator_revert_t_uint256(value)\n    }\n\n    function abi_decode_tuple_t_uint256(headStart, dataEnd) -> value0 {\n        if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }\n\n        {\n\n            let offset := 0\n\n            value0 := abi_decode_t_uint256(add(headStart, offset), dataEnd)\n        }\n\n    }\n\n    function array_length_t_string_memory_ptr(value) -> length {\n\n        length := mload(value)\n\n    }\n\n    function array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, length) -> updated_pos {\n        mstore(pos, length)\n        updated_pos := add(pos, 0x20)\n    }\n\n    function copy_memory_to_memory_with_cleanup(src, dst, length) {\n        let i := 0\n        for { } lt(i, length) { i := add(i, 32) }\n        {\n            mstore(add(dst, i), mload(add(src, i)))\n        }\n        mstore(add(dst, length), 0)\n    }\n\n    function round_up_to_mul_of_32(value) -> result {\n        result := and(add(value, 31), not(31))\n    }\n\n    function abi_encode_t_string_memory_ptr_to_t_string_memory_ptr_fromStack(value, pos) -> end {\n        let length := array_length_t_string_memory_ptr(value)\n        pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, length)\n        copy_memory_to_memory_with_cleanup(add(value, 0x20), pos, length)\n        end := add(pos, round_up_to_mul_of_32(length))\n    }\n\n    function cleanup_t_uint16(value) -> cleaned {\n        cleaned := and(value, 0xffff)\n    }\n\n    function abi_encode_t_uint16_to_t_uint16_fromStack(value, pos) {\n        mstore(pos, cleanup_t_uint16(value))\n    }\n\n    function cleanup_t_bytes32(value) -> cleaned {\n        cleaned := value\n    }\n\n    function abi_encode_t_bytes32_to_t_bytes32_fromStack(value, pos) {\n        mstore(pos, cleanup_t_bytes32(value))\n    }\n\n    function abi_encode_tuple_t_string_memory_ptr_t_uint16_t_uint16_t_string_memory_ptr_t_bytes32__to_t_string_memory_ptr_t_uint16_t_uint16_t_string_memory_ptr_t_bytes32__fromStack_reversed(headStart , value4, value3, value2, value1, value0) -> tail {\n        tail := add(headStart, 160)\n\n        mstore(add(headStart, 0), sub(tail, headStart))\n        tail := abi_encode_t_string_memory_ptr_to_t_string_memory_ptr_fromStack(value0,  tail)\n\n        abi_encode_t_uint16_to_t_uint16_fromStack(value1,  add(headStart, 32))\n\n        abi_encode_t_uint16_to_t_uint16_fromStack(value2,  add(headStart, 64))\n\n        mstore(add(headStart, 96), sub(tail, headStart))\n        tail := abi_encode_t_string_memory_ptr_to_t_string_memory_ptr_fromStack(value3,  tail)\n\n        abi_encode_t_bytes32_to_t_bytes32_fromStack(value4,  add(headStart, 128))\n\n    }\n\n    function revert_error_1b9f4a0a5773e33b91aa01db23bf8c55fce1411167c872835e7fa00a4f17d46d() {\n        revert(0, 0)\n    }\n\n    function revert_error_987264b3b1d58a9c7f8255e93e81c77d86d6299019c33110a076957a3e06e2ae() {\n        revert(0, 0)\n    }\n\n    function panic_error_0x41() {\n        mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)\n        mstore(4, 0x41)\n        revert(0, 0x24)\n    }\n\n    function finalize_allocation(memPtr, size) {\n        let newFreePtr := add(memPtr, round_up_to_mul_of_32(size))\n        // protect against overflow\n        if or(gt(newFreePtr, 0xffffffffffffffff), lt(newFreePtr, memPtr)) { panic_error_0x41() }\n        mstore(64, newFreePtr)\n    }\n\n    function allocate_memory(size) -> memPtr {\n        memPtr := allocate_unbounded()\n        finalize_allocation(memPtr, size)\n    }\n\n    function array_allocation_size_t_string_memory_ptr(length) -> size {\n        // Make sure we can allocate memory without overflow\n        if gt(length, 0xffffffffffffffff) { panic_error_0x41() }\n\n        size := round_up_to_mul_of_32(length)\n\n        // add length slot\n        size := add(size, 0x20)\n\n    }\n\n    function copy_calldata_to_memory_with_cleanup(src, dst, length) {\n        calldatacopy(dst, src, length)\n        mstore(add(dst, length), 0)\n    }\n\n    function abi_decode_available_length_t_string_memory_ptr(src, length, end) -> array {\n        array := allocate_memory(array_allocation_size_t_string_memory_ptr(length))\n        mstore(array, length)\n        let dst := add(array, 0x20)\n        if gt(add(src, length), end) { revert_error_987264b3b1d58a9c7f8255e93e81c77d86d6299019c33110a076957a3e06e2ae() }\n        copy_calldata_to_memory_with_cleanup(src, dst, length)\n    }\n\n    // string\n    function abi_decode_t_string_memory_ptr(offset, end) -> array {\n        if iszero(slt(add(offset, 0x1f), end)) { revert_error_1b9f4a0a5773e33b91aa01db23bf8c55fce1411167c872835e7fa00a4f17d46d() }\n        let length := calldataload(offset)\n        array := abi_decode_available_length_t_string_memory_ptr(add(offset, 0x20), length, end)\n    }\n\n    function validator_revert_t_uint16(value) {\n        if iszero(eq(value, cleanup_t_uint16(value))) { revert(0, 0) }\n    }\n\n    function abi_decode_t_uint16(offset, end) -> value {\n        value := calldataload(offset)\n        validator_revert_t_uint16(value)\n    }\n\n    function validator_revert_t_bytes32(value) {\n        if iszero(eq(value, cleanup_t_bytes32(value))) { revert(0, 0) }\n    }\n\n    function abi_decode_t_bytes32(offset, end) -> value {\n        value := calldataload(offset)\n        validator_revert_t_bytes32(value)\n    }\n\n    function abi_decode_tuple_t_string_memory_ptrt_uint16t_uint16t_string_memory_ptrt_bytes32(headStart, dataEnd) -> value0, value1, value2, value3, value4 {\n        if slt(sub(dataEnd, headStart), 160) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }\n\n        {\n\n            let offset := calldataload(add(headStart, 0))\n            if gt(offset, 0xffffffffffffffff) { revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() }\n\n            value0 := abi_decode_t_string_memory_ptr(add(headStart, offset), dataEnd)\n        }\n\n        {\n\n            let offset := 32\n\n            value1 := abi_decode_t_uint16(add(headStart, offset), dataEnd)\n        }\n\n        {\n\n            let offset := 64\n\n            value2 := abi_decode_t_uint16(add(headStart, offset), dataEnd)\n        }\n\n        {\n\n            let offset := calldataload(add(headStart, 96))\n            if gt(offset, 0xffffffffffffffff) { revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() }\n\n            value3 := abi_decode_t_string_memory_ptr(add(headStart, offset), dataEnd)\n        }\n\n        {\n\n            let offset := 128\n\n            value4 := abi_decode_t_bytes32(add(headStart, offset), dataEnd)\n        }\n\n    }\n\n    function abi_encode_t_uint256_to_t_uint256_fromStack(value, pos) {\n        mstore(pos, cleanup_t_uint256(value))\n    }\n\n    function abi_encode_tuple_t_uint256__to_t_uint256__fromStack_reversed(headStart , value0) -> tail {\n        tail := add(headStart, 32)\n\n        abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))\n\n    }\n\n    function store_literal_in_memory_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60(memPtr) {\n\n        mstore(add(memPtr, 0), \"Invalid index\")\n\n    }\n\n    function abi_encode_t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60_to_t_string_memory_ptr_fromStack(pos) -> end {\n        pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 13)\n        store_literal_in_memory_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60(pos)\n        end := add(pos, 32)\n    }\n\n    function abi_encode_tuple_t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60__to_t_string_memory_ptr__fromStack_reversed(headStart ) -> tail {\n        tail := add(headStart, 32)\n\n        mstore(add(headStart, 0), sub(tail, headStart))\n        tail := abi_encode_t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60_to_t_string_memory_ptr_fromStack( tail)\n\n    }\n\n    function panic_error_0x32() {\n        mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)\n        mstore(4, 0x32)\n        revert(0, 0x24)\n    }\n\n    function panic_error_0x22() {\n        mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)\n        mstore(4, 0x22)\n        revert(0, 0x24)\n    }\n\n    function extract_byte_array_length(data) -> length {\n        length := div(data, 2)\n        let outOfPlaceEncoding := and(data, 1)\n        if iszero(outOfPlaceEncoding) {\n            length := and(length, 0x7f)\n        }\n\n        if eq(outOfPlaceEncoding, lt(length, 32)) {\n            panic_error_0x22()\n        }\n    }\n\n    function array_dataslot_t_string_storage(ptr) -> data {\n        data := ptr\n\n        mstore(0, ptr)\n        data := keccak256(0, 0x20)\n\n    }\n\n    function divide_by_32_ceil(value) -> result {\n        result := div(add(value, 31), 32)\n    }\n\n    function shift_left_dynamic(bits, value) -> newValue {\n        newValue :=\n\n        shl(bits, value)\n\n    }\n\n    function update_byte_slice_dynamic32(value, shiftBytes, toInsert) -> result {\n        let shiftBits := mul(shiftBytes, 8)\n        let mask := shift_left_dynamic(shiftBits, 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)\n        toInsert := shift_left_dynamic(shiftBits, toInsert)\n        value := and(value, not(mask))\n        result := or(value, and(toInsert, mask))\n    }\n\n    function identity(value) -> ret {\n        ret := value\n    }\n\n    function convert_t_uint256_to_t_uint256(value) -> converted {\n        converted := cleanup_t_uint256(identity(cleanup_t_uint256(value)))\n    }\n\n    function prepare_store_t_uint256(value) -> ret {\n        ret := value\n    }\n\n    function update_storage_value_t_uint256_to_t_uint256(slot, offset, value_0) {\n        let convertedValue_0 := convert_t_uint256_to_t_uint256(value_0)\n        sstore(slot, update_byte_slice_dynamic32(sload(slot), offset, prepare_store_t_uint256(convertedValue_0)))\n    }\n\n    function zero_value_for_split_t_uint256() -> ret {\n        ret := 0\n    }\n\n    function storage_set_to_zero_t_uint256(slot, offset) {\n        let zero_0 := zero_value_for_split_t_uint256()\n        update_storage_value_t_uint256_to_t_uint256(slot, offset, zero_0)\n    }\n\n    function clear_storage_range_t_bytes1(start, end) {\n        for {} lt(start, end) { start := add(start, 1) }\n        {\n            storage_set_to_zero_t_uint256(start, 0)\n        }\n    }\n\n    function clean_up_bytearray_end_slots_t_string_storage(array, len, startIndex) {\n\n        if gt(len, 31) {\n            let dataArea := array_dataslot_t_string_storage(array)\n            let deleteStart := add(dataArea, divide_by_32_ceil(startIndex))\n            // If we are clearing array to be short byte array, we want to clear only data starting from array data area.\n            if lt(startIndex, 32) { deleteStart := dataArea }\n            clear_storage_range_t_bytes1(deleteStart, add(dataArea, divide_by_32_ceil(len)))\n        }\n\n    }\n\n    function shift_right_unsigned_dynamic(bits, value) -> newValue {\n        newValue :=\n\n        shr(bits, value)\n\n    }\n\n    function mask_bytes_dynamic(data, bytes) -> result {\n        let mask := not(shift_right_unsigned_dynamic(mul(8, bytes), not(0)))\n        result := and(data, mask)\n    }\n    function extract_used_part_and_set_length_of_short_byte_array(data, len) -> used {\n        // we want to save only elements that are part of the array after resizing\n        // others should be set to zero\n        data := mask_bytes_dynamic(data, len)\n        used := or(data, mul(2, len))\n    }\n    function copy_byte_array_to_storage_from_t_string_memory_ptr_to_t_string_storage(slot, src) {\n\n        let newLen := array_length_t_string_memory_ptr(src)\n        // Make sure array length is sane\n        if gt(newLen, 0xffffffffffffffff) { panic_error_0x41() }\n\n        let oldLen := extract_byte_array_length(sload(slot))\n\n        // potentially truncate data\n        clean_up_bytearray_end_slots_t_string_storage(slot, oldLen, newLen)\n\n        let srcOffset := 0\n\n        srcOffset := 0x20\n\n        switch gt(newLen, 31)\n        case 1 {\n            let loopEnd := and(newLen, not(0x1f))\n\n            let dstPtr := array_dataslot_t_string_storage(slot)\n            let i := 0\n            for { } lt(i, loopEnd) { i := add(i, 0x20) } {\n                sstore(dstPtr, mload(add(src, srcOffset)))\n                dstPtr := add(dstPtr, 1)\n                srcOffset := add(srcOffset, 32)\n            }\n            if lt(loopEnd, newLen) {\n                let lastValue := mload(add(src, srcOffset))\n                sstore(dstPtr, mask_bytes_dynamic(lastValue, and(newLen, 0x1f)))\n            }\n            sstore(slot, add(mul(newLen, 2), 1))\n        }\n        default {\n            let value := 0\n            if newLen {\n                value := mload(add(src, srcOffset))\n            }\n            sstore(slot, extract_used_part_and_set_length_of_short_byte_array(value, newLen))\n        }\n    }\n\n}\n",
      "id": 1,
      "language": "Yul",
      "name": "#utility.yul"
    }
  ],
  "sourceMap": "58:1295:0:-:0;;;;;;;;;;;;;;;;;;;",
  "deployedSourceMap": "58:1295:0:-:0;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;787:473;;;;;;;;;;;;;:::i;:::-;;:::i;:::-;;;;;;;;;;;:::i;:::-;;;;;;;;434:347;;;;;;;;;;;;;:::i;:::-;;:::i;:::-;;1266:85;;;:::i;:::-;;;;;;;:::i;:::-;;;;;;;;787:473;848:22;880:18;908:15;933:23;966:16;1015:7;:14;;;;1007:5;:22;999:48;;;;;;;;;;;;:::i;:::-;;;;;;;;;1057:18;1078:7;1086:5;1078:14;;;;;;;;:::i;:::-;;;;;;;;;;;;1057:35;;1123:3;:12;;1149:3;:15;;;;;;;;;;;;1178:3;:12;;;;;;;;;;;;1204:3;:13;;1231:3;:12;;;1102:151;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;787:473;;;;;;;:::o;434:347::-;618:7;631:60;;;;;;;;638:8;631:60;;;;648:11;631:60;;;;;;661:8;631:60;;;;;;671:9;631:60;;;;682:8;631:60;;;618:74;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;707:67;721:8;731:11;744:8;754:9;765:8;707:67;;;;;;;;;;:::i;:::-;;;;;;;;434:347;;;;;:::o;1266:85::-;1307:4;1330:7;:14;;;;1323:21;;1266:85;:::o;7:75:1:-;40:6;73:2;67:9;57:19;;7:75;:::o;88:117::-;197:1;194;187:12;211:117;320:1;317;310:12;334:77;371:7;400:5;389:16;;334:77;;;:::o;417:122::-;490:24;508:5;490:24;:::i;:::-;483:5;480:35;470:63;;529:1;526;519:12;470:63;417:122;:::o;545:139::-;591:5;629:6;616:20;607:29;;645:33;672:5;645:33;:::i;:::-;545:139;;;;:::o;690:329::-;749:6;798:2;786:9;777:7;773:23;769:32;766:119;;;804:79;;:::i;:::-;766:119;924:1;949:53;994:7;985:6;974:9;970:22;949:53;:::i;:::-;939:63;;895:117;690:329;;;;:::o;1025:99::-;1077:6;1111:5;1105:12;1095:22;;1025:99;;;:::o;1130:169::-;1214:11;1248:6;1243:3;1236:19;1288:4;1283:3;1279:14;1264:29;;1130:169;;;;:::o;1305:246::-;1386:1;1396:113;1410:6;1407:1;1404:13;1396:113;;;1495:1;1490:3;1486:11;1480:18;1476:1;1471:3;1467:11;1460:39;1432:2;1429:1;1425:10;1420:15;;1396:113;;;1543:1;1534:6;1529:3;1525:16;1518:27;1367:184;1305:246;;;:::o;1557:102::-;1598:6;1649:2;1645:7;1640:2;1633:5;1629:14;1625:28;1615:38;;1557:102;;;:::o;1665:377::-;1753:3;1781:39;1814:5;1781:39;:::i;:::-;1836:71;1900:6;1895:3;1836:71;:::i;:::-;1829:78;;1916:65;1974:6;1969:3;1962:4;1955:5;1951:16;1916:65;:::i;:::-;2006:29;2028:6;2006:29;:::i;:::-;2001:3;1997:39;1990:46;;1757:285;1665:377;;;;:::o;2048:89::-;2084:7;2124:6;2117:5;2113:18;2102:29;;2048:89;;;:::o;2143:115::-;2228:23;2245:5;2228:23;:::i;:::-;2223:3;2216:36;2143:115;;:::o;2264:77::-;2301:7;2330:5;2319:16;;2264:77;;;:::o;2347:118::-;2434:24;2452:5;2434:24;:::i;:::-;2429:3;2422:37;2347:118;;:::o;2471:838::-;2712:4;2750:3;2739:9;2735:19;2727:27;;2800:9;2794:4;2790:20;2786:1;2775:9;2771:17;2764:47;2828:78;2901:4;2892:6;2828:78;:::i;:::-;2820:86;;2916:70;2982:2;2971:9;2967:18;2958:6;2916:70;:::i;:::-;2996;3062:2;3051:9;3047:18;3038:6;2996:70;:::i;:::-;3113:9;3107:4;3103:20;3098:2;3087:9;3083:18;3076:48;3141:78;3214:4;3205:6;3141:78;:::i;:::-;3133:86;;3229:73;3297:3;3286:9;3282:19;3273:6;3229:73;:::i;:::-;2471:838;;;;;;;;:::o;3315:117::-;3424:1;3421;3414:12;3438:117;3547:1;3544;3537:12;3561:180;3609:77;3606:1;3599:88;3706:4;3703:1;3696:15;3730:4;3727:1;3720:15;3747:281;3830:27;3852:4;3830:27;:::i;:::-;3822:6;3818:40;3960:6;3948:10;3945:22;3924:18;3912:10;3909:34;3906:62;3903:88;;;3971:18;;:::i;:::-;3903:88;4011:10;4007:2;4000:22;3790:238;3747:281;;:::o;4034:129::-;4068:6;4095:20;;:::i;:::-;4085:30;;4124:33;4152:4;4144:6;4124:33;:::i;:::-;4034:129;;;:::o;4169:308::-;4231:4;4321:18;4313:6;4310:30;4307:56;;;4343:18;;:::i;:::-;4307:56;4381:29;4403:6;4381:29;:::i;:::-;4373:37;;4465:4;4459;4455:15;4447:23;;4169:308;;;:::o;4483:146::-;4580:6;4575:3;4570;4557:30;4621:1;4612:6;4607:3;4603:16;4596:27;4483:146;;;:::o;4635:425::-;4713:5;4738:66;4754:49;4796:6;4754:49;:::i;:::-;4738:66;:::i;:::-;4729:75;;4827:6;4820:5;4813:21;4865:4;4858:5;4854:16;4903:3;4894:6;4889:3;4885:16;4882:25;4879:112;;;4910:79;;:::i;:::-;4879:112;5000:54;5047:6;5042:3;5037;5000:54;:::i;:::-;4719:341;4635:425;;;;;:::o;5080:340::-;5136:5;5185:3;5178:4;5170:6;5166:17;5162:27;5152:122;;5193:79;;:::i;:::-;5152:122;5310:6;5297:20;5335:79;5410:3;5402:6;5395:4;5387:6;5383:17;5335:79;:::i;:::-;5326:88;;5142:278;5080:340;;;;:::o;5426:120::-;5498:23;5515:5;5498:23;:::i;:::-;5491:5;5488:34;5478:62;;5536:1;5533;5526:12;5478:62;5426:120;:::o;5552:137::-;5597:5;5635:6;5622:20;5613:29;;5651:32;5677:5;5651:32;:::i;:::-;5552:137;;;;:::o;5695:122::-;5768:24;5786:5;5768:24;:::i;:::-;5761:5;5758:35;5748:63;;5807:1;5804;5797:12;5748:63;5695:122;:::o;5823:139::-;5869:5;5907:6;5894:20;5885:29;;5923:33;5950:5;5923:33;:::i;:::-;5823:139;;;;:::o;5968:1267::-;6081:6;6089;6097;6105;6113;6162:3;6150:9;6141:7;6137:23;6133:33;6130:120;;;6169:79;;:::i;:::-;6130:120;6317:1;6306:9;6302:17;6289:31;6347:18;6339:6;6336:30;6333:117;;;6369:79;;:::i;:::-;6333:117;6474:63;6529:7;6520:6;6509:9;6505:22;6474:63;:::i;:::-;6464:73;;6260:287;6586:2;6612:52;6656:7;6647:6;6636:9;6632:22;6612:52;:::i;:::-;6602:62;;6557:117;6713:2;6739:52;6783:7;6774:6;6763:9;6759:22;6739:52;:::i;:::-;6729:62;;6684:117;6868:2;6857:9;6853:18;6840:32;6899:18;6891:6;6888:30;6885:117;;;6921:79;;:::i;:::-;6885:117;7026:63;7081:7;7072:6;7061:9;7057:22;7026:63;:::i;:::-;7016:73;;6811:288;7138:3;7165:53;7210:7;7201:6;7190:9;7186:22;7165:53;:::i;:::-;7155:63;;7109:119;5968:1267;;;;;;;;:::o;7241:118::-;7328:24;7346:5;7328:24;:::i;:::-;7323:3;7316:37;7241:118;;:::o;7365:222::-;7458:4;7496:2;7485:9;7481:18;7473:26;;7509:71;7577:1;7566:9;7562:17;7553:6;7509:71;:::i;:::-;7365:222;;;;:::o;7593:163::-;7733:15;7729:1;7721:6;7717:14;7710:39;7593:163;:::o;7762:366::-;7904:3;7925:67;7989:2;7984:3;7925:67;:::i;:::-;7918:74;;8001:93;8090:3;8001:93;:::i;:::-;8119:2;8114:3;8110:12;8103:19;;7762:366;;;:::o;8134:419::-;8300:4;8338:2;8327:9;8323:18;8315:26;;8387:9;8381:4;8377:20;8373:1;8362:9;8358:17;8351:47;8415:131;8541:4;8415:131;:::i;:::-;8407:139;;8134:419;;;:::o;8559:180::-;8607:77;8604:1;8597:88;8704:4;8701:1;8694:15;8728:4;8725:1;8718:15;8745:180;8793:77;8790:1;8783:88;8890:4;8887:1;8880:15;8914:4;8911:1;8904:15;8931:320;8975:6;9012:1;9006:4;9002:12;8992:22;;9059:1;9053:4;9049:12;9080:18;9070:81;;9136:4;9128:6;9124:17;9114:27;;9070:81;9198:2;9190:6;9187:14;9167:18;9164:38;9161:84;;9217:18;;:::i;:::-;9161:84;8982:269;8931:320;;;:::o;9257:141::-;9306:4;9329:3;9321:11;;9352:3;9349:1;9342:14;9386:4;9383:1;9373:18;9365:26;;9257:141;;;:::o;9404:93::-;9441:6;9488:2;9483;9476:5;9472:14;9468:23;9458:33;;9404:93;;;:::o;9503:107::-;9547:8;9597:5;9591:4;9587:16;9566:37;;9503:107;;;;:::o;9616:393::-;9685:6;9735:1;9723:10;9719:18;9758:97;9788:66;9777:9;9758:97;:::i;:::-;9876:39;9906:8;9895:9;9876:39;:::i;:::-;9864:51;;9948:4;9944:9;9937:5;9933:21;9924:30;;9997:4;9987:8;9983:19;9976:5;9973:30;9963:40;;9692:317;;9616:393;;;;;:::o;10015:60::-;10043:3;10064:5;10057:12;;10015:60;;;:::o;10081:142::-;10131:9;10164:53;10182:34;10191:24;10209:5;10191:24;:::i;:::-;10182:34;:::i;:::-;10164:53;:::i;:::-;10151:66;;10081:142;;;:::o;10229:75::-;10272:3;10293:5;10286:12;;10229:75;;;:::o;10310:269::-;10420:39;10451:7;10420:39;:::i;:::-;10481:91;10530:41;10554:16;10530:41;:::i;:::-;10522:6;10515:4;10509:11;10481:91;:::i;:::-;10475:4;10468:105;10386:193;10310:269;;;:::o;10585:73::-;10630:3;10585:73;:::o;10664:189::-;10741:32;;:::i;:::-;10782:65;10840:6;10832;10826:4;10782:65;:::i;:::-;10717:136;10664:189;;:::o;10859:186::-;10919:120;10936:3;10929:5;10926:14;10919:120;;;10990:39;11027:1;11020:5;10990:39;:::i;:::-;10963:1;10956:5;10952:13;10943:22;;10919:120;;;10859:186;;:::o;11051:543::-;11152:2;11147:3;11144:11;11141:446;;;11186:38;11218:5;11186:38;:::i;:::-;11270:29;11288:10;11270:29;:::i;:::-;11260:8;11256:44;11453:2;11441:10;11438:18;11435:49;;;11474:8;11459:23;;11435:49;11497:80;11553:22;11571:3;11553:22;:::i;:::-;11543:8;11539:37;11526:11;11497:80;:::i;:::-;11156:431;;11141:446;11051:543;;;:::o;11600:117::-;11654:8;11704:5;11698:4;11694:16;11673:37;;11600:117;;;;:::o;11723:169::-;11767:6;11800:51;11848:1;11844:6;11836:5;11833:1;11829:13;11800:51;:::i;:::-;11796:56;11881:4;11875;11871:15;11861:25;;11774:118;11723:169;;;;:::o;11897:295::-;11973:4;12119:29;12144:3;12138:4;12119:29;:::i;:::-;12111:37;;12181:3;12178:1;12174:11;12168:4;12165:21;12157:29;;11897:295;;;;:::o;12197:1395::-;12314:37;12347:3;12314:37;:::i;:::-;12416:18;12408:6;12405:30;12402:56;;;12438:18;;:::i;:::-;12402:56;12482:38;12514:4;12508:11;12482:38;:::i;:::-;12567:67;12627:6;12619;12613:4;12567:67;:::i;:::-;12661:1;12685:4;12672:17;;12717:2;12709:6;12706:14;12734:1;12729:618;;;;13391:1;13408:6;13405:77;;;13457:9;13452:3;13448:19;13442:26;13433:35;;13405:77;13508:67;13568:6;13561:5;13508:67;:::i;:::-;13502:4;13495:81;13364:222;12699:887;;12729:618;12781:4;12777:9;12769:6;12765:22;12815:37;12847:4;12815:37;:::i;:::-;12874:1;12888:208;12902:7;12899:1;12896:14;12888:208;;;12981:9;12976:3;12972:19;12966:26;12958:6;12951:42;13032:1;13024:6;13020:14;13010:24;;13079:2;13068:9;13064:18;13051:31;;12925:4;12922:1;12918:12;12913:17;;12888:208;;;13124:6;13115:7;13112:19;13109:179;;;13182:9;13177:3;13173:19;13167:26;13225:48;13267:4;13259:6;13255:17;13244:9;13225:48;:::i;:::-;13217:6;13210:64;13132:156;13109:179;13334:1;13330;13322:6;13318:14;13314:22;13308:4;13301:36;12736:611;;;12699:887;;12289:1303;;;12197:1395;;:::o",
  "source": "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.19;\n\ncontract SensorData {\n    struct Record {\n        string sensorId;\n        uint16 temperature;\n        uint16 humidity;\n        string timestamp;\n        bytes32 dataHash;\n    }\n\n    Record[] private records;\n\n    event DataPublished(\n        string sensorId,\n        uint16 temperature,\n        uint16 humidity,\n        string timestamp,\n        bytes32 dataHash\n    );\n\n    function publish(\n        string memory sensorId,\n        uint16 temperature,\n        uint16 humidity,\n        string memory timestamp,\n        bytes32 dataHash\n    ) public {\n        records.push(Record(sensorId, temperature, humidity, timestamp, dataHash));\n        emit DataPublished(sensorId, temperature, humidity, timestamp, dataHash);\n    }\n\n    function getRecord(uint index) public view returns (\n        string memory sensorId,\n        uint16 temperature,\n        uint16 humidity,\n        string memory timestamp,\n        bytes32 dataHash\n    ) {\n        require(index < records.length, \"Invalid index\");\n        Record storage rec = records[index];\n        return (\n            rec.sensorId,\n            rec.temperature,\n            rec.humidity,\n            rec.timestamp,\n            rec.dataHash\n        );\n    }\n\n    function getCount() public view returns (uint) {\n        return records.length;\n    }\n}\n\n",
  "sourcePath": "/home/vboxuser/projek/blockchain/contracts/SensorData.sol",
  "ast": {
    "absolutePath": "project:/contracts/SensorData.sol",
    "exportedSymbols": {
      "SensorData": [
        115
      ]
    },
    "id": 116,
    "license": "MIT",
    "nodeType": "SourceUnit",
    "nodes": [
      {
        "id": 1,
        "literals": [
          "solidity",
          "^",
          "0.8",
          ".19"
        ],
        "nodeType": "PragmaDirective",
        "src": "32:24:0"
      },
      {
        "abstract": false,
        "baseContracts": [],
        "canonicalName": "SensorData",
        "contractDependencies": [],
        "contractKind": "contract",
        "fullyImplemented": true,
        "id": 115,
        "linearizedBaseContracts": [
          115
        ],
        "name": "SensorData",
        "nameLocation": "67:10:0",
        "nodeType": "ContractDefinition",
        "nodes": [
          {
            "canonicalName": "SensorData.Record",
            "id": 12,
            "members": [
              {
                "constant": false,
                "id": 3,
                "mutability": "mutable",
                "name": "sensorId",
                "nameLocation": "115:8:0",
                "nodeType": "VariableDeclaration",
                "scope": 12,
                "src": "108:15:0",
                "stateVariable": false,
                "storageLocation": "default",
                "typeDescriptions": {
                  "typeIdentifier": "t_string_storage_ptr",
                  "typeString": "string"
                },
                "typeName": {
                  "id": 2,
                  "name": "string",
                  "nodeType": "ElementaryTypeName",
                  "src": "108:6:0",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_storage_ptr",
                    "typeString": "string"
                  }
                },
                "visibility": "internal"
              },
              {
                "constant": false,
                "id": 5,
                "mutability": "mutable",
                "name": "temperature",
                "nameLocation": "140:11:0",
                "nodeType": "VariableDeclaration",
                "scope": 12,
                "src": "133:18:0",
                "stateVariable": false,
                "storageLocation": "default",
                "typeDescriptions": {
                  "typeIdentifier": "t_uint16",
                  "typeString": "uint16"
                },
                "typeName": {
                  "id": 4,
                  "name": "uint16",
                  "nodeType": "ElementaryTypeName",
                  "src": "133:6:0",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  }
                },
                "visibility": "internal"
              },
              {
                "constant": false,
                "id": 7,
                "mutability": "mutable",
                "name": "humidity",
                "nameLocation": "168:8:0",
                "nodeType": "VariableDeclaration",
                "scope": 12,
                "src": "161:15:0",
                "stateVariable": false,
                "storageLocation": "default",
                "typeDescriptions": {
                  "typeIdentifier": "t_uint16",
                  "typeString": "uint16"
                },
                "typeName": {
                  "id": 6,
                  "name": "uint16",
                  "nodeType": "ElementaryTypeName",
                  "src": "161:6:0",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  }
                },
                "visibility": "internal"
              },
              {
                "constant": false,
                "id": 9,
                "mutability": "mutable",
                "name": "timestamp",
                "nameLocation": "193:9:0",
                "nodeType": "VariableDeclaration",
                "scope": 12,
                "src": "186:16:0",
                "stateVariable": false,
                "storageLocation": "default",
                "typeDescriptions": {
                  "typeIdentifier": "t_string_storage_ptr",
                  "typeString": "string"
                },
                "typeName": {
                  "id": 8,
                  "name": "string",
                  "nodeType": "ElementaryTypeName",
                  "src": "186:6:0",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_storage_ptr",
                    "typeString": "string"
                  }
                },
                "visibility": "internal"
              },
              {
                "constant": false,
                "id": 11,
                "mutability": "mutable",
                "name": "dataHash",
                "nameLocation": "220:8:0",
                "nodeType": "VariableDeclaration",
                "scope": 12,
                "src": "212:16:0",
                "stateVariable": false,
                "storageLocation": "default",
                "typeDescriptions": {
                  "typeIdentifier": "t_bytes32",
                  "typeString": "bytes32"
                },
                "typeName": {
                  "id": 10,
                  "name": "bytes32",
                  "nodeType": "ElementaryTypeName",
                  "src": "212:7:0",
                  "typeDescriptions": {
                    "typeIdentifier": "t_bytes32",
                    "typeString": "bytes32"
                  }
                },
                "visibility": "internal"
              }
            ],
            "name": "Record",
            "nameLocation": "91:6:0",
            "nodeType": "StructDefinition",
            "scope": 115,
            "src": "84:151:0",
            "visibility": "public"
          },
          {
            "constant": false,
            "id": 16,
            "mutability": "mutable",
            "name": "records",
            "nameLocation": "258:7:0",
            "nodeType": "VariableDeclaration",
            "scope": 115,
            "src": "241:24:0",
            "stateVariable": true,
            "storageLocation": "default",
            "typeDescriptions": {
              "typeIdentifier": "t_array$_t_struct$_Record_$12_storage_$dyn_storage",
              "typeString": "struct SensorData.Record[]"
            },
            "typeName": {
              "baseType": {
                "id": 14,
                "nodeType": "UserDefinedTypeName",
                "pathNode": {
                  "id": 13,
                  "name": "Record",
                  "nameLocations": [
                    "241:6:0"
                  ],
                  "nodeType": "IdentifierPath",
                  "referencedDeclaration": 12,
                  "src": "241:6:0"
                },
                "referencedDeclaration": 12,
                "src": "241:6:0",
                "typeDescriptions": {
                  "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                  "typeString": "struct SensorData.Record"
                }
              },
              "id": 15,
              "nodeType": "ArrayTypeName",
              "src": "241:8:0",
              "typeDescriptions": {
                "typeIdentifier": "t_array$_t_struct$_Record_$12_storage_$dyn_storage_ptr",
                "typeString": "struct SensorData.Record[]"
              }
            },
            "visibility": "private"
          },
          {
            "anonymous": false,
            "eventSelector": "6eb0565dcbe8aa0169af5bd5f4d41002bc8b3990be43b9649df2d48d330bc9c5",
            "id": 28,
            "name": "DataPublished",
            "nameLocation": "278:13:0",
            "nodeType": "EventDefinition",
            "parameters": {
              "id": 27,
              "nodeType": "ParameterList",
              "parameters": [
                {
                  "constant": false,
                  "id": 18,
                  "indexed": false,
                  "mutability": "mutable",
                  "name": "sensorId",
                  "nameLocation": "308:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 28,
                  "src": "301:15:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_memory_ptr",
                    "typeString": "string"
                  },
                  "typeName": {
                    "id": 17,
                    "name": "string",
                    "nodeType": "ElementaryTypeName",
                    "src": "301:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_string_storage_ptr",
                      "typeString": "string"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 20,
                  "indexed": false,
                  "mutability": "mutable",
                  "name": "temperature",
                  "nameLocation": "333:11:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 28,
                  "src": "326:18:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  },
                  "typeName": {
                    "id": 19,
                    "name": "uint16",
                    "nodeType": "ElementaryTypeName",
                    "src": "326:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint16",
                      "typeString": "uint16"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 22,
                  "indexed": false,
                  "mutability": "mutable",
                  "name": "humidity",
                  "nameLocation": "361:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 28,
                  "src": "354:15:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  },
                  "typeName": {
                    "id": 21,
                    "name": "uint16",
                    "nodeType": "ElementaryTypeName",
                    "src": "354:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint16",
                      "typeString": "uint16"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 24,
                  "indexed": false,
                  "mutability": "mutable",
                  "name": "timestamp",
                  "nameLocation": "386:9:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 28,
                  "src": "379:16:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_memory_ptr",
                    "typeString": "string"
                  },
                  "typeName": {
                    "id": 23,
                    "name": "string",
                    "nodeType": "ElementaryTypeName",
                    "src": "379:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_string_storage_ptr",
                      "typeString": "string"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 26,
                  "indexed": false,
                  "mutability": "mutable",
                  "name": "dataHash",
                  "nameLocation": "413:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 28,
                  "src": "405:16:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_bytes32",
                    "typeString": "bytes32"
                  },
                  "typeName": {
                    "id": 25,
                    "name": "bytes32",
                    "nodeType": "ElementaryTypeName",
                    "src": "405:7:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_bytes32",
                      "typeString": "bytes32"
                    }
                  },
                  "visibility": "internal"
                }
              ],
              "src": "291:136:0"
            },
            "src": "272:156:0"
          },
          {
            "body": {
              "id": 61,
              "nodeType": "Block",
              "src": "608:173:0",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "arguments": [
                          {
                            "id": 45,
                            "name": "sensorId",
                            "nodeType": "Identifier",
                            "overloadedDeclarations": [],
                            "referencedDeclaration": 30,
                            "src": "638:8:0",
                            "typeDescriptions": {
                              "typeIdentifier": "t_string_memory_ptr",
                              "typeString": "string memory"
                            }
                          },
                          {
                            "id": 46,
                            "name": "temperature",
                            "nodeType": "Identifier",
                            "overloadedDeclarations": [],
                            "referencedDeclaration": 32,
                            "src": "648:11:0",
                            "typeDescriptions": {
                              "typeIdentifier": "t_uint16",
                              "typeString": "uint16"
                            }
                          },
                          {
                            "id": 47,
                            "name": "humidity",
                            "nodeType": "Identifier",
                            "overloadedDeclarations": [],
                            "referencedDeclaration": 34,
                            "src": "661:8:0",
                            "typeDescriptions": {
                              "typeIdentifier": "t_uint16",
                              "typeString": "uint16"
                            }
                          },
                          {
                            "id": 48,
                            "name": "timestamp",
                            "nodeType": "Identifier",
                            "overloadedDeclarations": [],
                            "referencedDeclaration": 36,
                            "src": "671:9:0",
                            "typeDescriptions": {
                              "typeIdentifier": "t_string_memory_ptr",
                              "typeString": "string memory"
                            }
                          },
                          {
                            "id": 49,
                            "name": "dataHash",
                            "nodeType": "Identifier",
                            "overloadedDeclarations": [],
                            "referencedDeclaration": 38,
                            "src": "682:8:0",
                            "typeDescriptions": {
                              "typeIdentifier": "t_bytes32",
                              "typeString": "bytes32"
                            }
                          }
                        ],
                        "expression": {
                          "argumentTypes": [
                            {
                              "typeIdentifier": "t_string_memory_ptr",
                              "typeString": "string memory"
                            },
                            {
                              "typeIdentifier": "t_uint16",
                              "typeString": "uint16"
                            },
                            {
                              "typeIdentifier": "t_uint16",
                              "typeString": "uint16"
                            },
                            {
                              "typeIdentifier": "t_string_memory_ptr",
                              "typeString": "string memory"
                            },
                            {
                              "typeIdentifier": "t_bytes32",
                              "typeString": "bytes32"
                            }
                          ],
                          "id": 44,
                          "name": "Record",
                          "nodeType": "Identifier",
                          "overloadedDeclarations": [],
                          "referencedDeclaration": 12,
                          "src": "631:6:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_type$_t_struct$_Record_$12_storage_ptr_$",
                            "typeString": "type(struct SensorData.Record storage pointer)"
                          }
                        },
                        "id": 50,
                        "isConstant": false,
                        "isLValue": false,
                        "isPure": false,
                        "kind": "structConstructorCall",
                        "lValueRequested": false,
                        "nameLocations": [],
                        "names": [],
                        "nodeType": "FunctionCall",
                        "src": "631:60:0",
                        "tryCall": false,
                        "typeDescriptions": {
                          "typeIdentifier": "t_struct$_Record_$12_memory_ptr",
                          "typeString": "struct SensorData.Record memory"
                        }
                      }
                    ],
                    "expression": {
                      "argumentTypes": [
                        {
                          "typeIdentifier": "t_struct$_Record_$12_memory_ptr",
                          "typeString": "struct SensorData.Record memory"
                        }
                      ],
                      "expression": {
                        "id": 41,
                        "name": "records",
                        "nodeType": "Identifier",
                        "overloadedDeclarations": [],
                        "referencedDeclaration": 16,
                        "src": "618:7:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_array$_t_struct$_Record_$12_storage_$dyn_storage",
                          "typeString": "struct SensorData.Record storage ref[] storage ref"
                        }
                      },
                      "id": 43,
                      "isConstant": false,
                      "isLValue": false,
                      "isPure": false,
                      "lValueRequested": false,
                      "memberLocation": "626:4:0",
                      "memberName": "push",
                      "nodeType": "MemberAccess",
                      "src": "618:12:0",
                      "typeDescriptions": {
                        "typeIdentifier": "t_function_arraypush_nonpayable$_t_array$_t_struct$_Record_$12_storage_$dyn_storage_ptr_$_t_struct$_Record_$12_storage_$returns$__$attached_to$_t_array$_t_struct$_Record_$12_storage_$dyn_storage_ptr_$",
                        "typeString": "function (struct SensorData.Record storage ref[] storage pointer,struct SensorData.Record storage ref)"
                      }
                    },
                    "id": 51,
                    "isConstant": false,
                    "isLValue": false,
                    "isPure": false,
                    "kind": "functionCall",
                    "lValueRequested": false,
                    "nameLocations": [],
                    "names": [],
                    "nodeType": "FunctionCall",
                    "src": "618:74:0",
                    "tryCall": false,
                    "typeDescriptions": {
                      "typeIdentifier": "t_tuple$__$",
                      "typeString": "tuple()"
                    }
                  },
                  "id": 52,
                  "nodeType": "ExpressionStatement",
                  "src": "618:74:0"
                },
                {
                  "eventCall": {
                    "arguments": [
                      {
                        "id": 54,
                        "name": "sensorId",
                        "nodeType": "Identifier",
                        "overloadedDeclarations": [],
                        "referencedDeclaration": 30,
                        "src": "721:8:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_string_memory_ptr",
                          "typeString": "string memory"
                        }
                      },
                      {
                        "id": 55,
                        "name": "temperature",
                        "nodeType": "Identifier",
                        "overloadedDeclarations": [],
                        "referencedDeclaration": 32,
                        "src": "731:11:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_uint16",
                          "typeString": "uint16"
                        }
                      },
                      {
                        "id": 56,
                        "name": "humidity",
                        "nodeType": "Identifier",
                        "overloadedDeclarations": [],
                        "referencedDeclaration": 34,
                        "src": "744:8:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_uint16",
                          "typeString": "uint16"
                        }
                      },
                      {
                        "id": 57,
                        "name": "timestamp",
                        "nodeType": "Identifier",
                        "overloadedDeclarations": [],
                        "referencedDeclaration": 36,
                        "src": "754:9:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_string_memory_ptr",
                          "typeString": "string memory"
                        }
                      },
                      {
                        "id": 58,
                        "name": "dataHash",
                        "nodeType": "Identifier",
                        "overloadedDeclarations": [],
                        "referencedDeclaration": 38,
                        "src": "765:8:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_bytes32",
                          "typeString": "bytes32"
                        }
                      }
                    ],
                    "expression": {
                      "argumentTypes": [
                        {
                          "typeIdentifier": "t_string_memory_ptr",
                          "typeString": "string memory"
                        },
                        {
                          "typeIdentifier": "t_uint16",
                          "typeString": "uint16"
                        },
                        {
                          "typeIdentifier": "t_uint16",
                          "typeString": "uint16"
                        },
                        {
                          "typeIdentifier": "t_string_memory_ptr",
                          "typeString": "string memory"
                        },
                        {
                          "typeIdentifier": "t_bytes32",
                          "typeString": "bytes32"
                        }
                      ],
                      "id": 53,
                      "name": "DataPublished",
                      "nodeType": "Identifier",
                      "overloadedDeclarations": [],
                      "referencedDeclaration": 28,
                      "src": "707:13:0",
                      "typeDescriptions": {
                        "typeIdentifier": "t_function_event_nonpayable$_t_string_memory_ptr_$_t_uint16_$_t_uint16_$_t_string_memory_ptr_$_t_bytes32_$returns$__$",
                        "typeString": "function (string memory,uint16,uint16,string memory,bytes32)"
                      }
                    },
                    "id": 59,
                    "isConstant": false,
                    "isLValue": false,
                    "isPure": false,
                    "kind": "functionCall",
                    "lValueRequested": false,
                    "nameLocations": [],
                    "names": [],
                    "nodeType": "FunctionCall",
                    "src": "707:67:0",
                    "tryCall": false,
                    "typeDescriptions": {
                      "typeIdentifier": "t_tuple$__$",
                      "typeString": "tuple()"
                    }
                  },
                  "id": 60,
                  "nodeType": "EmitStatement",
                  "src": "702:72:0"
                }
              ]
            },
            "functionSelector": "64dc2c11",
            "id": 62,
            "implemented": true,
            "kind": "function",
            "modifiers": [],
            "name": "publish",
            "nameLocation": "443:7:0",
            "nodeType": "FunctionDefinition",
            "parameters": {
              "id": 39,
              "nodeType": "ParameterList",
              "parameters": [
                {
                  "constant": false,
                  "id": 30,
                  "mutability": "mutable",
                  "name": "sensorId",
                  "nameLocation": "474:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 62,
                  "src": "460:22:0",
                  "stateVariable": false,
                  "storageLocation": "memory",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_memory_ptr",
                    "typeString": "string"
                  },
                  "typeName": {
                    "id": 29,
                    "name": "string",
                    "nodeType": "ElementaryTypeName",
                    "src": "460:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_string_storage_ptr",
                      "typeString": "string"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 32,
                  "mutability": "mutable",
                  "name": "temperature",
                  "nameLocation": "499:11:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 62,
                  "src": "492:18:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  },
                  "typeName": {
                    "id": 31,
                    "name": "uint16",
                    "nodeType": "ElementaryTypeName",
                    "src": "492:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint16",
                      "typeString": "uint16"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 34,
                  "mutability": "mutable",
                  "name": "humidity",
                  "nameLocation": "527:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 62,
                  "src": "520:15:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  },
                  "typeName": {
                    "id": 33,
                    "name": "uint16",
                    "nodeType": "ElementaryTypeName",
                    "src": "520:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint16",
                      "typeString": "uint16"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 36,
                  "mutability": "mutable",
                  "name": "timestamp",
                  "nameLocation": "559:9:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 62,
                  "src": "545:23:0",
                  "stateVariable": false,
                  "storageLocation": "memory",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_memory_ptr",
                    "typeString": "string"
                  },
                  "typeName": {
                    "id": 35,
                    "name": "string",
                    "nodeType": "ElementaryTypeName",
                    "src": "545:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_string_storage_ptr",
                      "typeString": "string"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 38,
                  "mutability": "mutable",
                  "name": "dataHash",
                  "nameLocation": "586:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 62,
                  "src": "578:16:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_bytes32",
                    "typeString": "bytes32"
                  },
                  "typeName": {
                    "id": 37,
                    "name": "bytes32",
                    "nodeType": "ElementaryTypeName",
                    "src": "578:7:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_bytes32",
                      "typeString": "bytes32"
                    }
                  },
                  "visibility": "internal"
                }
              ],
              "src": "450:150:0"
            },
            "returnParameters": {
              "id": 40,
              "nodeType": "ParameterList",
              "parameters": [],
              "src": "608:0:0"
            },
            "scope": 115,
            "src": "434:347:0",
            "stateMutability": "nonpayable",
            "virtual": false,
            "visibility": "public"
          },
          {
            "body": {
              "id": 104,
              "nodeType": "Block",
              "src": "989:271:0",
              "statements": [
                {
                  "expression": {
                    "arguments": [
                      {
                        "commonType": {
                          "typeIdentifier": "t_uint256",
                          "typeString": "uint256"
                        },
                        "id": 81,
                        "isConstant": false,
                        "isLValue": false,
                        "isPure": false,
                        "lValueRequested": false,
                        "leftExpression": {
                          "id": 78,
                          "name": "index",
                          "nodeType": "Identifier",
                          "overloadedDeclarations": [],
                          "referencedDeclaration": 64,
                          "src": "1007:5:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_uint256",
                            "typeString": "uint256"
                          }
                        },
                        "nodeType": "BinaryOperation",
                        "operator": "<",
                        "rightExpression": {
                          "expression": {
                            "id": 79,
                            "name": "records",
                            "nodeType": "Identifier",
                            "overloadedDeclarations": [],
                            "referencedDeclaration": 16,
                            "src": "1015:7:0",
                            "typeDescriptions": {
                              "typeIdentifier": "t_array$_t_struct$_Record_$12_storage_$dyn_storage",
                              "typeString": "struct SensorData.Record storage ref[] storage ref"
                            }
                          },
                          "id": 80,
                          "isConstant": false,
                          "isLValue": false,
                          "isPure": false,
                          "lValueRequested": false,
                          "memberLocation": "1023:6:0",
                          "memberName": "length",
                          "nodeType": "MemberAccess",
                          "src": "1015:14:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_uint256",
                            "typeString": "uint256"
                          }
                        },
                        "src": "1007:22:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_bool",
                          "typeString": "bool"
                        }
                      },
                      {
                        "hexValue": "496e76616c696420696e646578",
                        "id": 82,
                        "isConstant": false,
                        "isLValue": false,
                        "isPure": true,
                        "kind": "string",
                        "lValueRequested": false,
                        "nodeType": "Literal",
                        "src": "1031:15:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60",
                          "typeString": "literal_string \"Invalid index\""
                        },
                        "value": "Invalid index"
                      }
                    ],
                    "expression": {
                      "argumentTypes": [
                        {
                          "typeIdentifier": "t_bool",
                          "typeString": "bool"
                        },
                        {
                          "typeIdentifier": "t_stringliteral_32cc480c4f0e15e5ce7060ec5e004886ed5a15831cba1ff1aa7cb787be55bb60",
                          "typeString": "literal_string \"Invalid index\""
                        }
                      ],
                      "id": 77,
                      "name": "require",
                      "nodeType": "Identifier",
                      "overloadedDeclarations": [
                        4294967278,
                        4294967278
                      ],
                      "referencedDeclaration": 4294967278,
                      "src": "999:7:0",
                      "typeDescriptions": {
                        "typeIdentifier": "t_function_require_pure$_t_bool_$_t_string_memory_ptr_$returns$__$",
                        "typeString": "function (bool,string memory) pure"
                      }
                    },
                    "id": 83,
                    "isConstant": false,
                    "isLValue": false,
                    "isPure": false,
                    "kind": "functionCall",
                    "lValueRequested": false,
                    "nameLocations": [],
                    "names": [],
                    "nodeType": "FunctionCall",
                    "src": "999:48:0",
                    "tryCall": false,
                    "typeDescriptions": {
                      "typeIdentifier": "t_tuple$__$",
                      "typeString": "tuple()"
                    }
                  },
                  "id": 84,
                  "nodeType": "ExpressionStatement",
                  "src": "999:48:0"
                },
                {
                  "assignments": [
                    87
                  ],
                  "declarations": [
                    {
                      "constant": false,
                      "id": 87,
                      "mutability": "mutable",
                      "name": "rec",
                      "nameLocation": "1072:3:0",
                      "nodeType": "VariableDeclaration",
                      "scope": 104,
                      "src": "1057:18:0",
                      "stateVariable": false,
                      "storageLocation": "storage",
                      "typeDescriptions": {
                        "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                        "typeString": "struct SensorData.Record"
                      },
                      "typeName": {
                        "id": 86,
                        "nodeType": "UserDefinedTypeName",
                        "pathNode": {
                          "id": 85,
                          "name": "Record",
                          "nameLocations": [
                            "1057:6:0"
                          ],
                          "nodeType": "IdentifierPath",
                          "referencedDeclaration": 12,
                          "src": "1057:6:0"
                        },
                        "referencedDeclaration": 12,
                        "src": "1057:6:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                          "typeString": "struct SensorData.Record"
                        }
                      },
                      "visibility": "internal"
                    }
                  ],
                  "id": 91,
                  "initialValue": {
                    "baseExpression": {
                      "id": 88,
                      "name": "records",
                      "nodeType": "Identifier",
                      "overloadedDeclarations": [],
                      "referencedDeclaration": 16,
                      "src": "1078:7:0",
                      "typeDescriptions": {
                        "typeIdentifier": "t_array$_t_struct$_Record_$12_storage_$dyn_storage",
                        "typeString": "struct SensorData.Record storage ref[] storage ref"
                      }
                    },
                    "id": 90,
                    "indexExpression": {
                      "id": 89,
                      "name": "index",
                      "nodeType": "Identifier",
                      "overloadedDeclarations": [],
                      "referencedDeclaration": 64,
                      "src": "1086:5:0",
                      "typeDescriptions": {
                        "typeIdentifier": "t_uint256",
                        "typeString": "uint256"
                      }
                    },
                    "isConstant": false,
                    "isLValue": true,
                    "isPure": false,
                    "lValueRequested": false,
                    "nodeType": "IndexAccess",
                    "src": "1078:14:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_struct$_Record_$12_storage",
                      "typeString": "struct SensorData.Record storage ref"
                    }
                  },
                  "nodeType": "VariableDeclarationStatement",
                  "src": "1057:35:0"
                },
                {
                  "expression": {
                    "components": [
                      {
                        "expression": {
                          "id": 92,
                          "name": "rec",
                          "nodeType": "Identifier",
                          "overloadedDeclarations": [],
                          "referencedDeclaration": 87,
                          "src": "1123:3:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                            "typeString": "struct SensorData.Record storage pointer"
                          }
                        },
                        "id": 93,
                        "isConstant": false,
                        "isLValue": true,
                        "isPure": false,
                        "lValueRequested": false,
                        "memberLocation": "1127:8:0",
                        "memberName": "sensorId",
                        "nodeType": "MemberAccess",
                        "referencedDeclaration": 3,
                        "src": "1123:12:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_string_storage",
                          "typeString": "string storage ref"
                        }
                      },
                      {
                        "expression": {
                          "id": 94,
                          "name": "rec",
                          "nodeType": "Identifier",
                          "overloadedDeclarations": [],
                          "referencedDeclaration": 87,
                          "src": "1149:3:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                            "typeString": "struct SensorData.Record storage pointer"
                          }
                        },
                        "id": 95,
                        "isConstant": false,
                        "isLValue": true,
                        "isPure": false,
                        "lValueRequested": false,
                        "memberLocation": "1153:11:0",
                        "memberName": "temperature",
                        "nodeType": "MemberAccess",
                        "referencedDeclaration": 5,
                        "src": "1149:15:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_uint16",
                          "typeString": "uint16"
                        }
                      },
                      {
                        "expression": {
                          "id": 96,
                          "name": "rec",
                          "nodeType": "Identifier",
                          "overloadedDeclarations": [],
                          "referencedDeclaration": 87,
                          "src": "1178:3:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                            "typeString": "struct SensorData.Record storage pointer"
                          }
                        },
                        "id": 97,
                        "isConstant": false,
                        "isLValue": true,
                        "isPure": false,
                        "lValueRequested": false,
                        "memberLocation": "1182:8:0",
                        "memberName": "humidity",
                        "nodeType": "MemberAccess",
                        "referencedDeclaration": 7,
                        "src": "1178:12:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_uint16",
                          "typeString": "uint16"
                        }
                      },
                      {
                        "expression": {
                          "id": 98,
                          "name": "rec",
                          "nodeType": "Identifier",
                          "overloadedDeclarations": [],
                          "referencedDeclaration": 87,
                          "src": "1204:3:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                            "typeString": "struct SensorData.Record storage pointer"
                          }
                        },
                        "id": 99,
                        "isConstant": false,
                        "isLValue": true,
                        "isPure": false,
                        "lValueRequested": false,
                        "memberLocation": "1208:9:0",
                        "memberName": "timestamp",
                        "nodeType": "MemberAccess",
                        "referencedDeclaration": 9,
                        "src": "1204:13:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_string_storage",
                          "typeString": "string storage ref"
                        }
                      },
                      {
                        "expression": {
                          "id": 100,
                          "name": "rec",
                          "nodeType": "Identifier",
                          "overloadedDeclarations": [],
                          "referencedDeclaration": 87,
                          "src": "1231:3:0",
                          "typeDescriptions": {
                            "typeIdentifier": "t_struct$_Record_$12_storage_ptr",
                            "typeString": "struct SensorData.Record storage pointer"
                          }
                        },
                        "id": 101,
                        "isConstant": false,
                        "isLValue": true,
                        "isPure": false,
                        "lValueRequested": false,
                        "memberLocation": "1235:8:0",
                        "memberName": "dataHash",
                        "nodeType": "MemberAccess",
                        "referencedDeclaration": 11,
                        "src": "1231:12:0",
                        "typeDescriptions": {
                          "typeIdentifier": "t_bytes32",
                          "typeString": "bytes32"
                        }
                      }
                    ],
                    "id": 102,
                    "isConstant": false,
                    "isInlineArray": false,
                    "isLValue": false,
                    "isPure": false,
                    "lValueRequested": false,
                    "nodeType": "TupleExpression",
                    "src": "1109:144:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_tuple$_t_string_storage_$_t_uint16_$_t_uint16_$_t_string_storage_$_t_bytes32_$",
                      "typeString": "tuple(string storage ref,uint16,uint16,string storage ref,bytes32)"
                    }
                  },
                  "functionReturnParameters": 76,
                  "id": 103,
                  "nodeType": "Return",
                  "src": "1102:151:0"
                }
              ]
            },
            "functionSelector": "03e9e609",
            "id": 105,
            "implemented": true,
            "kind": "function",
            "modifiers": [],
            "name": "getRecord",
            "nameLocation": "796:9:0",
            "nodeType": "FunctionDefinition",
            "parameters": {
              "id": 65,
              "nodeType": "ParameterList",
              "parameters": [
                {
                  "constant": false,
                  "id": 64,
                  "mutability": "mutable",
                  "name": "index",
                  "nameLocation": "811:5:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 105,
                  "src": "806:10:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint256",
                    "typeString": "uint256"
                  },
                  "typeName": {
                    "id": 63,
                    "name": "uint",
                    "nodeType": "ElementaryTypeName",
                    "src": "806:4:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint256",
                      "typeString": "uint256"
                    }
                  },
                  "visibility": "internal"
                }
              ],
              "src": "805:12:0"
            },
            "returnParameters": {
              "id": 76,
              "nodeType": "ParameterList",
              "parameters": [
                {
                  "constant": false,
                  "id": 67,
                  "mutability": "mutable",
                  "name": "sensorId",
                  "nameLocation": "862:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 105,
                  "src": "848:22:0",
                  "stateVariable": false,
                  "storageLocation": "memory",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_memory_ptr",
                    "typeString": "string"
                  },
                  "typeName": {
                    "id": 66,
                    "name": "string",
                    "nodeType": "ElementaryTypeName",
                    "src": "848:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_string_storage_ptr",
                      "typeString": "string"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 69,
                  "mutability": "mutable",
                  "name": "temperature",
                  "nameLocation": "887:11:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 105,
                  "src": "880:18:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  },
                  "typeName": {
                    "id": 68,
                    "name": "uint16",
                    "nodeType": "ElementaryTypeName",
                    "src": "880:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint16",
                      "typeString": "uint16"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 71,
                  "mutability": "mutable",
                  "name": "humidity",
                  "nameLocation": "915:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 105,
                  "src": "908:15:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint16",
                    "typeString": "uint16"
                  },
                  "typeName": {
                    "id": 70,
                    "name": "uint16",
                    "nodeType": "ElementaryTypeName",
                    "src": "908:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint16",
                      "typeString": "uint16"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 73,
                  "mutability": "mutable",
                  "name": "timestamp",
                  "nameLocation": "947:9:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 105,
                  "src": "933:23:0",
                  "stateVariable": false,
                  "storageLocation": "memory",
                  "typeDescriptions": {
                    "typeIdentifier": "t_string_memory_ptr",
                    "typeString": "string"
                  },
                  "typeName": {
                    "id": 72,
                    "name": "string",
                    "nodeType": "ElementaryTypeName",
                    "src": "933:6:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_string_storage_ptr",
                      "typeString": "string"
                    }
                  },
                  "visibility": "internal"
                },
                {
                  "constant": false,
                  "id": 75,
                  "mutability": "mutable",
                  "name": "dataHash",
                  "nameLocation": "974:8:0",
                  "nodeType": "VariableDeclaration",
                  "scope": 105,
                  "src": "966:16:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_bytes32",
                    "typeString": "bytes32"
                  },
                  "typeName": {
                    "id": 74,
                    "name": "bytes32",
                    "nodeType": "ElementaryTypeName",
                    "src": "966:7:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_bytes32",
                      "typeString": "bytes32"
                    }
                  },
                  "visibility": "internal"
                }
              ],
              "src": "838:150:0"
            },
            "scope": 115,
            "src": "787:473:0",
            "stateMutability": "view",
            "virtual": false,
            "visibility": "public"
          },
          {
            "body": {
              "id": 113,
              "nodeType": "Block",
              "src": "1313:38:0",
              "statements": [
                {
                  "expression": {
                    "expression": {
                      "id": 110,
                      "name": "records",
                      "nodeType": "Identifier",
                      "overloadedDeclarations": [],
                      "referencedDeclaration": 16,
                      "src": "1330:7:0",
                      "typeDescriptions": {
                        "typeIdentifier": "t_array$_t_struct$_Record_$12_storage_$dyn_storage",
                        "typeString": "struct SensorData.Record storage ref[] storage ref"
                      }
                    },
                    "id": 111,
                    "isConstant": false,
                    "isLValue": false,
                    "isPure": false,
                    "lValueRequested": false,
                    "memberLocation": "1338:6:0",
                    "memberName": "length",
                    "nodeType": "MemberAccess",
                    "src": "1330:14:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint256",
                      "typeString": "uint256"
                    }
                  },
                  "functionReturnParameters": 109,
                  "id": 112,
                  "nodeType": "Return",
                  "src": "1323:21:0"
                }
              ]
            },
            "functionSelector": "a87d942c",
            "id": 114,
            "implemented": true,
            "kind": "function",
            "modifiers": [],
            "name": "getCount",
            "nameLocation": "1275:8:0",
            "nodeType": "FunctionDefinition",
            "parameters": {
              "id": 106,
              "nodeType": "ParameterList",
              "parameters": [],
              "src": "1283:2:0"
            },
            "returnParameters": {
              "id": 109,
              "nodeType": "ParameterList",
              "parameters": [
                {
                  "constant": false,
                  "id": 108,
                  "mutability": "mutable",
                  "name": "",
                  "nameLocation": "-1:-1:-1",
                  "nodeType": "VariableDeclaration",
                  "scope": 114,
                  "src": "1307:4:0",
                  "stateVariable": false,
                  "storageLocation": "default",
                  "typeDescriptions": {
                    "typeIdentifier": "t_uint256",
                    "typeString": "uint256"
                  },
                  "typeName": {
                    "id": 107,
                    "name": "uint",
                    "nodeType": "ElementaryTypeName",
                    "src": "1307:4:0",
                    "typeDescriptions": {
                      "typeIdentifier": "t_uint256",
                      "typeString": "uint256"
                    }
                  },
                  "visibility": "internal"
                }
              ],
              "src": "1306:6:0"
            },
            "scope": 115,
            "src": "1266:85:0",
            "stateMutability": "view",
            "virtual": false,
            "visibility": "public"
          }
        ],
        "scope": 116,
        "src": "58:1295:0",
        "usedErrors": []
      }
    ],
    "src": "32:1323:0"
  },
  "compiler": {
    "name": "solc",
    "version": "0.8.19+commit.7dd6d404.Emscripten.clang"
  },
  "networks": {
    "1337": {
      "events": {},
      "links": {},
      "address": "0xD5784F849D4a718a66e51Cd9908087f653578C25",
      "transactionHash": "0x7beac1459b69a50b7bd900f02c2f797a82ac95ee8e8981a688c0db899f934e49"
    }
  },
  "schemaVersion": "3.4.16",
  "updatedAt": "2025-06-22T21:42:03.551Z",
  "networkType": "ethereum",
  "devdoc": {
    "kind": "dev",
    "methods": {},
    "version": 1
  },
  "userdoc": {
    "kind": "user",
    "methods": {},
    "version": 1
  }
}
