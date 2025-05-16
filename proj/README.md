# Website Status Checker (Rust)

A concurrent command-line util. to check availability and latency of multiple websites in parallel, entirely built  in Rust.

## Main Features

- Accepts input URLs via a text file (`--file`) or as command-line arguments
- Fixed-size worker thread pool for concurrent requests
- Configurable per-request timeout (`--timeout`)
- Optional retries on failure (`--retries`)
- Real-time terminal output and JSON result summary (`status.json`)

##  Build & Run Instructions

```bash
#Clone repository
git clone https://github.com/Yuririyu/RustSpring2025.git
cd proj/website_checker

#Build in release mode
cargo build --release
```
##  Usage
Note: If neither --file nor any positional URLs are provided, the program prints usage help and exits with code 2.
```bash
#Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]
```
Run in terminal with:
```bash
#FOR INCLUDED FILE:
cargo run -- --file websites.txt
#Release ver.
cargo run --release -- --file websites.txt
#With additional arguments eg.
cargo run --release -- --file websites.txt --workers 8 --timeout 5 --retries 1

#FOR DIRECT URL:
cargo run -- https://google.com
#Release ver. & multiple URLs
cargo run --release -- https://fakeurl.com https://utrgv.edu

```
