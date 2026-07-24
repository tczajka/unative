//! Tests for the `serde` integration.

#![cfg(feature = "serde")]

use serde_test::{Token, assert_de_tokens, assert_de_tokens_error, assert_tokens};
use unative::{INative, UNative};

#[test]
fn unative_round_trip_zero() {
    assert_tokens(&UNative::ZERO, &[Token::U64(0)]);
}

#[test]
fn unative_round_trip_small() {
    assert_tokens(&UNative::from(12345u16), &[Token::U64(12345)]);
}

#[test]
fn unative_round_trip_max() {
    let max = UNative::MAX;
    assert_tokens(&max, &[Token::U64(u64::from(max))]);
}

#[test]
fn inative_round_trip_zero() {
    assert_tokens(&INative::from(0i8), &[Token::I64(0)]);
}

#[test]
fn inative_round_trip_positive() {
    assert_tokens(&INative::from(12345i16), &[Token::I64(12345)]);
}

#[test]
fn inative_round_trip_negative() {
    assert_tokens(&INative::from(-12345i16), &[Token::I64(-12345)]);
}

#[test]
fn inative_round_trip_min() {
    let min = INative::MIN;
    assert_tokens(&min, &[Token::I64(i64::from(min))]);
}

#[test]
fn inative_round_trip_max() {
    let max = INative::MAX;
    assert_tokens(&max, &[Token::I64(i64::from(max))]);
}

#[test]
fn unative_deserialize_accepts_smaller_widths() {
    assert_de_tokens(&UNative::from(5u8), &[Token::U8(5)]);
    assert_de_tokens(&UNative::from(5u8), &[Token::U16(5)]);
    assert_de_tokens(&UNative::from(5u8), &[Token::U32(5)]);
    assert_de_tokens(&UNative::from(5u8), &[Token::I64(5)]);
}

#[test]
fn inative_deserialize_accepts_smaller_widths() {
    assert_de_tokens(&INative::from(-3i8), &[Token::I8(-3)]);
    assert_de_tokens(&INative::from(-3i8), &[Token::I16(-3)]);
    assert_de_tokens(&INative::from(5i8), &[Token::U32(5)]);
}

#[test]
fn unative_deserialize_rejects_negative() {
    assert_de_tokens_error::<UNative>(&[Token::I64(-1)], "value out of range for UNative");
}

#[test]
fn unative_deserialize_rejects_overflow() {
    // Pick a value strictly greater than UNative::MAX, if one fits in u64.
    // On a 64-bit-wide UNative target this is unreachable through serde_test
    // (no U128 token), so the check is skipped there.
    let above_max = u128::from(UNative::MAX) + 1;
    if let Ok(above_max_u64) = u64::try_from(above_max) {
        assert_de_tokens_error::<UNative>(
            &[Token::U64(above_max_u64)],
            "value out of range for UNative",
        );
    }
}

#[test]
fn inative_deserialize_rejects_overflow() {
    // u64::MAX exceeds INative::MAX on every supported target width (16/32/64).
    assert_de_tokens_error::<INative>(&[Token::U64(u64::MAX)], "value out of range for INative");
}
