# Mural Chain

Mural Chain showcases the implementation of a simple cryptocurrency wallet using Rust on the Ethereum network. I have developed an affinity for Rust and have been experimenting with it as a problem-solving tool.

The wallet can generate a secret key, a public key, and a public address. Future enhancements will include features such as seed phrase creation and security measures related to DeFi concepts. Additionally, the wallet will display the balance of the associated address, and demonstrate the process of sending Ether (ETH) from the wallet to a different address.

## Potential Implementation

The goal is to utilize this wallet as a back-end component for a DeFi mobile app. The wallet's capabilities include generating a secret key, a public key, and a public address, as well as showing the balance of the wallet's address. The wallet also provides an example of sending ETH to another address.

## Getting Started

To run Mural Chain, a `.env` file is required containing an endpoint for connecting to the Ethereum network via WebSockets. For example:

```
INFURA_RINKEBY_WS=wss://rinkeby.infura.io/ws/v3/08xxxxxxxxx
```

The `INFURA_RINKEBY_WS` value is an endpoint address from [Infura.io](https://infura.io), but any valid Ethereum network WebSocket endpoint can be used. For this demonstration, we will use Infura.io to connect to an Ethereum node endpoint.

For more information, please visit the [Mural Chain GitHub repository](https://github.com/muralcode/mural_chain).
