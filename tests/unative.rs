#![allow(clippy::op_ref)]

use core::cmp::Ordering;
use proptest::prelude::*;
use unative::{INative, UNative};

proptest! {
    // `u8 -> UNative -> u128` agrees with `u8 -> u128`, and `from_u8` agrees with `From`.
    #[test]
    fn from_u8_unative_u128(x: u8) {
        prop_assert_eq!(u128::from(UNative::from(x)), u128::from(x));
        prop_assert_eq!(UNative::from_u8(x), UNative::from(x));
    }

    // `u16 -> UNative -> u64` agrees with `u16 -> u64`, and `from_u16` agrees with `From`.
    #[test]
    fn from_u16_unative_u64(x: u16) {
        prop_assert_eq!(u64::from(UNative::from(x)), u64::from(x));
        prop_assert_eq!(UNative::from_u16(x), UNative::from(x));
    }

    // `bool -> UNative -> u64` agrees with `bool -> u64`.
    #[test]
    fn from_bool_unative_u64(b: bool) {
        prop_assert_eq!(u64::from(UNative::from(b)), u64::from(b));
    }
}

#[test]
fn try_from() {
    assert_eq!(UNative::try_from(0u32), Ok(UNative::ZERO));
    assert_eq!(UNative::try_from(42i64), Ok(UNative::from(42u8)));
    assert!(UNative::try_from(-1i8).is_err());

    let max = u128::from(UNative::MAX);
    assert_eq!(UNative::try_from(max), Ok(UNative::MAX));
    assert!(UNative::try_from(max + 1).is_err());

    let small = UNative::from(42u8);
    assert_eq!(u8::try_from(small), Ok(42u8));
    assert_eq!(i8::try_from(small), Ok(42i8));
    assert!(u8::try_from(UNative::from(u16::MAX)).is_err());

    assert_eq!(
        UNative::try_from(INative::from(42i8)),
        Ok(UNative::from(42u8))
    );
    assert!(UNative::try_from(INative::from(-1i8)).is_err());

    // Conversion to `bool`.
    assert_eq!(bool::try_from(UNative::ZERO), Ok(false));
    assert_eq!(bool::try_from(UNative::from(1u8)), Ok(true));
    assert!(bool::try_from(UNative::from(2u8)).is_err());
    assert!(bool::try_from(UNative::MAX).is_err());
}

#[test]
fn const_conversions() {
    // Usable in const context.
    const FORTY_TWO: UNative = UNative::from_u8(42);
    assert_eq!(FORTY_TWO, UNative::from(42u8));

    // Infallible `from_*`.
    assert_eq!(UNative::from_u8(0), UNative::ZERO);
    assert_eq!(UNative::from_u8(u8::MAX), UNative::from(u8::MAX));
    assert_eq!(UNative::from_u16(u16::MAX), UNative::from(u16::MAX));

    // Fallible `from_*`.
    let max = u128::from(UNative::MAX);
    assert_eq!(UNative::try_from_u32(42), Some(UNative::from(42u8)));
    assert_eq!(UNative::try_from_u64(42), Some(UNative::from(42u8)));
    assert_eq!(UNative::try_from_u128(max), Some(UNative::MAX));
    assert_eq!(UNative::try_from_u128(max + 1), None);
    assert_eq!(UNative::try_from_usize(42), Some(UNative::from(42u8)));

    // Infallible `to_*`.
    assert_eq!(UNative::from(42u8).to_u64(), 42u64);
    assert_eq!(u128::from(UNative::MAX), UNative::MAX.to_u128());

    // Fallible `to_*`.
    assert_eq!(UNative::from(42u8).try_to_u8(), Some(42u8));
    assert_eq!(UNative::from(u16::MAX).try_to_u8(), None);
    assert_eq!(UNative::from(42u8).try_to_u16(), Some(42u16));
    assert_eq!(UNative::from(42u8).try_to_u32(), Some(42u32));
    assert_eq!(UNative::from(42u8).try_to_usize(), Some(42usize));
}

#[test]
fn default() {
    assert_eq!(UNative::default(), UNative::ZERO);
}

#[test]
fn bits() {
    assert_eq!(UNative::BITS, INative::BITS);
    assert_eq!(UNative::MAX >> (UNative::BITS - 1), UNative::from(1u8));
}

#[test]
fn bytes() {
    assert_eq!(UNative::BYTES * 8, UNative::BITS as usize);
    assert_eq!(UNative::BYTES, INative::BYTES);
}

proptest! {
    // `from_*_bytes` round-trips `to_*_bytes` for every value, in all three byte orders.
    #[test]
    fn bytes_round_trip(x: UNative) {
        prop_assert_eq!(UNative::from_be_bytes(x.to_be_bytes()), x);
        prop_assert_eq!(UNative::from_le_bytes(x.to_le_bytes()), x);
        prop_assert_eq!(UNative::from_ne_bytes(x.to_ne_bytes()), x);
    }
}

#[test]
fn bytes_byte_order() {
    // A small value occupies the least-significant byte: the highest-index byte in big-endian
    // and the lowest-index byte in little-endian.
    let x = UNative::from(42u8);

    let be = x.to_be_bytes();
    assert_eq!(be[UNative::BYTES - 1], 42);
    assert!(be[..UNative::BYTES - 1].iter().all(|&b| b == 0));

    let le = x.to_le_bytes();
    assert_eq!(le[0], 42);
    assert!(le[1..].iter().all(|&b| b == 0));
}

#[test]
fn midpoint() {
    assert_eq!(
        UNative::from(4u8).midpoint(UNative::from(10u8)),
        UNative::from(7u8),
    );
    assert_eq!(
        UNative::from(5u8).midpoint(UNative::from(10u8)),
        UNative::from(7u8),
    );
    assert_eq!(UNative::ZERO.midpoint(UNative::ZERO), UNative::ZERO);
    // No overflow even for very large operands.
    assert_eq!(UNative::MAX.midpoint(UNative::MAX), UNative::MAX);
}

#[test]
fn ilog() {
    assert_eq!(UNative::from(100u8).ilog(UNative::from(10u8)), 2);
    assert_eq!(UNative::from(99u8).ilog(UNative::from(10u8)), 1);
    assert_eq!(UNative::from(1u8).ilog(UNative::from(10u8)), 0);
}

#[test]
#[should_panic]
fn ilog_of_zero() {
    let _ = UNative::ZERO.ilog(UNative::from(10u8));
}

#[test]
#[should_panic]
fn ilog_with_base_1() {
    let _ = UNative::from(10u8).ilog(UNative::from(1u8));
}

#[test]
fn ilog2() {
    assert_eq!(UNative::from(1u8).ilog2(), 0);
    assert_eq!(UNative::from(2u8).ilog2(), 1);
    assert_eq!(UNative::from(3u8).ilog2(), 1);
    assert_eq!(UNative::from(4u8).ilog2(), 2);
    assert_eq!(UNative::MAX.ilog2(), UNative::BITS - 1);
}

#[test]
#[should_panic]
fn ilog2_of_zero() {
    let _ = UNative::ZERO.ilog2();
}

#[test]
fn ilog10() {
    assert_eq!(UNative::from(1u8).ilog10(), 0);
    assert_eq!(UNative::from(9u8).ilog10(), 0);
    assert_eq!(UNative::from(10u8).ilog10(), 1);
    assert_eq!(UNative::from(99u8).ilog10(), 1);
    assert_eq!(UNative::from(100u8).ilog10(), 2);
}

#[test]
#[should_panic]
fn ilog10_of_zero() {
    let _ = UNative::ZERO.ilog10();
}

#[test]
fn pow() {
    assert_eq!(UNative::from(2u8).pow(10), UNative::from(1024u16));
    assert_eq!(UNative::from(3u8).pow(0), UNative::from(1u8));
    assert_eq!(UNative::ZERO.pow(0), UNative::from(1u8));
    assert_eq!(UNative::ZERO.pow(5), UNative::ZERO);
}

#[test]
fn isqrt() {
    assert_eq!(UNative::ZERO.isqrt(), UNative::ZERO);
    assert_eq!(UNative::from(1u8).isqrt(), UNative::from(1u8));
    assert_eq!(UNative::from(4u8).isqrt(), UNative::from(2u8));
    assert_eq!(UNative::from(8u8).isqrt(), UNative::from(2u8));
    assert_eq!(UNative::from(9u8).isqrt(), UNative::from(3u8));
    assert_eq!(UNative::from(100u8).isqrt(), UNative::from(10u8));
}

#[test]
fn div_euclid() {
    assert_eq!(
        UNative::from(23u8).div_euclid(UNative::from(10u8)),
        UNative::from(2u8),
    );
    assert_eq!(
        UNative::from(20u8).div_euclid(UNative::from(10u8)),
        UNative::from(2u8),
    );
    assert_eq!(UNative::ZERO.div_euclid(UNative::from(5u8)), UNative::ZERO);
}

#[test]
#[should_panic]
fn div_euclid_by_zero() {
    let _ = UNative::from(5u8).div_euclid(UNative::ZERO);
}

#[test]
fn rem_euclid() {
    assert_eq!(
        UNative::from(23u8).rem_euclid(UNative::from(10u8)),
        UNative::from(3u8),
    );
    assert_eq!(
        UNative::from(20u8).rem_euclid(UNative::from(10u8)),
        UNative::ZERO,
    );
    assert_eq!(UNative::ZERO.rem_euclid(UNative::from(5u8)), UNative::ZERO);
}

#[test]
#[should_panic]
fn rem_euclid_by_zero() {
    let _ = UNative::from(5u8).rem_euclid(UNative::ZERO);
}

#[test]
fn count_ones() {
    assert_eq!(UNative::ZERO.count_ones(), 0);
    assert_eq!(UNative::MAX.count_ones(), UNative::BITS);
    assert_eq!(UNative::from(0b10110u8).count_ones(), 3);
}

#[test]
fn count_zeros() {
    assert_eq!(UNative::ZERO.count_zeros(), UNative::BITS);
    assert_eq!(UNative::MAX.count_zeros(), 0);
    let x = UNative::from(0b10110u8);
    assert_eq!(x.count_ones() + x.count_zeros(), UNative::BITS);
}

#[test]
fn leading_zeros() {
    assert_eq!(UNative::ZERO.leading_zeros(), UNative::BITS);
    assert_eq!(UNative::MAX.leading_zeros(), 0);
    assert_eq!(UNative::from(1u8).leading_zeros(), UNative::BITS - 1);
}

#[test]
fn trailing_zeros() {
    assert_eq!(UNative::ZERO.trailing_zeros(), UNative::BITS);
    assert_eq!(UNative::MAX.trailing_zeros(), 0);
    assert_eq!(UNative::from(1u8).trailing_zeros(), 0);
    assert_eq!(UNative::from(0b1000u8).trailing_zeros(), 3);
}

#[test]
fn leading_ones() {
    assert_eq!(UNative::ZERO.leading_ones(), 0);
    assert_eq!(UNative::MAX.leading_ones(), UNative::BITS);
    assert_eq!((UNative::MAX << 1u8).leading_ones(), UNative::BITS - 1);
}

#[test]
fn trailing_ones() {
    assert_eq!(UNative::ZERO.trailing_ones(), 0);
    assert_eq!(UNative::MAX.trailing_ones(), UNative::BITS);
    assert_eq!(UNative::from(0b1011u8).trailing_ones(), 2);
}

#[test]
fn rotate_left() {
    assert_eq!(UNative::ZERO.rotate_left(3), UNative::ZERO);
    assert_eq!(UNative::MAX.rotate_left(3), UNative::MAX);
    assert_eq!(UNative::from(42u8).rotate_left(0), UNative::from(42u8));
    assert_eq!(UNative::from(1u8).rotate_left(3), UNative::from(8u8));
    let x = UNative::from(42u8);
    assert_eq!(x.rotate_left(UNative::BITS), x);
    assert_eq!(
        UNative::from(1u8).rotate_left(UNative::BITS - 1),
        UNative::from(1u8) << (UNative::BITS - 1),
    );
}

#[test]
fn rotate_right() {
    assert_eq!(UNative::ZERO.rotate_right(3), UNative::ZERO);
    assert_eq!(UNative::MAX.rotate_right(3), UNative::MAX);
    assert_eq!(UNative::from(42u8).rotate_right(0), UNative::from(42u8));
    assert_eq!(UNative::from(8u8).rotate_right(3), UNative::from(1u8));
    let x = UNative::from(42u8);
    assert_eq!(x.rotate_right(UNative::BITS), x);
    assert_eq!(x.rotate_left(5).rotate_right(5), x);
}

#[test]
fn swap_bytes() {
    assert_eq!(UNative::ZERO.swap_bytes(), UNative::ZERO);
    assert_eq!(UNative::MAX.swap_bytes(), UNative::MAX);
    assert_eq!(
        UNative::from(1u8).swap_bytes(),
        UNative::from(1u8) << (UNative::BITS - 8),
    );
    let x = UNative::from(42u8);
    assert_eq!(x.swap_bytes().swap_bytes(), x);
}

#[test]
fn reverse_bits() {
    assert_eq!(UNative::ZERO.reverse_bits(), UNative::ZERO);
    assert_eq!(UNative::MAX.reverse_bits(), UNative::MAX);
    assert_eq!(
        UNative::from(1u8).reverse_bits(),
        UNative::from(1u8) << (UNative::BITS - 1),
    );
    let x = UNative::from(42u8);
    assert_eq!(x.reverse_bits().reverse_bits(), x);
}

#[test]
fn big_endian() {
    let x = UNative::from(42u8);
    assert_eq!(UNative::from_be(x.to_be()), x);
    assert_eq!(UNative::ZERO.to_be(), UNative::ZERO);
    assert_eq!(UNative::MAX.to_be(), UNative::MAX);
    assert_eq!(UNative::from_be(UNative::MAX), UNative::MAX);
}

#[test]
fn little_endian() {
    let x = UNative::from(42u8);
    assert_eq!(UNative::from_le(x.to_le()), x);
    assert_eq!(UNative::ZERO.to_le(), UNative::ZERO);
    assert_eq!(UNative::MAX.to_le(), UNative::MAX);
    assert_eq!(UNative::from_le(UNative::MAX), UNative::MAX);
}

#[test]
fn cast_signed() {
    assert_eq!(UNative::ZERO.cast_signed(), INative::ZERO);
    assert_eq!(UNative::MAX.cast_signed(), INative::from(-1i8));
    assert_eq!(UNative::from(1u8).cast_signed(), INative::from(1i8));
    assert_eq!(
        (UNative::from(1u8) << (UNative::BITS - 1)).cast_signed(),
        INative::MIN,
    );
    let x = UNative::from(42u8);
    assert_eq!(x.cast_signed().cast_unsigned(), x);
}

#[test]
fn equality() {
    let x = UNative::from(42u8);
    assert_eq!(x, x);
    assert_eq!(&x, &UNative::from(42u8));
    assert_eq!(UNative::from(42u8), UNative::from(42u16));
    assert_ne!(UNative::from(1u8), UNative::from(2u8));
    assert_eq!(UNative::MIN, UNative::ZERO);
    assert_ne!(UNative::MIN, UNative::MAX);
}

#[test]
fn ordering() {
    let a = UNative::from(1u8);
    let b = UNative::from(2u8);
    assert!(a < b);
    assert!(b > a);
    assert!(a <= b);
    assert!(a <= a);
    assert!(b >= a);
    assert!(b >= b);
    assert_eq!(a.cmp(&b), Ordering::Less);
    assert_eq!(b.cmp(&a), Ordering::Greater);
    assert_eq!(a.cmp(&a), Ordering::Equal);
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    assert!(UNative::MIN < UNative::MAX);
}

#[test]
fn addition() {
    let a = UNative::from(2u8);
    let b = UNative::from(3u8);
    let sum = UNative::from(5u8);
    assert_eq!(a + b, sum);
    assert_eq!(a + &b, sum);
    assert_eq!(&a + b, sum);
    assert_eq!(&a + &b, sum);

    let mut s = a;
    s += b;
    assert_eq!(s, sum);
    s += &a;
    assert_eq!(s, UNative::from(7u8));
}

#[test]
fn subtraction() {
    let a = UNative::from(7u8);
    let b = UNative::from(3u8);
    let diff = UNative::from(4u8);
    assert_eq!(a - b, diff);
    assert_eq!(a - &b, diff);
    assert_eq!(&a - b, diff);
    assert_eq!(&a - &b, diff);

    let mut s = a;
    s -= b;
    assert_eq!(s, diff);
    s -= &b;
    assert_eq!(s, UNative::from(1u8));
}

#[test]
fn multiplication() {
    let a = UNative::from(4u8);
    let b = UNative::from(3u8);
    let prod = UNative::from(12u8);
    assert_eq!(a * b, prod);
    assert_eq!(a * &b, prod);
    assert_eq!(&a * b, prod);
    assert_eq!(&a * &b, prod);

    let mut s = a;
    s *= b;
    assert_eq!(s, prod);
    s *= &a;
    assert_eq!(s, UNative::from(48u8));
}

#[test]
fn division() {
    let a = UNative::from(20u8);
    let b = UNative::from(4u8);
    let quot = UNative::from(5u8);
    assert_eq!(a / b, quot);
    assert_eq!(a / &b, quot);
    assert_eq!(&a / b, quot);
    assert_eq!(&a / &b, quot);

    let mut s = a;
    s /= b;
    assert_eq!(s, quot);
    s /= &b;
    assert_eq!(s, UNative::from(1u8));
}

#[test]
fn div_ceil() {
    assert_eq!(
        UNative::from(10u8).div_ceil(UNative::from(3u8)),
        UNative::from(4u8),
    );
    assert_eq!(
        UNative::from(9u8).div_ceil(UNative::from(3u8)),
        UNative::from(3u8),
    );
    assert_eq!(UNative::ZERO.div_ceil(UNative::from(5u8)), UNative::ZERO);
}

#[test]
#[should_panic]
fn div_ceil_by_zero() {
    let _ = UNative::from(5u8).div_ceil(UNative::ZERO);
}

#[test]
fn abs_diff() {
    assert_eq!(
        UNative::from(7u8).abs_diff(UNative::from(3u8)),
        UNative::from(4u8),
    );
    assert_eq!(
        UNative::from(3u8).abs_diff(UNative::from(7u8)),
        UNative::from(4u8),
    );
    assert_eq!(UNative::ZERO.abs_diff(UNative::ZERO), UNative::ZERO);
    assert_eq!(UNative::MAX.abs_diff(UNative::ZERO), UNative::MAX);
    assert_eq!(UNative::ZERO.abs_diff(UNative::MAX), UNative::MAX);
}

#[test]
fn next_multiple_of() {
    assert_eq!(
        UNative::from(7u8).next_multiple_of(UNative::from(3u8)),
        UNative::from(9u8),
    );
    assert_eq!(
        UNative::from(6u8).next_multiple_of(UNative::from(3u8)),
        UNative::from(6u8),
    );
    assert_eq!(
        UNative::ZERO.next_multiple_of(UNative::from(3u8)),
        UNative::ZERO,
    );
}

#[test]
#[should_panic]
fn next_multiple_of_by_zero() {
    let _ = UNative::from(5u8).next_multiple_of(UNative::ZERO);
}

#[test]
fn is_multiple_of() {
    assert!(UNative::from(6u8).is_multiple_of(UNative::from(3u8)));
    assert!(!UNative::from(7u8).is_multiple_of(UNative::from(3u8)));
    assert!(UNative::ZERO.is_multiple_of(UNative::from(3u8)));
    assert!(UNative::ZERO.is_multiple_of(UNative::ZERO));
    assert!(!UNative::from(5u8).is_multiple_of(UNative::ZERO));
}

#[test]
fn next_power_of_two() {
    assert_eq!(UNative::ZERO.next_power_of_two(), UNative::from(1u8));
    assert_eq!(UNative::from(1u8).next_power_of_two(), UNative::from(1u8));
    assert_eq!(UNative::from(5u8).next_power_of_two(), UNative::from(8u8));
    assert_eq!(UNative::from(8u8).next_power_of_two(), UNative::from(8u8));
}

#[test]
#[should_panic]
fn next_power_of_two_overflow() {
    let _ = UNative::MAX.next_power_of_two();
}

#[test]
fn is_power_of_two() {
    assert!(!UNative::ZERO.is_power_of_two());
    assert!(UNative::from(1u8).is_power_of_two());
    assert!(UNative::from(2u8).is_power_of_two());
    assert!(!UNative::from(3u8).is_power_of_two());
    assert!(UNative::from(64u8).is_power_of_two());
}

#[test]
fn remainder() {
    let a = UNative::from(23u8);
    let b = UNative::from(5u8);
    let rem = UNative::from(3u8);
    assert_eq!(a % b, rem);
    assert_eq!(a % &b, rem);
    assert_eq!(&a % b, rem);
    assert_eq!(&a % &b, rem);

    let mut s = a;
    s %= b;
    assert_eq!(s, rem);
    s %= &UNative::from(2u8);
    assert_eq!(s, UNative::from(1u8));
}

#[test]
fn bit_and() {
    let a = UNative::from(0b1100u8);
    let b = UNative::from(0b1010u8);
    let and = UNative::from(0b1000u8);
    assert_eq!(a & b, and);
    assert_eq!(a & &b, and);
    assert_eq!(&a & b, and);
    assert_eq!(&a & &b, and);

    let mut s = a;
    s &= b;
    assert_eq!(s, and);
    s &= &UNative::from(0b0111u8);
    assert_eq!(s, UNative::from(0b0000u8));
}

#[test]
fn bit_or() {
    let a = UNative::from(0b1100u8);
    let b = UNative::from(0b1010u8);
    let or = UNative::from(0b1110u8);
    assert_eq!(a | b, or);
    assert_eq!(a | &b, or);
    assert_eq!(&a | b, or);
    assert_eq!(&a | &b, or);

    let mut s = a;
    s |= b;
    assert_eq!(s, or);
    s |= &UNative::from(0b0001u8);
    assert_eq!(s, UNative::from(0b1111u8));
}

#[test]
fn bit_xor() {
    let a = UNative::from(0b1100u8);
    let b = UNative::from(0b1010u8);
    let xor = UNative::from(0b0110u8);
    assert_eq!(a ^ b, xor);
    assert_eq!(a ^ &b, xor);
    assert_eq!(&a ^ b, xor);
    assert_eq!(&a ^ &b, xor);

    let mut s = a;
    s ^= b;
    assert_eq!(s, xor);
    s ^= &a;
    assert_eq!(s, UNative::from(0b1010u8));
}

#[test]
fn not() {
    let a = UNative::from(5u8);
    assert_eq!(!a, !&a);
    assert_eq!(!!a, a);
    assert_eq!(!UNative::ZERO, UNative::MAX);
    assert_eq!(!UNative::MAX, UNative::ZERO);
}

#[test]
fn shl() {
    let a = UNative::from(1u8);
    let shifted = UNative::from(8u8);

    assert_eq!(a << 3u8, shifted);
    assert_eq!(a << &3u8, shifted);
    assert_eq!(&a << 3u8, shifted);
    assert_eq!(&a << &3u8, shifted);
    assert_eq!(a << UNative::from(3u8), shifted);
    assert_eq!(a << INative::from(3i8), shifted);
    assert_eq!(1u32 << UNative::from(3u8), 8u32);

    let mut s = a;
    s <<= 3u8;
    assert_eq!(s, shifted);
    s <<= &UNative::from(1u8);
    assert_eq!(s, UNative::from(16u8));
}

#[test]
fn shr() {
    let a = UNative::from(16u8);
    let shifted = UNative::from(2u8);

    assert_eq!(a >> 3u8, shifted);
    assert_eq!(a >> &3u8, shifted);
    assert_eq!(&a >> 3u8, shifted);
    assert_eq!(&a >> &3u8, shifted);
    assert_eq!(a >> UNative::from(3u8), shifted);
    assert_eq!(a >> INative::from(3i8), shifted);
    assert_eq!(16u32 >> UNative::from(3u8), 2u32);

    let mut s = a;
    s >>= 3u8;
    assert_eq!(s, shifted);
    s >>= &UNative::from(1u8);
    assert_eq!(s, UNative::from(1u8));
}

#[test]
fn sum() {
    let xs = [
        UNative::from(1u8),
        UNative::from(2u8),
        UNative::from(3u8),
        UNative::from(4u8),
    ];
    assert_eq!(xs.into_iter().sum::<UNative>(), UNative::from(10u8));
    assert_eq!(xs.iter().sum::<UNative>(), UNative::from(10u8));
    assert_eq!(
        core::iter::empty::<UNative>().sum::<UNative>(),
        UNative::ZERO,
    );
}

#[test]
fn product() {
    let xs = [UNative::from(2u8), UNative::from(3u8), UNative::from(4u8)];
    assert_eq!(xs.into_iter().product::<UNative>(), UNative::from(24u8));
    assert_eq!(xs.iter().product::<UNative>(), UNative::from(24u8));
    assert_eq!(
        core::iter::empty::<UNative>().product::<UNative>(),
        UNative::from(1u8),
    );
}

#[test]
fn formatting() {
    assert_eq!(format!("{}", UNative::from(42u8)), "42");
    assert_eq!(format!("{:?}", UNative::from(42u8)), "42");
    assert_eq!(format!("{:b}", UNative::from(5u8)), "101");
    assert_eq!(format!("{:o}", UNative::from(8u8)), "10");
    assert_eq!(format!("{:x}", UNative::from(255u8)), "ff");
    assert_eq!(format!("{:X}", UNative::from(255u8)), "FF");

    assert_eq!(format!("{:5}", UNative::from(42u8)), "   42");
    assert_eq!(format!("{:<5}", UNative::from(42u8)), "42   ");
    assert_eq!(format!("{:05}", UNative::from(42u8)), "00042");
    assert_eq!(format!("{:*^5}", UNative::from(42u8)), "*42**");
    assert_eq!(format!("{:#b}", UNative::from(5u8)), "0b101");
    assert_eq!(format!("{:#x}", UNative::from(255u8)), "0xff");
}

#[test]
fn from_str() {
    assert_eq!("42".parse::<UNative>(), Ok(UNative::from(42u8)));
    assert_eq!("0".parse::<UNative>(), Ok(UNative::ZERO));
    assert_eq!(
        format!("{}", UNative::MAX).parse::<UNative>(),
        Ok(UNative::MAX),
    );
    assert!("abc".parse::<UNative>().is_err());
    assert!("-1".parse::<UNative>().is_err());
    assert!("".parse::<UNative>().is_err());
}

#[test]
fn checked_add() {
    assert_eq!(
        UNative::from(2u8).checked_add(UNative::from(3u8)),
        Some(UNative::from(5u8)),
    );
    assert_eq!(UNative::MAX.checked_add(UNative::from(1u8)), None);
    assert_eq!(UNative::MAX.checked_add(UNative::ZERO), Some(UNative::MAX));
}

#[test]
fn checked_add_signed() {
    assert_eq!(
        UNative::from(5u8).checked_add_signed(INative::from(3i8)),
        Some(UNative::from(8u8)),
    );
    assert_eq!(
        UNative::from(5u8).checked_add_signed(INative::from(-3i8)),
        Some(UNative::from(2u8)),
    );
    assert_eq!(
        UNative::from(5u8).checked_add_signed(INative::from(-10i8)),
        None,
    );
    assert_eq!(UNative::MAX.checked_add_signed(INative::from(1i8)), None);
}

#[test]
fn checked_sub() {
    assert_eq!(
        UNative::from(5u8).checked_sub(UNative::from(3u8)),
        Some(UNative::from(2u8)),
    );
    assert_eq!(UNative::ZERO.checked_sub(UNative::from(1u8)), None);
    assert_eq!(UNative::MAX.checked_sub(UNative::ZERO), Some(UNative::MAX));
}

#[test]
fn checked_sub_signed() {
    assert_eq!(
        UNative::from(5u8).checked_sub_signed(INative::from(3i8)),
        Some(UNative::from(2u8)),
    );
    assert_eq!(
        UNative::from(5u8).checked_sub_signed(INative::from(-3i8)),
        Some(UNative::from(8u8)),
    );
    assert_eq!(
        UNative::from(5u8).checked_sub_signed(INative::from(10i8)),
        None,
    );
    assert_eq!(UNative::MAX.checked_sub_signed(INative::from(-1i8)), None);
}

#[test]
fn checked_signed_diff() {
    assert_eq!(
        UNative::from(5u8).checked_signed_diff(UNative::from(3u8)),
        Some(INative::from(2i8)),
    );
    assert_eq!(
        UNative::from(3u8).checked_signed_diff(UNative::from(5u8)),
        Some(INative::from(-2i8)),
    );
    assert_eq!(
        UNative::ZERO.checked_signed_diff(UNative::ZERO),
        Some(INative::ZERO),
    );
    assert_eq!(UNative::MAX.checked_signed_diff(UNative::ZERO), None);
    assert_eq!(UNative::ZERO.checked_signed_diff(UNative::MAX), None);
}

#[test]
fn checked_neg() {
    assert_eq!(UNative::ZERO.checked_neg(), Some(UNative::ZERO));
    assert_eq!(UNative::from(1u8).checked_neg(), None);
    assert_eq!(UNative::MAX.checked_neg(), None);
}

#[test]
fn checked_mul() {
    assert_eq!(
        UNative::from(4u8).checked_mul(UNative::from(3u8)),
        Some(UNative::from(12u8)),
    );
    assert_eq!(UNative::MAX.checked_mul(UNative::from(2u8)), None);
    assert_eq!(UNative::ZERO.checked_mul(UNative::MAX), Some(UNative::ZERO));
}

#[test]
fn checked_div() {
    assert_eq!(
        UNative::from(23u8).checked_div(UNative::from(10u8)),
        Some(UNative::from(2u8)),
    );
    assert_eq!(
        UNative::ZERO.checked_div(UNative::from(5u8)),
        Some(UNative::ZERO)
    );
    assert_eq!(UNative::from(5u8).checked_div(UNative::ZERO), None);
}

#[test]
fn checked_div_euclid() {
    assert_eq!(
        UNative::from(23u8).checked_div_euclid(UNative::from(10u8)),
        Some(UNative::from(2u8)),
    );
    assert_eq!(UNative::from(5u8).checked_div_euclid(UNative::ZERO), None);
}

#[test]
fn checked_rem() {
    assert_eq!(
        UNative::from(23u8).checked_rem(UNative::from(10u8)),
        Some(UNative::from(3u8)),
    );
    assert_eq!(UNative::from(5u8).checked_rem(UNative::ZERO), None);
}

#[test]
fn checked_rem_euclid() {
    assert_eq!(
        UNative::from(23u8).checked_rem_euclid(UNative::from(10u8)),
        Some(UNative::from(3u8)),
    );
    assert_eq!(UNative::from(5u8).checked_rem_euclid(UNative::ZERO), None);
}

#[test]
fn checked_shl() {
    assert_eq!(UNative::from(1u8).checked_shl(3), Some(UNative::from(8u8)),);
    assert_eq!(
        UNative::from(1u8).checked_shl(UNative::BITS - 1),
        Some(UNative::from(1u8) << (UNative::BITS - 1)),
    );
    assert_eq!(UNative::from(1u8).checked_shl(UNative::BITS), None);
}

#[test]
fn checked_shr() {
    assert_eq!(UNative::from(8u8).checked_shr(3), Some(UNative::from(1u8)),);
    assert_eq!(UNative::from(8u8).checked_shr(UNative::BITS), None);
}

#[test]
fn checked_ilog() {
    assert_eq!(
        UNative::from(100u8).checked_ilog(UNative::from(10u8)),
        Some(2),
    );
    assert_eq!(UNative::ZERO.checked_ilog(UNative::from(10u8)), None);
    assert_eq!(UNative::from(10u8).checked_ilog(UNative::from(1u8)), None);
}

#[test]
fn checked_ilog2() {
    assert_eq!(UNative::from(4u8).checked_ilog2(), Some(2));
    assert_eq!(UNative::from(1u8).checked_ilog2(), Some(0));
    assert_eq!(UNative::ZERO.checked_ilog2(), None);
}

#[test]
fn checked_ilog10() {
    assert_eq!(UNative::from(100u8).checked_ilog10(), Some(2));
    assert_eq!(UNative::from(1u8).checked_ilog10(), Some(0));
    assert_eq!(UNative::ZERO.checked_ilog10(), None);
}

#[test]
fn checked_pow() {
    assert_eq!(
        UNative::from(2u8).checked_pow(10),
        Some(UNative::from(1024u16)),
    );
    assert_eq!(UNative::from(3u8).checked_pow(0), Some(UNative::from(1u8)));
    assert_eq!(UNative::MAX.checked_pow(2), None);
}

#[test]
fn checked_next_multiple_of() {
    assert_eq!(
        UNative::from(7u8).checked_next_multiple_of(UNative::from(3u8)),
        Some(UNative::from(9u8)),
    );
    assert_eq!(
        UNative::from(6u8).checked_next_multiple_of(UNative::from(3u8)),
        Some(UNative::from(6u8)),
    );
    assert_eq!(
        UNative::from(5u8).checked_next_multiple_of(UNative::ZERO),
        None,
    );
    assert_eq!(
        UNative::MAX.checked_next_multiple_of(UNative::from(2u8)),
        None
    );
}

#[test]
fn checked_next_power_of_two() {
    assert_eq!(
        UNative::ZERO.checked_next_power_of_two(),
        Some(UNative::from(1u8)),
    );
    assert_eq!(
        UNative::from(5u8).checked_next_power_of_two(),
        Some(UNative::from(8u8)),
    );
    assert_eq!(UNative::MAX.checked_next_power_of_two(), None);
}

#[test]
fn overflowing_add() {
    assert_eq!(
        UNative::from(2u8).overflowing_add(UNative::from(3u8)),
        (UNative::from(5u8), false),
    );
    assert_eq!(
        UNative::MAX.overflowing_add(UNative::from(1u8)),
        (UNative::ZERO, true),
    );
    assert_eq!(
        UNative::MAX.overflowing_add(UNative::MAX),
        (UNative::MAX - UNative::from(1u8), true),
    );
}

#[test]
fn overflowing_add_signed() {
    assert_eq!(
        UNative::from(5u8).overflowing_add_signed(INative::from(3i8)),
        (UNative::from(8u8), false),
    );
    assert_eq!(
        UNative::from(5u8).overflowing_add_signed(INative::from(-3i8)),
        (UNative::from(2u8), false),
    );
    assert_eq!(
        UNative::from(5u8).overflowing_add_signed(INative::from(-10i8)),
        (UNative::MAX - UNative::from(4u8), true),
    );
    assert_eq!(
        UNative::MAX.overflowing_add_signed(INative::from(1i8)),
        (UNative::ZERO, true),
    );
}

#[test]
fn overflowing_sub() {
    assert_eq!(
        UNative::from(5u8).overflowing_sub(UNative::from(3u8)),
        (UNative::from(2u8), false),
    );
    assert_eq!(
        UNative::ZERO.overflowing_sub(UNative::from(1u8)),
        (UNative::MAX, true),
    );
}

#[test]
fn overflowing_sub_signed() {
    assert_eq!(
        UNative::from(5u8).overflowing_sub_signed(INative::from(3i8)),
        (UNative::from(2u8), false),
    );
    assert_eq!(
        UNative::from(5u8).overflowing_sub_signed(INative::from(-3i8)),
        (UNative::from(8u8), false),
    );
    assert_eq!(
        UNative::from(5u8).overflowing_sub_signed(INative::from(10i8)),
        (UNative::MAX - UNative::from(4u8), true),
    );
    assert_eq!(
        UNative::MAX.overflowing_sub_signed(INative::from(-1i8)),
        (UNative::ZERO, true),
    );
}

#[test]
fn overflowing_neg() {
    assert_eq!(UNative::ZERO.overflowing_neg(), (UNative::ZERO, false));
    assert_eq!(UNative::from(1u8).overflowing_neg(), (UNative::MAX, true));
    assert_eq!(UNative::MAX.overflowing_neg(), (UNative::from(1u8), true),);
}

#[test]
fn overflowing_mul() {
    assert_eq!(
        UNative::from(4u8).overflowing_mul(UNative::from(3u8)),
        (UNative::from(12u8), false),
    );
    assert_eq!(
        UNative::ZERO.overflowing_mul(UNative::MAX),
        (UNative::ZERO, false),
    );
    // MAX * 2 = MAX - 1 (mod 2^BITS), with overflow.
    assert_eq!(
        UNative::MAX.overflowing_mul(UNative::from(2u8)),
        (UNative::MAX - UNative::from(1u8), true),
    );
}

#[test]
fn overflowing_div() {
    assert_eq!(
        UNative::from(7u8).overflowing_div(UNative::from(2u8)),
        (UNative::from(3u8), false),
    );
    assert_eq!(
        UNative::ZERO.overflowing_div(UNative::from(5u8)),
        (UNative::ZERO, false),
    );
}

#[test]
#[should_panic]
fn overflowing_div_by_zero() {
    let _ = UNative::from(5u8).overflowing_div(UNative::ZERO);
}

#[test]
fn overflowing_div_euclid() {
    assert_eq!(
        UNative::from(7u8).overflowing_div_euclid(UNative::from(2u8)),
        (UNative::from(3u8), false),
    );
}

#[test]
#[should_panic]
fn overflowing_div_euclid_by_zero() {
    let _ = UNative::from(5u8).overflowing_div_euclid(UNative::ZERO);
}

#[test]
fn overflowing_rem() {
    assert_eq!(
        UNative::from(7u8).overflowing_rem(UNative::from(2u8)),
        (UNative::from(1u8), false),
    );
}

#[test]
#[should_panic]
fn overflowing_rem_by_zero() {
    let _ = UNative::from(5u8).overflowing_rem(UNative::ZERO);
}

#[test]
fn overflowing_rem_euclid() {
    assert_eq!(
        UNative::from(7u8).overflowing_rem_euclid(UNative::from(2u8)),
        (UNative::from(1u8), false),
    );
}

#[test]
#[should_panic]
fn overflowing_rem_euclid_by_zero() {
    let _ = UNative::from(5u8).overflowing_rem_euclid(UNative::ZERO);
}

#[test]
fn overflowing_shl() {
    assert_eq!(
        UNative::from(1u8).overflowing_shl(3),
        (UNative::from(8u8), false),
    );
    assert_eq!(
        UNative::from(1u8).overflowing_shl(UNative::BITS),
        (UNative::from(1u8), true),
    );
    assert_eq!(
        UNative::from(1u8).overflowing_shl(UNative::BITS + 3),
        (UNative::from(8u8), true),
    );
}

#[test]
fn overflowing_shr() {
    assert_eq!(
        UNative::from(8u8).overflowing_shr(3),
        (UNative::from(1u8), false),
    );
    assert_eq!(
        UNative::from(8u8).overflowing_shr(UNative::BITS),
        (UNative::from(8u8), true),
    );
}

#[test]
fn overflowing_pow() {
    assert_eq!(
        UNative::from(2u8).overflowing_pow(10),
        (UNative::from(1024u16), false),
    );
    assert_eq!(
        UNative::from(3u8).overflowing_pow(0),
        (UNative::from(1u8), false),
    );
    // MAX^2 = (2^BITS - 1)^2 = 2^(2*BITS) - 2^(BITS+1) + 1, which wraps to 1.
    assert_eq!(UNative::MAX.overflowing_pow(2), (UNative::from(1u8), true),);
    // 2^BITS wraps to 0.
    assert_eq!(
        UNative::from(2u8).overflowing_pow(UNative::BITS),
        (UNative::ZERO, true),
    );
}

#[test]
fn saturating_add() {
    assert_eq!(
        UNative::from(2u8).saturating_add(UNative::from(3u8)),
        UNative::from(5u8),
    );
    assert_eq!(
        UNative::MAX.saturating_add(UNative::from(1u8)),
        UNative::MAX
    );
    assert_eq!(UNative::MAX.saturating_add(UNative::MAX), UNative::MAX);
}

#[test]
fn saturating_add_signed() {
    assert_eq!(
        UNative::from(5u8).saturating_add_signed(INative::from(3i8)),
        UNative::from(8u8),
    );
    assert_eq!(
        UNative::from(5u8).saturating_add_signed(INative::from(-3i8)),
        UNative::from(2u8),
    );
    assert_eq!(
        UNative::from(5u8).saturating_add_signed(INative::from(-10i8)),
        UNative::ZERO,
    );
    assert_eq!(
        UNative::MAX.saturating_add_signed(INative::from(1i8)),
        UNative::MAX,
    );
}

#[test]
fn saturating_sub() {
    assert_eq!(
        UNative::from(5u8).saturating_sub(UNative::from(3u8)),
        UNative::from(2u8),
    );
    assert_eq!(
        UNative::ZERO.saturating_sub(UNative::from(1u8)),
        UNative::ZERO
    );
    assert_eq!(UNative::ZERO.saturating_sub(UNative::MAX), UNative::ZERO);
}

#[test]
fn saturating_sub_signed() {
    assert_eq!(
        UNative::from(5u8).saturating_sub_signed(INative::from(3i8)),
        UNative::from(2u8),
    );
    assert_eq!(
        UNative::from(5u8).saturating_sub_signed(INative::from(-3i8)),
        UNative::from(8u8),
    );
    assert_eq!(
        UNative::from(5u8).saturating_sub_signed(INative::from(10i8)),
        UNative::ZERO,
    );
    assert_eq!(
        UNative::MAX.saturating_sub_signed(INative::from(-1i8)),
        UNative::MAX,
    );
}

#[test]
fn saturating_mul() {
    assert_eq!(
        UNative::from(4u8).saturating_mul(UNative::from(3u8)),
        UNative::from(12u8),
    );
    assert_eq!(UNative::ZERO.saturating_mul(UNative::MAX), UNative::ZERO);
    assert_eq!(
        UNative::MAX.saturating_mul(UNative::from(2u8)),
        UNative::MAX
    );
    assert_eq!(UNative::MAX.saturating_mul(UNative::MAX), UNative::MAX);
}

#[test]
fn saturating_div() {
    assert_eq!(
        UNative::from(7u8).saturating_div(UNative::from(2u8)),
        UNative::from(3u8),
    );
    assert_eq!(
        UNative::ZERO.saturating_div(UNative::from(5u8)),
        UNative::ZERO
    );
}

#[test]
#[should_panic]
fn saturating_div_by_zero() {
    let _ = UNative::from(5u8).saturating_div(UNative::ZERO);
}

#[test]
fn saturating_pow() {
    assert_eq!(
        UNative::from(2u8).saturating_pow(10),
        UNative::from(1024u16),
    );
    assert_eq!(UNative::from(3u8).saturating_pow(0), UNative::from(1u8));
    assert_eq!(UNative::MAX.saturating_pow(2), UNative::MAX);
    assert_eq!(
        UNative::from(2u8).saturating_pow(UNative::BITS),
        UNative::MAX
    );
}

#[test]
fn wrapping_add() {
    assert_eq!(
        UNative::from(2u8).wrapping_add(UNative::from(3u8)),
        UNative::from(5u8),
    );
    assert_eq!(UNative::MAX.wrapping_add(UNative::from(1u8)), UNative::ZERO);
    assert_eq!(
        UNative::MAX.wrapping_add(UNative::MAX),
        UNative::MAX - UNative::from(1u8),
    );
}

#[test]
fn wrapping_add_signed() {
    assert_eq!(
        UNative::from(5u8).wrapping_add_signed(INative::from(3i8)),
        UNative::from(8u8),
    );
    assert_eq!(
        UNative::from(5u8).wrapping_add_signed(INative::from(-3i8)),
        UNative::from(2u8),
    );
    assert_eq!(
        UNative::from(5u8).wrapping_add_signed(INative::from(-10i8)),
        UNative::MAX - UNative::from(4u8),
    );
    assert_eq!(
        UNative::MAX.wrapping_add_signed(INative::from(1i8)),
        UNative::ZERO,
    );
}

#[test]
fn wrapping_sub() {
    assert_eq!(
        UNative::from(5u8).wrapping_sub(UNative::from(3u8)),
        UNative::from(2u8),
    );
    assert_eq!(UNative::ZERO.wrapping_sub(UNative::from(1u8)), UNative::MAX);
    assert_eq!(UNative::ZERO.wrapping_sub(UNative::MAX), UNative::from(1u8));
}

#[test]
fn wrapping_sub_signed() {
    assert_eq!(
        UNative::from(5u8).wrapping_sub_signed(INative::from(3i8)),
        UNative::from(2u8),
    );
    assert_eq!(
        UNative::from(5u8).wrapping_sub_signed(INative::from(-3i8)),
        UNative::from(8u8),
    );
    assert_eq!(
        UNative::from(5u8).wrapping_sub_signed(INative::from(10i8)),
        UNative::MAX - UNative::from(4u8),
    );
    assert_eq!(
        UNative::MAX.wrapping_sub_signed(INative::from(-1i8)),
        UNative::ZERO,
    );
}

#[test]
fn wrapping_neg() {
    assert_eq!(UNative::ZERO.wrapping_neg(), UNative::ZERO);
    assert_eq!(UNative::from(1u8).wrapping_neg(), UNative::MAX);
    assert_eq!(UNative::MAX.wrapping_neg(), UNative::from(1u8));
}

#[test]
fn wrapping_mul() {
    assert_eq!(
        UNative::from(4u8).wrapping_mul(UNative::from(3u8)),
        UNative::from(12u8),
    );
    assert_eq!(UNative::ZERO.wrapping_mul(UNative::MAX), UNative::ZERO);
    // MAX * 2 = -2 (mod 2^BITS) = MAX - 1.
    assert_eq!(
        UNative::MAX.wrapping_mul(UNative::from(2u8)),
        UNative::MAX - UNative::from(1u8),
    );
    // MAX * MAX = 1 (mod 2^BITS).
    assert_eq!(UNative::MAX.wrapping_mul(UNative::MAX), UNative::from(1u8));
}

#[test]
fn wrapping_div() {
    assert_eq!(
        UNative::from(7u8).wrapping_div(UNative::from(2u8)),
        UNative::from(3u8),
    );
}

#[test]
#[should_panic]
fn wrapping_div_by_zero() {
    let _ = UNative::from(5u8).wrapping_div(UNative::ZERO);
}

#[test]
fn wrapping_div_euclid() {
    assert_eq!(
        UNative::from(7u8).wrapping_div_euclid(UNative::from(2u8)),
        UNative::from(3u8),
    );
}

#[test]
#[should_panic]
fn wrapping_div_euclid_by_zero() {
    let _ = UNative::from(5u8).wrapping_div_euclid(UNative::ZERO);
}

#[test]
fn wrapping_rem() {
    assert_eq!(
        UNative::from(7u8).wrapping_rem(UNative::from(2u8)),
        UNative::from(1u8),
    );
}

#[test]
#[should_panic]
fn wrapping_rem_by_zero() {
    let _ = UNative::from(5u8).wrapping_rem(UNative::ZERO);
}

#[test]
fn wrapping_rem_euclid() {
    assert_eq!(
        UNative::from(7u8).wrapping_rem_euclid(UNative::from(2u8)),
        UNative::from(1u8),
    );
}

#[test]
#[should_panic]
fn wrapping_rem_euclid_by_zero() {
    let _ = UNative::from(5u8).wrapping_rem_euclid(UNative::ZERO);
}

#[test]
fn wrapping_shl() {
    assert_eq!(UNative::from(1u8).wrapping_shl(3), UNative::from(8u8));
    // Shift amount is masked modulo BITS.
    assert_eq!(
        UNative::from(1u8).wrapping_shl(UNative::BITS),
        UNative::from(1u8),
    );
    assert_eq!(
        UNative::from(1u8).wrapping_shl(UNative::BITS + 3),
        UNative::from(8u8),
    );
}

#[test]
fn wrapping_shr() {
    assert_eq!(UNative::from(8u8).wrapping_shr(3), UNative::from(1u8));
    assert_eq!(
        UNative::from(8u8).wrapping_shr(UNative::BITS),
        UNative::from(8u8),
    );
}

#[test]
fn wrapping_pow() {
    assert_eq!(UNative::from(2u8).wrapping_pow(10), UNative::from(1024u16),);
    assert_eq!(UNative::from(3u8).wrapping_pow(0), UNative::from(1u8));
    // MAX^2 = 1 (mod 2^BITS).
    assert_eq!(UNative::MAX.wrapping_pow(2), UNative::from(1u8));
    // 2^BITS wraps to 0.
    assert_eq!(
        UNative::from(2u8).wrapping_pow(UNative::BITS),
        UNative::ZERO
    );
}

#[test]
fn strict_add() {
    assert_eq!(
        UNative::from(2u8).strict_add(UNative::from(3u8)),
        UNative::from(5u8),
    );
}

#[test]
#[should_panic]
fn strict_add_overflow() {
    let _ = UNative::MAX.strict_add(UNative::from(1u8));
}

#[test]
fn strict_add_signed() {
    assert_eq!(
        UNative::from(5u8).strict_add_signed(INative::from(3i8)),
        UNative::from(8u8),
    );
    assert_eq!(
        UNative::from(5u8).strict_add_signed(INative::from(-3i8)),
        UNative::from(2u8),
    );
}

#[test]
#[should_panic]
fn strict_add_signed_overflow_above() {
    let _ = UNative::MAX.strict_add_signed(INative::from(1i8));
}

#[test]
#[should_panic]
fn strict_add_signed_overflow_below() {
    let _ = UNative::from(5u8).strict_add_signed(INative::from(-10i8));
}

#[test]
fn strict_sub() {
    assert_eq!(
        UNative::from(5u8).strict_sub(UNative::from(3u8)),
        UNative::from(2u8),
    );
}

#[test]
#[should_panic]
fn strict_sub_overflow() {
    let _ = UNative::ZERO.strict_sub(UNative::from(1u8));
}

#[test]
fn strict_sub_signed() {
    assert_eq!(
        UNative::from(5u8).strict_sub_signed(INative::from(3i8)),
        UNative::from(2u8),
    );
    assert_eq!(
        UNative::from(5u8).strict_sub_signed(INative::from(-3i8)),
        UNative::from(8u8),
    );
}

#[test]
#[should_panic]
fn strict_sub_signed_overflow_below() {
    let _ = UNative::from(5u8).strict_sub_signed(INative::from(10i8));
}

#[test]
#[should_panic]
fn strict_sub_signed_overflow_above() {
    let _ = UNative::MAX.strict_sub_signed(INative::from(-1i8));
}

#[test]
fn strict_neg() {
    assert_eq!(UNative::ZERO.strict_neg(), UNative::ZERO);
}

#[test]
#[should_panic]
fn strict_neg_overflow() {
    let _ = UNative::from(1u8).strict_neg();
}

#[test]
fn strict_mul() {
    assert_eq!(
        UNative::from(4u8).strict_mul(UNative::from(3u8)),
        UNative::from(12u8),
    );
}

#[test]
#[should_panic]
fn strict_mul_overflow() {
    let _ = UNative::MAX.strict_mul(UNative::from(2u8));
}

#[test]
fn strict_div() {
    assert_eq!(
        UNative::from(23u8).strict_div(UNative::from(10u8)),
        UNative::from(2u8),
    );
}

#[test]
#[should_panic]
fn strict_div_by_zero() {
    let _ = UNative::from(5u8).strict_div(UNative::ZERO);
}

#[test]
fn strict_div_euclid() {
    assert_eq!(
        UNative::from(23u8).strict_div_euclid(UNative::from(10u8)),
        UNative::from(2u8),
    );
}

#[test]
#[should_panic]
fn strict_div_euclid_by_zero() {
    let _ = UNative::from(5u8).strict_div_euclid(UNative::ZERO);
}

#[test]
fn strict_rem() {
    assert_eq!(
        UNative::from(23u8).strict_rem(UNative::from(10u8)),
        UNative::from(3u8),
    );
}

#[test]
#[should_panic]
fn strict_rem_by_zero() {
    let _ = UNative::from(5u8).strict_rem(UNative::ZERO);
}

#[test]
fn strict_rem_euclid() {
    assert_eq!(
        UNative::from(23u8).strict_rem_euclid(UNative::from(10u8)),
        UNative::from(3u8),
    );
}

#[test]
#[should_panic]
fn strict_rem_euclid_by_zero() {
    let _ = UNative::from(5u8).strict_rem_euclid(UNative::ZERO);
}

#[test]
fn strict_shl() {
    assert_eq!(UNative::from(1u8).strict_shl(3), UNative::from(8u8),);
}

#[test]
#[should_panic]
fn strict_shl_overflow() {
    let _ = UNative::from(1u8).strict_shl(UNative::BITS);
}

#[test]
fn strict_shr() {
    assert_eq!(UNative::from(8u8).strict_shr(3), UNative::from(1u8),);
}

#[test]
#[should_panic]
fn strict_shr_overflow() {
    let _ = UNative::from(8u8).strict_shr(UNative::BITS);
}

#[test]
fn strict_pow() {
    assert_eq!(UNative::from(2u8).strict_pow(10), UNative::from(1024u16));
    assert_eq!(UNative::from(3u8).strict_pow(0), UNative::from(1u8));
}

#[test]
#[should_panic]
fn strict_pow_overflow() {
    let _ = UNative::MAX.strict_pow(2);
}

#[test]
fn unchecked_add() {
    // SAFETY: 2 + 3 doesn't overflow.
    let result = unsafe { UNative::from(2u8).unchecked_add(UNative::from(3u8)) };
    assert_eq!(result, UNative::from(5u8));
}

#[test]
fn unchecked_sub() {
    // SAFETY: 5 - 3 doesn't overflow.
    let result = unsafe { UNative::from(5u8).unchecked_sub(UNative::from(3u8)) };
    assert_eq!(result, UNative::from(2u8));
}

#[test]
fn unchecked_mul() {
    // SAFETY: 4 * 3 doesn't overflow.
    let result = unsafe { UNative::from(4u8).unchecked_mul(UNative::from(3u8)) };
    assert_eq!(result, UNative::from(12u8));
}

#[test]
fn unchecked_shl() {
    // SAFETY: 3 < BITS.
    let result = unsafe { UNative::from(1u8).unchecked_shl(3) };
    assert_eq!(result, UNative::from(8u8));
}

#[test]
fn unchecked_shr() {
    // SAFETY: 3 < BITS.
    let result = unsafe { UNative::from(8u8).unchecked_shr(3) };
    assert_eq!(result, UNative::from(1u8));
}

#[test]
fn unbounded_shl() {
    assert_eq!(UNative::from(1u8).unbounded_shl(3), UNative::from(8u8));
    assert_eq!(
        UNative::from(1u8).unbounded_shl(UNative::BITS),
        UNative::ZERO
    );
    assert_eq!(
        UNative::from(1u8).unbounded_shl(UNative::BITS + 100),
        UNative::ZERO,
    );
}

#[test]
fn unbounded_shr() {
    assert_eq!(UNative::from(8u8).unbounded_shr(3), UNative::from(1u8));
    assert_eq!(UNative::MAX.unbounded_shr(UNative::BITS), UNative::ZERO);
    assert_eq!(
        UNative::MAX.unbounded_shr(UNative::BITS + 100),
        UNative::ZERO
    );
}

#[test]
fn borrowing_sub() {
    assert_eq!(
        UNative::from(5u8).borrowing_sub(UNative::from(3u8), false),
        (UNative::from(2u8), false),
    );
    assert_eq!(
        UNative::from(5u8).borrowing_sub(UNative::from(3u8), true),
        (UNative::from(1u8), false),
    );
    assert_eq!(
        UNative::ZERO.borrowing_sub(UNative::ZERO, true),
        (UNative::MAX, true),
    );
    assert_eq!(
        UNative::ZERO.borrowing_sub(UNative::from(1u8), false),
        (UNative::MAX, true),
    );
    assert_eq!(
        UNative::ZERO.borrowing_sub(UNative::MAX, true),
        (UNative::ZERO, true),
    );
}

#[test]
fn carrying_add() {
    assert_eq!(
        UNative::from(2u8).carrying_add(UNative::from(3u8), false),
        (UNative::from(5u8), false),
    );
    assert_eq!(
        UNative::from(2u8).carrying_add(UNative::from(3u8), true),
        (UNative::from(6u8), false),
    );
    assert_eq!(
        UNative::MAX.carrying_add(UNative::ZERO, false),
        (UNative::MAX, false),
    );
    assert_eq!(
        UNative::MAX.carrying_add(UNative::ZERO, true),
        (UNative::ZERO, true),
    );
    assert_eq!(
        UNative::MAX.carrying_add(UNative::from(1u8), false),
        (UNative::ZERO, true),
    );
    assert_eq!(
        UNative::MAX.carrying_add(UNative::MAX, true),
        (UNative::MAX, true),
    );
}

#[test]
fn carrying_mul() {
    assert_eq!(
        UNative::from(2u8).carrying_mul(UNative::from(3u8), UNative::ZERO),
        (UNative::from(6u8), UNative::ZERO),
    );
    assert_eq!(
        UNative::from(2u8).carrying_mul(UNative::from(3u8), UNative::from(4u8)),
        (UNative::from(10u8), UNative::ZERO),
    );
    assert_eq!(
        UNative::ZERO.carrying_mul(UNative::MAX, UNative::from(5u8)),
        (UNative::from(5u8), UNative::ZERO),
    );
    // MAX * MAX = (MAX - 1) * 2^BITS + 1
    assert_eq!(
        UNative::MAX.carrying_mul(UNative::MAX, UNative::ZERO),
        (UNative::from(1u8), UNative::MAX - UNative::from(1u8)),
    );
    // MAX * MAX + MAX = MAX * (MAX + 1) = MAX * 2^BITS
    assert_eq!(
        UNative::MAX.carrying_mul(UNative::MAX, UNative::MAX),
        (UNative::ZERO, UNative::MAX),
    );
}

#[test]
fn carrying_mul_add() {
    assert_eq!(
        UNative::from(2u8).carrying_mul_add(
            UNative::from(3u8),
            UNative::from(4u8),
            UNative::from(5u8),
        ),
        (UNative::from(15u8), UNative::ZERO),
    );
    assert_eq!(
        UNative::ZERO.carrying_mul_add(UNative::ZERO, UNative::ZERO, UNative::ZERO),
        (UNative::ZERO, UNative::ZERO),
    );
    // MAX * MAX + MAX + MAX = MAX * (MAX + 2) = MAX * 2^BITS + MAX
    assert_eq!(
        UNative::MAX.carrying_mul_add(UNative::MAX, UNative::MAX, UNative::MAX),
        (UNative::MAX, UNative::MAX),
    );
}

#[test]
fn const_eq() {
    // Bind to locals so the `const` evaluation is exercised without tripping
    // clippy's `assertions_on_constants` lint.
    const A: bool = UNative::ZERO.const_eq(UNative::ZERO);
    const B: bool = UNative::ZERO.const_eq(UNative::MAX);
    let (a, b) = (A, B);
    assert!(a);
    assert!(!b);
    assert!(UNative::from(7u8).const_eq(UNative::from(7u8)));
    assert!(!UNative::from(7u8).const_eq(UNative::from(8u8)));
}

#[test]
fn const_cmp() {
    const C: Ordering = UNative::ZERO.const_cmp(UNative::MAX);
    let c = C;
    assert_eq!(c, Ordering::Less);
    assert_eq!(
        UNative::from(7u8).const_cmp(UNative::from(7u8)),
        Ordering::Equal
    );
    assert_eq!(UNative::MAX.const_cmp(UNative::ZERO), Ordering::Greater);
}

#[test]
fn const_ordering_predicates() {
    let lo = UNative::from(3u8);
    let hi = UNative::from(7u8);
    assert!(lo.const_lt(hi) && !hi.const_lt(lo) && !lo.const_lt(lo));
    assert!(lo.const_le(hi) && lo.const_le(lo) && !hi.const_le(lo));
    assert!(hi.const_gt(lo) && !lo.const_gt(hi) && !hi.const_gt(hi));
    assert!(hi.const_ge(lo) && hi.const_ge(hi) && !lo.const_ge(hi));
}

#[test]
fn const_min_max() {
    const MIN: UNative = UNative::ZERO.const_min(UNative::MAX);
    const MAX: UNative = UNative::ZERO.const_max(UNative::MAX);
    assert_eq!(MIN, UNative::ZERO);
    assert_eq!(MAX, UNative::MAX);
    let x = UNative::from(7u8);
    assert_eq!(x.const_min(x), x);
    assert_eq!(x.const_max(x), x);
}

#[test]
fn const_clamp() {
    const C: UNative = UNative::from_u8(20).const_clamp(UNative::from_u8(5), UNative::from_u8(10));
    assert_eq!(C, UNative::from(10u8));
    let lo = UNative::from(5u8);
    let hi = UNative::from(10u8);
    assert_eq!(UNative::from(3u8).const_clamp(lo, hi), lo);
    assert_eq!(UNative::from(7u8).const_clamp(lo, hi), UNative::from(7u8));
    assert_eq!(UNative::from(15u8).const_clamp(lo, hi), hi);
}

#[test]
#[should_panic]
fn const_clamp_min_greater_than_max() {
    let _ = UNative::from(7u8).const_clamp(UNative::from(10u8), UNative::from(5u8));
}

#[test]
fn const_bit_ops() {
    let a = UNative::from(0b1100u8);
    let b = UNative::from(0b1010u8);
    assert_eq!(a.const_bitand(b), UNative::from(0b1000u8));
    assert_eq!(a.const_bitor(b), UNative::from(0b1110u8));
    assert_eq!(a.const_bitxor(b), UNative::from(0b0110u8));
    assert_eq!(UNative::ZERO.const_not(), UNative::MAX);
    assert_eq!(UNative::MAX.const_not(), UNative::ZERO);
}

#[test]
fn from_str_radix() {
    assert_eq!(UNative::from_str_radix("42", 10), Ok(UNative::from(42u8)));
    assert_eq!(UNative::from_str_radix("+42", 10), Ok(UNative::from(42u8)));
    assert_eq!(UNative::from_str_radix("0", 10), Ok(UNative::ZERO));
    assert_eq!(UNative::from_str_radix("ff", 16), Ok(UNative::from(255u8)));
    assert_eq!(UNative::from_str_radix("FF", 16), Ok(UNative::from(255u8)));
    assert_eq!(UNative::from_str_radix("101", 2), Ok(UNative::from(5u8)));
    assert_eq!(UNative::from_str_radix("z", 36), Ok(UNative::from(35u8)));
    assert_eq!(
        UNative::from_str_radix(&format!("{}", UNative::MAX), 10),
        Ok(UNative::MAX),
    );
    assert!(UNative::from_str_radix("-1", 10).is_err());
    assert!(UNative::from_str_radix("", 10).is_err());
    assert!(UNative::from_str_radix("g", 16).is_err());
    assert!(UNative::from_str_radix(" 1", 10).is_err());
}

#[test]
#[should_panic]
fn from_str_radix_invalid_radix_low() {
    let _ = UNative::from_str_radix("0", 1);
}

#[test]
#[should_panic]
fn from_str_radix_invalid_radix_high() {
    let _ = UNative::from_str_radix("0", 37);
}
