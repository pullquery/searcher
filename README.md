# Searcher

CLI searcher built with Rust.

Example from '[The Rust Programming Language](https://doc.rust-lang.org/book/)', chapter 12.

## Usage

```sh
# Run
cargo run [query] [filename]
cargo run language ./examples/example.txt

# Build
cargo build
```
Set environment variable `CASE_INSENSITIVE` as any value to search case insensitive.

```sh
# Shell
CASE_INSENSITIVE=1

# PowerShell
$env:CASE_INSENSITIVE=1
```