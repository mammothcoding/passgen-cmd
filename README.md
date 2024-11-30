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

## Usage



#### You can create a strong token including all leterals, numbers and special symbols with length 30 charasters:

```rust
let result = Passgen::default().generate(30);
```



## License

[MIT](https://choosealicense.com/licenses/mit/)