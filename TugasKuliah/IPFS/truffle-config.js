module.exports = {
  networks: {
    development: {
      host: "127.0.0.1",     // Ganache local
      port: 7545,            // Port Ganache default
      network_id: "*"        // Match any network id
    }
  },

  compilers: {
    solc: {
      version: "0.8.19"
    }
  }
};
