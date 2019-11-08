# Travesty in Rust lang

A simple project for me to learn Rust Lang.  This analyzes input text and then
randomly generates text output based on the execution.

## History

My first exposure was via a Pascal program published in the [November Issue of Byte Magazine](https://archive.org/stream/byte-magazine-1984-11/1984_11_BYTE_09-12_New_Chips#page/n129/mode/2up).
Since then I have implemented this algorithm to learn new languages.

## Getting Started

After cloning the repo you can quickly run with the following:
```sh
cargo run -- sample.txt
```

If you want to run with debugging on try the following:
```sh
RUST_BACKTRACE=1 cargo run -- -a 1000 -o 200 -d sample.txt
```

## Application Usage

Display usage message with `cargo run -- --help`

```
USAGE:
    travestyrs [FLAGS] [OPTIONS] [INPUT]

FLAGS:
    -d, --debug      Number of characters to output.
    -h, --help       Prints help information
    -v, --verse      Sets output to verse mode, defaults to prose
    -V, --version    Prints version information

OPTIONS:
    -a, --arrsize <array_size>       ArraySize
    -o, --outputsize <out_chars>     Number of characters to output.
    -p, --patlen <pattern_length>    Pattern Length

ARGS:
    <INPUT>    Sets the input file to use
```

## Attributions:
`sample.txt` - Extract from [bbejeck](https://github.com/bbejeck/hadoop-algorithms/blob/master/src/shakespeare.txt)
