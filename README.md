# nlpgen

A Rust command line tool for generating [natural language passwords](https://www.youtube.com/watch?v=QW4tSTiDCT8).

## Installation

There are two methods which can be used to install the tool:

### Download

Visit the [releases](https://github.com/cmarkwell/nlpgen/releases) section of this repository and download the binary appropriate for your operating system. Place the executable in your system path -- or route the path to it, and you are set to use `nlpgen`!

### Build

This tool can be manually built to support executable file formats that are not provided in the latest release. Since this tool is built with Rust, you will need to have [installed the language and its toolchain](https://www.rust-lang.org/tools/install). After setting up Rust, open your favorite terminal in the root of this repository and issue the command:

```
cargo build --release
```

If the build was successful, you will see a portable executable named `nlpgen` in the `target/release` directory!

## Usage

`nlpgen` is intended to be interacted with through a command line interface. 

```
Generate natural language passwords

USAGE:
    nlpgen.exe [OPTIONS]

OPTIONS:
    -c, --count <COUNT>              Number of natural language passwords to generate [default: 1]
    -l, --length <LENGTH>            Number of adj-n pairs per generated password [default: 1]
    -a, --adjectives <ADJECTIVES>    Path to a text file containing newline-delimited adjectives
    -n, --nouns <NOUNS>              Path to a text file containing newline-delimited nouns
    -h, --help                       Print help information
    -V, --version                    Print version information
```