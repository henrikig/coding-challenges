# ccwc

## Inspiration

[Build Your Own wc Tool](https://codingchallenges.fyi/challenges/challenge-wc)

## Description

A simple implementation of the `wc` (word count) utility

Usage: ccwc [OPTIONS] [FILE]

Arguments:
  [FILE]  File name. If no filename is provided, stdin is used

Options:
  -c             Number of bytes in file
  -l             Number of lines in file
  -w             Number of words in file
  -m             Number of characters in file, supporting multibyte characters
  -h, --help     Print help
  -V, --version  Print version

## Installation and Usage

```sh
cargo build --release
```

```sh
./target/release/ccwc -h
```
