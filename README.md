# Password Generator

A simple command-line password generator written in Rust.

## Features

- Generate secure passwords with custom length and maximum length.
- Supports both short (`-l`, `-m`, `-V`) and long (`--length`, `--max`, `--help`, `--version`) options.

## Installation

To install the password generator, first clone the repository and then run:

```bash
cargo build --release
```

## Usage

```bash
password-gen-rust.exe [OPTIONS]
```

```bash
password-gen-rust.exe --level 3
```

```bash
password-gen-rust.exe --max 1
```
