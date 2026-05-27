# unative

[![CI](https://github.com/tczajka/unative/actions/workflows/ci.yml/badge.svg)](https://github.com/tczajka/unative/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/unative.svg)](https://crates.io/crates/unative)
[![docs.rs](https://docs.rs/unative/badge.svg)](https://docs.rs/unative)

Platform-native integer types.

This crate provides `UNative` and `INative`, unsigned and signed integer types
corresponding to the target platform's native integer types.

The two types always have the same width.

The concept of "native integer type" has no unambiguous definition, but we try
to choose the largest width for which the target supports efficient hardware
arithmetic, including multiplication and division.

Often this is the same width as `usize`, but not always. For example, on
`x86_64-unknown-linux-gnux32`, `usize` is 32 bits and `UNative` is 64 bits.

`UNative` and `INative` are always 16, 32, or 64 bits wide.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
