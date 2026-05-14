//! Platform-native integer types.
//!
//! This crate provides [`UNative`] and [`INative`], unsigned and signed integer
//! types whose width is chosen to be the largest size for which the target
//! supports hardware arithmetic, including multiplication and division.

#![no_std]

pub use inative::INative;
pub use unative::UNative;

mod inative;
mod inner;
mod unative;
