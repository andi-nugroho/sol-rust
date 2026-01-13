# 🦀 sol-rust

A lightweight Rust-based HTTP server built with [Poem](https://github.com/poem-web/poem) to interact with the Solana blockchain.

Currently supports:
- ✅ Get wallet balance
- 🚰 Request SOL airdrop (devnet)

> 🔗 Live API: [https://sol-rust.onrender.com](https://sol-rust.onrender.com)

## ✨ Features

- Built in Rust using the `Poem` web framework
- Connects with the Solana Devnet using `solana-client`
- JSON API with clear input/output
- Deployed and ready for testing

## 📦 Endpoints

### `GET /get_balance`

Returns the balance (in SOL) of a given wallet address.

**Request**

```json
{
  "wallet": "YourWalletPublicKey"
}
```

**Example cURL**

```bash
curl -X GET 'https://sol-rust.onrender.com/get_balance' \
  -H 'Content-Type: application/json' \
  -d '{ "wallet": "YourWalletPublicKey" }'
```

---

### `GET /get_airdrop`

Requests SOL airdrop (devnet only) for the given wallet.

**Request**

```json
{
  "wallet": "WalletPublicKey",
  "sol": 1
}
```

**Example cURL**

```bash
curl -X GET 'https://sol-rust.onrender.com/get_airdrop' \
  -H 'Content-Type: application/json' \
  -d '{ "wallet": "CqjcHGE7g9ngSUvTupqdXoWnfo8jcAgZPqjNr6qZREEU", "sol": 1 }'
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (for local development)

### Run Locally

```bash
git clone https://github.com/prince981620/sol-rust
cd sol-rust
cargo run
```

The server will start on `127.0.0.1:3000` by default.

---

## 🧪 Test It

You can also import the endpoints into **Postman** using raw JSON or cURL.

Try it live:  
[https://sol-rust.onrender.com](https://sol-rust.onrender.com)

---

## 💡 Use Cases

- Quick Solana devnet balance checks
- Easy SOL airdrop for development/testing
- Lightweight backend for Solana apps

---

## 🤝 Contributing

Feel free to open issues or PRs to add more endpoints (e.g., transaction status, token balances, etc.).

---

## 📜 License

MIT © [Prince Yadav](https://github.com/prince981620)

---

## 🙌 Acknowledgments

Thanks to:
- @SuperteamIN
- @100xDevs
- @solanaturbine  
for the motivation and support.
