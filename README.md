# File Converter CLI

A blazing fast, minimal file converter written in **Rust**.
Supports **TXT**, **CSV**, **JSON**, and **TOML** formats.

---

## Features
- Convert between TXT, CSV, JSON, and TOML
- Ultra-fast thanks to Rust
- Simple command-line interface
- Error-safe: panics with clear messages if input/output types are invalid

## Installation

Clone the repository and build:

```bash
git clone https://github.com/YOUR_USERNAME/File-converter-CLI.git
cd File-converter-CLI
cargo build --release
```

## Usage

```bash
./file-converter-cli <input_file> <output_file>
```

## Conversions

- TXT => CSV / JSON / TOML
- JSON => TOML
- TOML => JSON
- CSV => TXT

# Note

I'll still add more conversions and supported file formats but this is just an MVP built in one day which is why there's only two commits.
