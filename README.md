# Central Limit Order Book - Capstone Project of Q4 Turbin3

A minimal CLOB built with Anchor for Solana.
This repository contains smart contract code, tests, and tooling with the AMM. 🚀

## Features 📊
- On-chain order book with bids & asks 📈📉
- Limit & market order support 🎯
- Price-time priority matching engine ⚡
- Order placement & cancellation ✍️
- Partial & full trade fill logic 🔄
- Built-in fee + rebate handling 💰
- Unit & integration tests ✅
- Compatible with Anchor & Solana runtime 🦀

## Arc Diagram of CLOB 
[📄 View the Protocol Design PDF](./Assignment_3:_Architecture_Design.pdf)

## Quick Start 🚦

Prerequisites:
- Rust toolchain (stable) 🦀
- Solana CLI (recommended latest) ☀️
- Anchor CLI ⚓
- Node.js & npm (for frontend/tests if present) 🧩

Build the programs:
```bash
anchor build
```

Run tests (local validator):
```bash
npm install
anchor test
```

## Project Layout 📁
- programs/ — Anchor smart contract
- programs/rapid-flow/src/instructions - All the Instructions
- programs/rapid-flow/src/state - Account states
- tests/ — Anchor tests & integration tests

## Contributors
- [@bytehash69](https://github.com/bytehash69)
- [@Vdkk07](https://github.com/Vdkk07)

## License 📜
This project is licensed under the MIT License. See LICENSE for details.
