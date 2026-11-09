# ZW-CLI: Markdown to HTML Converter 🦀

A high-performance CLI tool built with **Rust** and `pulldown-cmark` designed to convert Markdown (`.md`) files into standalone, well-structured HTML documents.

## 🚀 Features
- **Fast & Memory-Safe:** Leverages Rust's zero-cost abstractions and strict memory safety guarantees.
- **Full Standard Markdown Support:** Handles tables, code syntax blocks, checklists, and strikethroughs seamlessly.
- **Automated CI/CD:** Built-in GitHub Actions workflow to automatically build and deploy rendered documentation directly to GitHub Pages.

## 📦 Installation & Local Setup

### Prerequisites
Ensure you have the Rust toolchain installed on your machine. If not, install it via [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh


## Build and Run
git clone [https://github.com/Wishprogram01/ZW-CLI.git](https://github.com/Wishprogram01/ZW-CLI.git)
cd ZW-CLI

## Build the optimized release binary:
cargo build --release

## Run directly via Cargo
cargo run --release -- --input sample.md --output index.html

## Or execute the compiled release binary directly
./target/release/md2html --input sample.md --output index.html


