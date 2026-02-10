# ⚓ AnchorBox

A simple learning-focused collection of **Solana Anchor programs** written in **Rust**, created to understand core on-chain concepts step by step.

This repository is meant for **practice, experimentation, and learning Anchor** by building small but practical programs.

---

## 📌 What is AnchorBox?

**AnchorBox** is a personal collection of mini Anchor projects that demonstrate common Solana smart contract patterns such as counters, to-do lists, basic banking logic, and calculators.

If you are learning:

* Solana development
* Anchor framework
* Rust for blockchain

this repo is a good hands-on reference.

---

## 📦 Programs Included

| Program       | Description                             |
| ------------- | --------------------------------------- |
| `hello_world` | Basic Anchor program structure          |
| `counter`     | Simple counter (initialize & increment) |
| `todo`        | On-chain to-do list example             |
| `bank`        | Simple bank logic (deposit & withdraw)  |
| `calc`        | On-chain calculator example             |
| `tests`       | Anchor integration tests (TypeScript)   |

Each folder is an independent Anchor program showcasing a specific concept.

---

## 🛠 Tech Stack

* **Rust** – Smart contract language
* **Anchor** – Solana framework
* **Solana CLI** – Local validator & deployment
* **TypeScript** – Testing & client interaction

---

## 🚀 Setup & Installation

### Prerequisites

Make sure you have the following installed:

* Rust
* Solana CLI
* Anchor CLI

### Install Anchor

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

---

## 📁 Project Structure

```
AnchorBox/
├── hello_world/
├── counter/
├── todo/
├── bank/
├── calc/
├── tests/
├── Anchor.toml
└── Cargo.toml
```

---

## 🧪 Build & Test

Build all programs:

```bash
anchor build
```

Run tests on local validator:

```bash
anchor test
```

---

## 🎯 What You Will Learn

* Anchor project structure
* Account initialization & state management
* Writing Solana programs in Rust
* Testing smart contracts using Anchor
* Common reusable on-chain patterns

---

## 📚 Helpful Resources

* Anchor Docs: [https://www.anchor-lang.com/docs](https://www.anchor-lang.com/docs)
* Solana Developers: [https://solana.com/developers](https://solana.com/developers)
* Rust Book: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)

---

## 📜 License

MIT License

---

⭐ If this repo helped you learn Anchor, consider giving it a star!
