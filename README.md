![alt text](./McDev_thin_900x70.png "McDev_thin_900x70.png")

[![Latest version](https://img.shields.io/crates/v/passgen-cmd.svg)](https://crates.io/crates/passgen-cmd)
[![Download](https://img.shields.io/crates/d/passgen-cmd.svg)](https://crates.io/crates/passgen-cmd)
[![docs.rs](https://docs.rs/passgen-cmd/badge.svg)](https://docs.rs/passgen-cmd/)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://choosealicense.com/licenses/mit/)
[![Build Status](https://github.com/mammothcoding/passgen-cmd/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/mammothcoding/passgen-cmd/actions/workflows/rust.yml)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

Readme in different languages:
[EN](https://github.com/mammothcoding/passgen-cmd/blob/master/README.md)
[RU](https://github.com/mammothcoding/passgen-cmd/blob/master/README.ru.md)

# 💻 Passgen-cmd

Cross-platform tool for generating cryptographically secure passwords/tokens and other sets and sequences

[CSPRNGs](https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs) Isaac64Rng and Hc128Rng are used.

## Install
```bash
cargo install passgen-cmd
```

## Usage

#### Print help
```bash
passgen-cmd -h
```
### Examples
#### You can create a strong token 30 characters long including all leterals, numbers and special symbols 30 characters long:
```bash
passgen-cmd 30
```
#### You can create a strong and usability password with default 8 characters long:
```bash
passgen-cmd -S
```
#### You can create a set from your custom charset 12 characters long:
```bash
passgen-cmd 12 -c bla@.321
```
#### You can create a token like Telegram tokens (first part: 10 numbers, second part: 30 characters from all leterals and numbers) [unix-like]:
```bash
echo $(passgen-cmd 10 -n)":"$(passgen-cmd 35 -lLn)
```
#### You can create a UUID (8-4-4-4-4-12 hexadecimal pattern) [unix-like]:
```bash
echo $(passgen-cmd 8 -c 0123456789abcdef)"-"$(passgen-cmd 4 -c 0123456789abcdef)"-"$(passgen-cmd 4 -c 0123456789abcdef)"-"$(passgen-cmd 4 -c 0123456789abcdef)"-"$(passgen-cmd 12 -c 0123456789abcdef)
```

## Downloads
[Releases/Builded packages](https://github.com/mammothcoding/passgen-cmd/releases/)

## License
[MIT](https://choosealicense.com/licenses/mit/)