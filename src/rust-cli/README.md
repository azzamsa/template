<div align="center">
  <h1>Rust CLI</h1>

<img src='docs/construction.svg' width=80px />

Rust CLI Template 🏗️

<a href="https://github.com/azzamsa/rust-cli/workflows/ci.yml">
    <img src="https://github.com/azzamsa/rust-cli/workflows/ci/badge.svg" alt="Build status" />
  </a>

</div>

---

## Features

- Read the file blazingly fast!

## Usage

## Usage Examples

```bash
$ cli text.md                              Print the file
$ cli foo.md bar.tex                       ... multiple files
$ cli foo.md bar.tex --color never         ... without color
```

### Command-line options

```bash
cli [version]
CLI 🔐.

            Rust CLI Template

USAGE:
    cli [OPTIONS] <FILE>...

ARGS:
    <FILE>...    File(s) to print

OPTIONS:
        --color <color>    When to use colors (*auto*, never, always) [default: auto] [possible
                           values: auto, never, always]
    -h, --help             Print help information
    -V, --version          Print version information

Note: `cli -h` prints a short and concise overview while `cli --help` gives all details
```

## Installation

### From binaries

The [release page](https://github.com/azzamsa/rust-cli/releases) includes
pre-compiled binaries for GNU/Linux, macOS and Windows.

### From source

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
$ cargo install cli
```

## Development

```bash
$ clone the repository 

$ # Run unit tests and integration tests
$ cargo test

$ # Install
$ cargo install --path .
```

## Credits

- [bat: A cat(1) clone with wings.](https://github.com/sharkdp/bat) by David Peter
- Icons and emoji from [Noto Emoji](https://github.com/googlefonts/noto-emoji)
