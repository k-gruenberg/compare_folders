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
        --commononly               With this flag, only the commonalities are listed, i.e. only
                                   those files are outputted that occur in more than one folder but
                                   not necessarily in all of the folders and not necessarily under
                                   the same name. Files that occur more than once but only in one
                                   folder are not listed. This is NOT the exact opposite of the
                                   --diffonly flag!
        --diffonly                 With this flag, only the differences are listed, i.e. only those
                                   files are outputted that either (a) don't occur in all folders,
                                   or (b) don't have the same name in all folders, or (c) occur more
                                   than once in at least one folder. This is NOT the exact opposite
                                   of the --commononly flag!
        --extension <EXTENSION>    Optional filter: only regard files with this extension
    -h, --help                     Print help information
    -V, --version                  Print version information
```