# compare_folders
Simple command line tool to compare the contents of the given folders

## Installation

0. If you haven't installed *Rust* / *rustup* yet, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions for your operating system. 
1. `rustup update`
2. `cargo install --git https://github.com/k-gruenberg/compare_folders`

## Usage

```
compare_folders 1.0.0
Kendrick Grünberg
Simple command line tool to compare the contents of the given folders

USAGE:
    compare_folders [OPTIONS] <DIRECTORIES>...

ARGS:
    <DIRECTORIES>...    A list of multiple directories

OPTIONS:
        --colwidth <COLWIDTH>      The width of each column in the output ASCII table [default: 20]
        --extension <EXTENSION>    Optional filter: only regard files with this extension
    -h, --help                     Print help information
    -V, --version                  Print version information
```