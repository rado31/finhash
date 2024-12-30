# Rust Hash Finder Application

## Description

- Rust console application that calculates SHA-256 hashes for consecutive integers, starting from 1. The program print the hash and the number if the hash ends with N zeros.

## How to use

- Example: ```finhash -N 3 -F 3```, will print:

```bash
4163, 95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000
11848, cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000
12843, bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000
```

- By default it uses 5 threads, so you can set another value for it, example:

```bash
finhash -N 3 -F 3 10
```

- For other informations, run: ```finhash --help```:

```bash
Usage: finhash -N <N> -F <F> [THREADS]

Arguments:
  [THREADS]  number of threads [default: 5]

Options:
  -N <N>         number of zeros
  -F <F>         number of results
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

### Pre build binary

- Download it from [releases](https://github.com/rado31/finhash/releases)

### Build it yourself

- ```git clone https://github.com/rado31/finhash``` clone to your folder
- ```cargo fetch``` install dependencies
- ```cargo build --release``` build for your platform

binary file location - ```target/release/finhash```

## Development

- ```cargo run```
- ```cargo test```


