# PNG Height&Width Fix (png-hw-fix) in Rust

[English Version](./README.md)

一个用于修复 PNG 图片，其宽高被人为调整过，CRC32 无法匹配的场景（多见于 CTF 题目中）

## 功能

1. 跨平台无需安装
2. 支持编译到 musl，无需运行时
3. 支持自动检测宽高被篡改的场景，并将原始宽高写入修复后的新文件

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

## 安装

### 预编译二进制文件

从 [Releases](https://github.com/yourusername/png-hw-fix/releases) 页面下载适合您系统的版本：

- Windows (x64): `png-hw-fix-windows-x86_64.exe`
- macOS (Intel): `png-hw-fix-macos-x86_64.tar.gz`
- macOS (Apple Silicon): `png-hw-fix-macos-aarch64.tar.gz`
- Linux (x64, GNU): `png-hw-fix-linux-x86_64-gnu.tar.gz`
- Linux (ARM64, GNU): `png-hw-fix-linux-aarch64-gnu.tar.gz`
- Linux (x64, MUSL): `png-hw-fix-linux-x86_64-musl.tar.gz`
- Linux (ARM64, MUSL): `png-hw-fix-linux-aarch64-musl.tar.gz`

### 从源码安装

需要安装 Rust 工具链，然后执行：

```sh
git clone https://github.com/yourusername/png-hw-fix.git
cd png-hw-fix
cargo install --path .
```

如需编译到 musl，请首先使用如下命令

```sh
rustup target add x86_64-unknown-linux-musl
cargo install --target x86_64-unknown-linux-musl --path .
```

## 使用说明

```sh
png-hw-fix <input-file-or-directory> [options]
```

### 选项

- `-r, --recursive` 递归处理子目录
- `-o, --output <output-directory>` 指定输出目录
- `-h, --help` 显示帮助信息

## 支持的平台

- Windows(x64)
- macOS(x64/arm64)
- Linux(x64/arm64 gnu/musl)

## 构建说明

1. 安装 Rust 工具链
2. 克隆仓库：`git clone https://github.com/yourusername/png-hw-fix.git`
3. 进入项目目录：`cd png-hw-fix`
4. 构建项目：`cargo build --release`
