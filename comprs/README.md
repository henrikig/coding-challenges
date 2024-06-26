# comprs

This is a simple command line tool to compress and decompress files using
Huffman coding.

## Usage

```text
A program to encode and decode files with Huffman encoding.

Usage: comprs <COMMAND>

Commands:
  encode
  decode
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

To invoke with cargo, run the following:

```sh
# Encode a file
cargo run --release -- encode <source> <destination>
# Decode a file
cargo run --release -- decode <source> <destination>
```
