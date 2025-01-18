# PNG Height&Width Fix (png-hw-fix) in Rust

[简体中文版本](./README-zh_CN.md)

A tool for fixing PNG images where the height and width have been manually altered, resulting in CRC32 mismatch (commonly seen in CTF challenges)

## Features

1. Cross-platform, no installation required
2. Supports MUSL compilation, no runtime dependencies
3. Automatically detects tampered dimensions and writes original dimensions to a new fixed file

```bash
$ png-hw-fix
Fixes PNG dimensions by brute-forcing CRC32

Usage: png-hw-fix [OPTIONS] <PNG_FILE_PATH>

Arguments:
  <PNG_FILE_PATH>  Path to the PNG file

Options:
  -o, --output <OUTPUT>  Output file name
  -h, --help             Print help
  -V, --version          Print version
```

## Installation

### Pre-compiled Binaries

Download the appropriate version for your system from the [Releases](https://github.com/yourusername/png-hw-fix/releases) page:

- Windows (x64): `png-hw-fix-windows-x86_64.exe`
- macOS (Intel): `png-hw-fix-macos-x86_64.tar.gz`
- macOS (Apple Silicon): `png-hw-fix-macos-aarch64.tar.gz`
- Linux (x64, GNU): `png-hw-fix-linux-x86_64-gnu.tar.gz`
- Linux (ARM64, GNU): `png-hw-fix-linux-aarch64-gnu.tar.gz`
- Linux (x64, MUSL): `png-hw-fix-linux-x86_64-musl.tar.gz`
- Linux (ARM64, MUSL): `png-hw-fix-linux-aarch64-musl.tar.gz`

### Building from Source

Install the Rust toolchain and run:

```sh
git clone https://github.com/yourusername/png-hw-fix.git
cd png-hw-fix
cargo install --path .
```

For MUSL compilation, first run:

```sh
rustup target add x86_64-unknown-linux-musl
cargo install --target x86_64-unknown-linux-musl --path .
```

## Usage

```sh
png-hw-fix <input-file-or-directory> [options]
```

### Options

- `-r, --recursive` Process subdirectories recursively
- `-o, --output <output-directory>` Specify output directory
- `-h, --help` Display help information

## Supported Platforms

- Windows(x64)
- macOS(x64/arm64)
- Linux(x64/arm64 gnu/musl)

## Build Instructions

1. Install Rust toolchain
2. Clone repository: `git clone https://github.com/yourusername/png-hw-fix.git`
3. Enter project directory: `cd png-hw-fix`
4. Build project: `cargo build --release`
