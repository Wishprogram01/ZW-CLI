# ZW-CLI: Markdown to HTML Converter 🦀

Alat CLI pantas yang dibina menggunakan **Rust** dan `pulldown-cmark` untuk menukar fail Markdown (.md) kepada dokumen HTML yang lengkap.

## 🚀 Ciri-Ciri
- Kompilasi pantas dengan keselamatan memori Rust
- Sokongan sintaks standard Markdown (jadual, kod, senarai, dll.)
- Automasi build & deploy terus ke GitHub Pages melalui GitHub Actions

## 📦 Pemasangan & Penggunaan Tempatan

```bash
# Klon repositori
git clone [https://github.com/Wishprogram01/ZW-CLI.git](https://github.com/Wishprogram01/ZW-CLI.git)
cd ZW-CLI

# Bina versi release
cargo build --release

# Jalankan CLI
cargo run --release -- --input fail_anda.md --output hasil.html
