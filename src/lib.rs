//! Platform-native integer types.
//!
//! This crate provides [`UNative`] and [`INative`], unsigned and signed integer
//! types corresponding to the target platform's native integer types.
//!
//! The two types always have the same width.
//!
//! The concept of "native integer type" has no unambiguous definition, but we try to choose the
//! largest width for which the target supports efficient hardware arithmetic, including
//! multiplication and division.
//!
//! Often this is the same width as `usize`, but not always. For example, on
//! `x86_64-unknown-linux-gnux32`, `usize` is 32 bits and `UNative` is 64 bits.
//!
//! [`UNative`] and [`INative`] are always 16, 32, or 64 bits wide.
//!
//! # Features
//!
//! - `num-traits`: [`num-traits`](https://crates.io/crates/num-traits) integer-trait impls.
//! - `proptest`: [`proptest`](https://crates.io/crates/proptest) support.
//! - `rand`: [`rand`](https://crates.io/crates/rand) sampling.
//! - `serde`: [`serde`](https://crates.io/crates/serde) portable serialization.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg))]
#![no_std]

pub use inative::INative;
pub use unative::UNative;

mod inative;
mod inner;
mod native;
mod unative;

#[cfg(feature = "num-traits")]
mod num_traits;

#[cfg(feature = "proptest")]
pub mod proptest;

#[cfg(feature = "rand")]
mod rand;

#[cfg(feature = "serde")]
mod serde;
