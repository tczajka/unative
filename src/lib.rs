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
//! - `num-traits`: implements the standard `num-traits` integer traits (`Num`, `PrimInt`,
//!   `Bounded`, `Signed`/`Unsigned`, `CheckedAdd`/`WrappingAdd`/etc.) for [`UNative`] and
//!   [`INative`], for use as generic numeric types.
//! - `rand`: implements `rand::distr::Distribution<UNative>` / `Distribution<INative>` for
//!   `StandardUniform`, and `SampleUniform` for use with `Uniform` and `random_range`.
//! - `serde`: implements `Serialize`/`Deserialize` for [`UNative`] and [`INative`]. Values are
//!   serialized portably as `u64`/`i64`, so data written on one target can be read on another.

#![no_std]

pub use inative::INative;
pub use unative::UNative;

mod inative;
mod inner;
mod native;
mod unative;

#[cfg(feature = "num-traits")]
mod num_traits;

#[cfg(feature = "rand")]
mod rand;

#[cfg(feature = "serde")]
mod serde;
