# Axxon-Backend
Backend migration to rust introducing python microservice for AI features

# Axxon Backend Setup

This guide walks you through setting up the backend project for **Axxon** on a Linux machine. It ensures a stable Rust environment, required dependencies, and OpenSSL configuration.

---

## Prerequisites

- Linux system (Ubuntu/Debian recommended)
- `curl` installed
- `git` installed
- `build-essential` for compiling native dependencies

---

## 1. Install Rust

Install Rust via `rustup` (recommended):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Setup System & Dev Dependencies

```bash
sudo apt update
sudo apt install -y pkg-config libssl-dev build-essential
```

```bash 

```

## 3. Clean and build

```bash
cargo clean 
rm -f Cargo.lock
cargo build
```

## 4. Run Project

```bash
cargo run
```