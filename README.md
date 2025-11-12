## Fiox

A blazing fast, minimal file conversion CLI tool written in **Rust**.
Supports **TXT**, **CSV**, **JSON**, and **TOML** formats.

---

### Features

- Convert between TXT, CSV, JSON, TOML and more as I add them!
- Ultra-fast thanks to Rust.
- Simple command-line interface.
- Minimal, descriptive, beautifully colored logs for easier usage.
- Clean, easy to understand repo structure and code.
- Active maintainer, frequent updates.

---

### Installation

Clone the repository and build:

```bash
git clone https://github.com/tahamahmoud7097-wq/fiox.git
cd fiox
cargo build --release
# optional: move binary to ~/.local/bin for convenience
mv ~/fiox/target/release/fiox ~/.local/bin/fiox
```

**Note**: This installation is temporary until I publish as a binary crate on **crates.io**.

---

### Usage

```bash
fiox <input_file> <output_file>
```

---

### Plans

- Modularize readers and writers.
- Add compatibility for more file formats like YAML and XML.
- Add parallelism using **rayon**.
- Optimize performance for writing into files with `std::io::BufWriter`.
- Publish as a binary crate on **crates.io** (crate you can install by running `cargo install fiox`)
