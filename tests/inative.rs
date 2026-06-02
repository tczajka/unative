#![allow(clippy::op_ref)]

use core::cmp::Ordering;
use unative::{INative, UNative};

#[test]
fn from() {
    assert_eq!(i128::from(INative::from(false)), 0i128);
    assert_eq!(i128::from(INative::from(true)), 1i128);
    assert_eq!(i128::from(INative::from(u8::MAX)), i128::from(u8::MAX));
    assert_eq!(i128::from(INative::from(i8::MIN)), i128::from(i8::MIN));
    assert_eq!(i128::from(INative::from(i8::MAX)), i128::from(i8::MAX));
    assert_eq!(i128::from(INative::from(i16::MIN)), i128::from(i16::MIN));
    assert_eq!(i128::from(INative::from(i16::MAX)), i128::from(i16::MAX));

    assert_eq!(i64::from(INative::ZERO), 0i64);
    assert_eq!(i64::from(INative::from(42i8)), 42i64);
    assert_eq!(i64::from(INative::from(-1i8)), -1i64);
    assert_eq!(
        i128::from(i64::from(INative::MAX)),
        i128::from(INative::MAX)
    );
    assert_eq!(
        i128::from(i64::from(INative::MIN)),
        i128::from(INative::MIN)
    );
}

#[test]
fn try_from() {
    assert_eq!(INative::try_from(0u32), Ok(INative::ZERO));
    assert_eq!(INative::try_from(-1i32), Ok(INative::from(-1i8)));

    let max = i128::from(INative::MAX);
    let min = i128::from(INative::MIN);
    assert_eq!(INative::try_from(max), Ok(INative::MAX));
    assert_eq!(INative::try_from(min), Ok(INative::MIN));
    assert!(INative::try_from(max + 1).is_err());
    assert!(INative::try_from(min - 1).is_err());

    let neg = INative::from(-1i8);
    assert_eq!(i8::try_from(neg), Ok(-1i8));
    assert!(u8::try_from(neg).is_err());

    assert_eq!(
        INative::try_from(UNative::from(42u8)),
        Ok(INative::from(42i8))
    );
    assert!(INative::try_from(UNative::MAX).is_err());
}

#[test]
fn const_conversions() {
    // Usable in const context.
    const NEG_ONE: INative = INative::from_i8(-1);
    assert_eq!(NEG_ONE, INative::from(-1i8));

    // Infallible `from_*`.
    assert_eq!(INative::from_i8(0), INative::ZERO);
    assert_eq!(INative::from_i8(i8::MIN), INative::from(i8::MIN));
    assert_eq!(INative::from_i16(i16::MAX), INative::from(i16::MAX));

    // Fallible `from_*`.
    let max = i128::from(INative::MAX);
    let min = i128::from(INative::MIN);
    assert_eq!(INative::try_from_i32(-42), Some(INative::from(-42i8)));
    assert_eq!(INative::try_from_i64(-42), Some(INative::from(-42i8)));
    assert_eq!(INative::try_from_i128(max), Some(INative::MAX));
    assert_eq!(INative::try_from_i128(min), Some(INative::MIN));
    assert_eq!(INative::try_from_i128(max + 1), None);
    assert_eq!(INative::try_from_i128(min - 1), None);
    assert_eq!(INative::try_from_isize(-42), Some(INative::from(-42i8)));

    // Infallible `to_*`.
    assert_eq!(INative::from(-42i8).to_i64(), -42i64);
    assert_eq!(i128::from(INative::MAX), INative::MAX.to_i128());

    // Fallible `to_*`.
    assert_eq!(INative::from(-42i8).try_to_i8(), Some(-42i8));
    assert_eq!(INative::from(i16::MIN).try_to_i8(), None);
    assert_eq!(INative::from(-42i8).try_to_i16(), Some(-42i16));
    assert_eq!(INative::from(-42i8).try_to_i32(), Some(-42i32));
    assert_eq!(INative::from(-42i8).try_to_isize(), Some(-42isize));
}

#[test]
fn default() {
    assert_eq!(INative::default(), INative::ZERO);
}

#[test]
fn bits() {
    assert_eq!(INative::BITS, UNative::BITS);
    assert_eq!(INative::MAX >> (INative::BITS - 2), INative::from(1i8));
}

#[test]
fn bytes() {
    assert_eq!(INative::BYTES * 8, INative::BITS as usize);
    assert_eq!(INative::BYTES, UNative::BYTES);
}

#[test]
fn be_bytes() {
    let x = INative::from(-42i8);
    assert_eq!(INative::from_be_bytes(x.to_be_bytes()), x);
    let zero_bytes = INative::ZERO.to_be_bytes();
    assert!(zero_bytes.iter().all(|&b| b == 0));
    let neg_one_bytes = INative::from(-1i8).to_be_bytes();
    assert!(neg_one_bytes.iter().all(|&b| b == 0xff));
    let one_bytes = INative::from(1i8).to_be_bytes();
    assert_eq!(one_bytes[INative::BYTES - 1], 1);
    assert!(one_bytes[..INative::BYTES - 1].iter().all(|&b| b == 0));
}

#[test]
fn le_bytes() {
    let x = INative::from(-42i8);
    assert_eq!(INative::from_le_bytes(x.to_le_bytes()), x);
    let one_bytes = INative::from(1i8).to_le_bytes();
    assert_eq!(one_bytes[0], 1);
    assert!(one_bytes[1..].iter().all(|&b| b == 0));
}

#[test]
fn ne_bytes() {
    let x = INative::from(-42i8);
    assert_eq!(INative::from_ne_bytes(x.to_ne_bytes()), x);
}

#[test]
fn midpoint() {
    assert_eq!(
        INative::from(4i8).midpoint(INative::from(10i8)),
        INative::from(7i8),
    );
    assert_eq!(
        INative::from(-4i8).midpoint(INative::from(-10i8)),
        INative::from(-7i8),
    );
    // For signed midpoint, non-exact results round toward zero.
    assert_eq!(
        INative::from(-1i8).midpoint(INative::from(2i8)),
        INative::ZERO,
    );
    assert_eq!(
        INative::from(-2i8).midpoint(INative::from(1i8)),
        INative::ZERO,
    );
    assert_eq!(INative::ZERO.midpoint(INative::ZERO), INative::ZERO);
    // No overflow even at the extremes.
    assert_eq!(INative::MAX.midpoint(INative::MAX), INative::MAX);
    assert_eq!(INative::MIN.midpoint(INative::MIN), INative::MIN);
}

#[test]
fn ilog() {
    assert_eq!(INative::from(100i8).ilog(INative::from(10i8)), 2);
    assert_eq!(INative::from(99i8).ilog(INative::from(10i8)), 1);
}

#[test]
#[should_panic]
fn ilog_of_zero() {
    let _ = INative::ZERO.ilog(INative::from(10i8));
}

#[test]
#[should_panic]
fn ilog_of_negative() {
    let _ = INative::from(-1i8).ilog(INative::from(10i8));
}

#[test]
#[should_panic]
fn ilog_with_base_1() {
    let _ = INative::from(10i8).ilog(INative::from(1i8));
}

#[test]
fn ilog2() {
    assert_eq!(INative::from(1i8).ilog2(), 0);
    assert_eq!(INative::from(2i8).ilog2(), 1);
    assert_eq!(INative::from(4i8).ilog2(), 2);
    assert_eq!(INative::MAX.ilog2(), INative::BITS - 2);
}

#[test]
#[should_panic]
fn ilog2_of_zero() {
    let _ = INative::ZERO.ilog2();
}

#[test]
#[should_panic]
fn ilog2_of_negative() {
    let _ = INative::from(-1i8).ilog2();
}

#[test]
fn ilog10() {
    assert_eq!(INative::from(1i8).ilog10(), 0);
    assert_eq!(INative::from(100i8).ilog10(), 2);
}

#[test]
#[should_panic]
fn ilog10_of_zero() {
    let _ = INative::ZERO.ilog10();
}

#[test]
#[should_panic]
fn ilog10_of_negative() {
    let _ = INative::from(-1i8).ilog10();
}

#[test]
fn pow() {
    assert_eq!(INative::from(2i8).pow(10), INative::from(1024i16));
    assert_eq!(INative::from(-2i8).pow(3), INative::from(-8i8));
    assert_eq!(INative::from(-2i8).pow(4), INative::from(16i8));
    assert_eq!(INative::from(3i8).pow(0), INative::from(1i8));
}

#[test]
fn isqrt() {
    assert_eq!(INative::ZERO.isqrt(), INative::ZERO);
    assert_eq!(INative::from(1i8).isqrt(), INative::from(1i8));
    assert_eq!(INative::from(4i8).isqrt(), INative::from(2i8));
    assert_eq!(INative::from(8i8).isqrt(), INative::from(2i8));
    assert_eq!(INative::from(100i8).isqrt(), INative::from(10i8));
}

#[test]
#[should_panic]
fn isqrt_of_negative() {
    let _ = INative::from(-1i8).isqrt();
}

#[test]
fn div_euclid() {
    assert_eq!(
        INative::from(23i8).div_euclid(INative::from(10i8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(-23i8).div_euclid(INative::from(10i8)),
        INative::from(-3i8),
    );
    assert_eq!(
        INative::from(23i8).div_euclid(INative::from(-10i8)),
        INative::from(-2i8),
    );
    assert_eq!(
        INative::from(-23i8).div_euclid(INative::from(-10i8)),
        INative::from(3i8),
    );
    assert_eq!(INative::ZERO.div_euclid(INative::from(5i8)), INative::ZERO);
}

#[test]
#[should_panic]
fn div_euclid_by_zero() {
    let _ = INative::from(5i8).div_euclid(INative::ZERO);
}

#[test]
#[should_panic]
fn div_euclid_overflow() {
    let _ = INative::MIN.div_euclid(INative::from(-1i8));
}

#[test]
fn rem_euclid() {
    assert_eq!(
        INative::from(23i8).rem_euclid(INative::from(10i8)),
        INative::from(3i8),
    );
    assert_eq!(
        INative::from(-23i8).rem_euclid(INative::from(10i8)),
        INative::from(7i8),
    );
    assert_eq!(
        INative::from(23i8).rem_euclid(INative::from(-10i8)),
        INative::from(3i8),
    );
    assert_eq!(
        INative::from(-23i8).rem_euclid(INative::from(-10i8)),
        INative::from(7i8),
    );
    assert_eq!(INative::ZERO.rem_euclid(INative::from(5i8)), INative::ZERO);
}

#[test]
#[should_panic]
fn rem_euclid_by_zero() {
    let _ = INative::from(5i8).rem_euclid(INative::ZERO);
}

#[test]
#[should_panic]
fn rem_euclid_overflow() {
    let _ = INative::MIN.rem_euclid(INative::from(-1i8));
}

#[test]
fn count_ones() {
    assert_eq!(INative::ZERO.count_ones(), 0);
    assert_eq!(INative::MAX.count_ones(), INative::BITS - 1);
    assert_eq!(INative::from(-1i8).count_ones(), INative::BITS);
    assert_eq!(INative::from(0b10110i8).count_ones(), 3);
}

#[test]
fn count_zeros() {
    assert_eq!(INative::ZERO.count_zeros(), INative::BITS);
    assert_eq!(INative::MAX.count_zeros(), 1);
    assert_eq!(INative::from(-1i8).count_zeros(), 0);
    let x = INative::from(0b10110i8);
    assert_eq!(x.count_ones() + x.count_zeros(), INative::BITS);
}

#[test]
fn leading_zeros() {
    assert_eq!(INative::ZERO.leading_zeros(), INative::BITS);
    assert_eq!(INative::MAX.leading_zeros(), 1);
    assert_eq!(INative::from(-1i8).leading_zeros(), 0);
    assert_eq!(INative::from(1i8).leading_zeros(), INative::BITS - 1);
}

#[test]
fn trailing_zeros() {
    assert_eq!(INative::ZERO.trailing_zeros(), INative::BITS);
    assert_eq!(INative::MIN.trailing_zeros(), INative::BITS - 1);
    assert_eq!(INative::from(-1i8).trailing_zeros(), 0);
    assert_eq!(INative::from(0b1000i8).trailing_zeros(), 3);
}

#[test]
fn leading_ones() {
    assert_eq!(INative::ZERO.leading_ones(), 0);
    assert_eq!(INative::MAX.leading_ones(), 0);
    assert_eq!(INative::from(-1i8).leading_ones(), INative::BITS);
    assert_eq!(INative::MIN.leading_ones(), 1);
}

#[test]
fn trailing_ones() {
    assert_eq!(INative::ZERO.trailing_ones(), 0);
    assert_eq!(INative::from(-1i8).trailing_ones(), INative::BITS);
    assert_eq!(INative::from(0b1011i8).trailing_ones(), 2);
}

#[test]
fn rotate_left() {
    assert_eq!(INative::ZERO.rotate_left(3), INative::ZERO);
    assert_eq!(INative::from(-1i8).rotate_left(3), INative::from(-1i8));
    assert_eq!(INative::from(42i8).rotate_left(0), INative::from(42i8));
    assert_eq!(INative::from(1i8).rotate_left(3), INative::from(8i8));
    let x = INative::from(42i8);
    assert_eq!(x.rotate_left(INative::BITS), x);
}

#[test]
fn rotate_right() {
    assert_eq!(INative::ZERO.rotate_right(3), INative::ZERO);
    assert_eq!(INative::from(-1i8).rotate_right(3), INative::from(-1i8));
    assert_eq!(INative::from(42i8).rotate_right(0), INative::from(42i8));
    assert_eq!(INative::from(8i8).rotate_right(3), INative::from(1i8));
    let x = INative::from(42i8);
    assert_eq!(x.rotate_right(INative::BITS), x);
    assert_eq!(x.rotate_left(5).rotate_right(5), x);
}

#[test]
fn swap_bytes() {
    assert_eq!(INative::ZERO.swap_bytes(), INative::ZERO);
    assert_eq!(INative::from(-1i8).swap_bytes(), INative::from(-1i8));
    assert_eq!(
        INative::from(1i8).swap_bytes(),
        INative::from(1i8) << (INative::BITS - 8),
    );
    let x = INative::from(42i8);
    assert_eq!(x.swap_bytes().swap_bytes(), x);
}

#[test]
fn reverse_bits() {
    assert_eq!(INative::ZERO.reverse_bits(), INative::ZERO);
    assert_eq!(INative::from(-1i8).reverse_bits(), INative::from(-1i8));
    assert_eq!(INative::from(1i8).reverse_bits(), INative::MIN);
    let x = INative::from(42i8);
    assert_eq!(x.reverse_bits().reverse_bits(), x);
}

#[test]
fn big_endian() {
    let x = INative::from(42i8);
    assert_eq!(INative::from_be(x.to_be()), x);
    assert_eq!(INative::ZERO.to_be(), INative::ZERO);
    assert_eq!(INative::from(-1i8).to_be(), INative::from(-1i8));
    assert_eq!(INative::from_be(INative::from(-1i8)), INative::from(-1i8));
}

#[test]
fn little_endian() {
    let x = INative::from(42i8);
    assert_eq!(INative::from_le(x.to_le()), x);
    assert_eq!(INative::ZERO.to_le(), INative::ZERO);
    assert_eq!(INative::from(-1i8).to_le(), INative::from(-1i8));
    assert_eq!(INative::from_le(INative::from(-1i8)), INative::from(-1i8));
}

#[test]
fn cast_unsigned() {
    assert_eq!(INative::ZERO.cast_unsigned(), UNative::ZERO);
    assert_eq!(INative::from(-1i8).cast_unsigned(), UNative::MAX);
    assert_eq!(INative::from(1i8).cast_unsigned(), UNative::from(1u8));
    assert_eq!(
        INative::MIN.cast_unsigned(),
        UNative::from(1u8) << (UNative::BITS - 1),
    );
    let x = INative::from(-42i8);
    assert_eq!(x.cast_unsigned().cast_signed(), x);
}

#[test]
fn equality() {
    let x = INative::from(-42i8);
    assert_eq!(x, x);
    assert_eq!(&x, &INative::from(-42i8));
    assert_eq!(INative::from(42i8), INative::from(42i16));
    assert_ne!(INative::from(-1i8), INative::from(1i8));
    assert_ne!(INative::MIN, INative::ZERO);
    assert_ne!(INative::MAX, INative::ZERO);
    assert_ne!(INative::MIN, INative::MAX);
}

#[test]
fn ordering() {
    let neg = INative::from(-1i8);
    let zero = INative::ZERO;
    let pos = INative::from(1i8);
    assert!(neg < zero);
    assert!(zero < pos);
    assert!(pos > neg);
    assert!(neg <= zero);
    assert!(zero <= zero);
    assert!(pos >= zero);
    assert_eq!(neg.cmp(&pos), Ordering::Less);
    assert_eq!(pos.cmp(&neg), Ordering::Greater);
    assert_eq!(zero.cmp(&zero), Ordering::Equal);
    assert_eq!(neg.partial_cmp(&pos), Some(Ordering::Less));
    assert!(INative::MIN < INative::MAX);
    assert!(INative::MIN < INative::ZERO);
    assert!(INative::ZERO < INative::MAX);
}

#[test]
fn abs() {
    assert_eq!(INative::from(5i8).abs(), INative::from(5i8));
    assert_eq!(INative::from(-5i8).abs(), INative::from(5i8));
    assert_eq!(INative::ZERO.abs(), INative::ZERO);
}

#[test]
#[should_panic]
fn abs_overflow() {
    let _ = INative::MIN.abs();
}

#[test]
fn unsigned_abs() {
    assert_eq!(INative::from(5i8).unsigned_abs(), UNative::from(5u8));
    assert_eq!(INative::from(-5i8).unsigned_abs(), UNative::from(5u8));
    assert_eq!(INative::ZERO.unsigned_abs(), UNative::ZERO);
    assert_eq!(
        INative::MIN.unsigned_abs(),
        UNative::from(1u8) << (UNative::BITS - 1),
    );
}

#[test]
fn signum() {
    assert_eq!(INative::from(5i8).signum(), INative::from(1i8));
    assert_eq!(INative::from(-5i8).signum(), INative::from(-1i8));
    assert_eq!(INative::ZERO.signum(), INative::ZERO);
    assert_eq!(INative::MAX.signum(), INative::from(1i8));
    assert_eq!(INative::MIN.signum(), INative::from(-1i8));
}

#[test]
fn is_positive() {
    assert!(INative::from(5i8).is_positive());
    assert!(INative::MAX.is_positive());
    assert!(!INative::ZERO.is_positive());
    assert!(!INative::from(-5i8).is_positive());
    assert!(!INative::MIN.is_positive());
}

#[test]
fn is_negative() {
    assert!(INative::from(-5i8).is_negative());
    assert!(INative::MIN.is_negative());
    assert!(!INative::ZERO.is_negative());
    assert!(!INative::from(5i8).is_negative());
    assert!(!INative::MAX.is_negative());
}

#[test]
fn abs_diff() {
    assert_eq!(
        INative::from(7i8).abs_diff(INative::from(3i8)),
        UNative::from(4u8),
    );
    assert_eq!(
        INative::from(3i8).abs_diff(INative::from(7i8)),
        UNative::from(4u8),
    );
    assert_eq!(
        INative::from(-5i8).abs_diff(INative::from(5i8)),
        UNative::from(10u8),
    );
    assert_eq!(INative::ZERO.abs_diff(INative::ZERO), UNative::ZERO);
    assert_eq!(INative::MIN.abs_diff(INative::MAX), UNative::MAX);
}

#[test]
fn addition() {
    let a = INative::from(-2i8);
    let b = INative::from(5i8);
    let sum = INative::from(3i8);
    assert_eq!(a + b, sum);
    assert_eq!(a + &b, sum);
    assert_eq!(&a + b, sum);
    assert_eq!(&a + &b, sum);

    let mut s = a;
    s += b;
    assert_eq!(s, sum);
    s += &a;
    assert_eq!(s, INative::from(1i8));
}

#[test]
fn subtraction() {
    let a = INative::from(5i8);
    let b = INative::from(-3i8);
    let diff = INative::from(8i8);
    assert_eq!(a - b, diff);
    assert_eq!(a - &b, diff);
    assert_eq!(&a - b, diff);
    assert_eq!(&a - &b, diff);

    let mut s = a;
    s -= b;
    assert_eq!(s, diff);
    s -= &a;
    assert_eq!(s, INative::from(3i8));
}

#[test]
fn multiplication() {
    let a = INative::from(-4i8);
    let b = INative::from(3i8);
    let prod = INative::from(-12i8);
    assert_eq!(a * b, prod);
    assert_eq!(a * &b, prod);
    assert_eq!(&a * b, prod);
    assert_eq!(&a * &b, prod);

    let mut s = a;
    s *= b;
    assert_eq!(s, prod);
    s *= &a;
    assert_eq!(s, INative::from(48i8));
}

#[test]
fn division() {
    let a = INative::from(-20i8);
    let b = INative::from(4i8);
    let quot = INative::from(-5i8);
    assert_eq!(a / b, quot);
    assert_eq!(a / &b, quot);
    assert_eq!(&a / b, quot);
    assert_eq!(&a / &b, quot);

    let mut s = a;
    s /= b;
    assert_eq!(s, quot);
    s /= &b;
    assert_eq!(s, INative::from(-1i8));
}

#[test]
fn remainder() {
    let a = INative::from(-23i8);
    let b = INative::from(5i8);
    let rem = INative::from(-3i8);
    assert_eq!(a % b, rem);
    assert_eq!(a % &b, rem);
    assert_eq!(&a % b, rem);
    assert_eq!(&a % &b, rem);

    let mut s = a;
    s %= b;
    assert_eq!(s, rem);
    s %= &INative::from(2i8);
    assert_eq!(s, INative::from(-1i8));
}

#[test]
fn bit_and() {
    let a = INative::from(0b1100i8);
    let b = INative::from(0b1010i8);
    let and = INative::from(0b1000i8);
    assert_eq!(a & b, and);
    assert_eq!(a & &b, and);
    assert_eq!(&a & b, and);
    assert_eq!(&a & &b, and);

    let mut s = a;
    s &= b;
    assert_eq!(s, and);
    s &= &INative::from(0b0111i8);
    assert_eq!(s, INative::from(0b0000i8));
}

#[test]
fn bit_or() {
    let a = INative::from(0b1100i8);
    let b = INative::from(0b1010i8);
    let or = INative::from(0b1110i8);
    assert_eq!(a | b, or);
    assert_eq!(a | &b, or);
    assert_eq!(&a | b, or);
    assert_eq!(&a | &b, or);

    let mut s = a;
    s |= b;
    assert_eq!(s, or);
    s |= &INative::from(0b0001i8);
    assert_eq!(s, INative::from(0b1111i8));
}

#[test]
fn bit_xor() {
    let a = INative::from(0b1100i8);
    let b = INative::from(0b1010i8);
    let xor = INative::from(0b0110i8);
    assert_eq!(a ^ b, xor);
    assert_eq!(a ^ &b, xor);
    assert_eq!(&a ^ b, xor);
    assert_eq!(&a ^ &b, xor);

    let mut s = a;
    s ^= b;
    assert_eq!(s, xor);
    s ^= &a;
    assert_eq!(s, INative::from(0b1010i8));
}

#[test]
fn not() {
    let a = INative::from(5i8);
    assert_eq!(!a, !&a);
    assert_eq!(!!a, a);
    assert_eq!(!INative::ZERO, INative::from(-1i8));
    assert_eq!(!INative::from(-1i8), INative::ZERO);
}

#[test]
fn negation() {
    let a = INative::from(5i8);
    let neg = INative::from(-5i8);
    assert_eq!(-a, neg);
    assert_eq!(-&a, neg);
    assert_eq!(-(-a), a);
    assert_eq!(-INative::ZERO, INative::ZERO);
}

#[test]
fn shl() {
    let a = INative::from(1i8);
    let shifted = INative::from(8i8);

    assert_eq!(a << 3u8, shifted);
    assert_eq!(a << &3u8, shifted);
    assert_eq!(&a << 3u8, shifted);
    assert_eq!(&a << &3u8, shifted);
    assert_eq!(a << UNative::from(3u8), shifted);
    assert_eq!(a << INative::from(3i8), shifted);
    assert_eq!(1i32 << INative::from(3i8), 8i32);

    let mut s = a;
    s <<= 3u8;
    assert_eq!(s, shifted);
    s <<= &INative::from(1i8);
    assert_eq!(s, INative::from(16i8));
}

#[test]
fn shr() {
    let a = INative::from(-16i8);
    let shifted = INative::from(-2i8);

    assert_eq!(a >> 3u8, shifted);
    assert_eq!(a >> &3u8, shifted);
    assert_eq!(&a >> 3u8, shifted);
    assert_eq!(&a >> &3u8, shifted);
    assert_eq!(a >> UNative::from(3u8), shifted);
    assert_eq!(a >> INative::from(3i8), shifted);
    assert_eq!(-16i32 >> INative::from(3i8), -2i32);

    let mut s = a;
    s >>= 1u8;
    assert_eq!(s, INative::from(-8i8));
    s >>= &INative::from(2i8);
    assert_eq!(s, INative::from(-2i8));
}

#[test]
fn sum() {
    let xs = [
        INative::from(-1i8),
        INative::from(2i8),
        INative::from(-3i8),
        INative::from(4i8),
    ];
    assert_eq!(xs.into_iter().sum::<INative>(), INative::from(2i8));
    assert_eq!(xs.iter().sum::<INative>(), INative::from(2i8));
    assert_eq!(
        core::iter::empty::<INative>().sum::<INative>(),
        INative::ZERO,
    );
}

#[test]
fn product() {
    let xs = [INative::from(-2i8), INative::from(3i8), INative::from(-4i8)];
    assert_eq!(xs.into_iter().product::<INative>(), INative::from(24i8));
    assert_eq!(xs.iter().product::<INative>(), INative::from(24i8));
    assert_eq!(
        core::iter::empty::<INative>().product::<INative>(),
        INative::from(1i8),
    );
}

#[test]
fn formatting() {
    assert_eq!(format!("{}", INative::from(42i8)), "42");
    assert_eq!(format!("{}", INative::from(-42i8)), "-42");
    assert_eq!(format!("{:?}", INative::from(42i8)), "42");
    assert_eq!(format!("{:?}", INative::from(-42i8)), "-42");
    assert_eq!(format!("{:b}", INative::from(5i8)), "101");
    assert_eq!(format!("{:o}", INative::from(8i8)), "10");
    assert_eq!(format!("{:x}", INative::from(0x7Fi8)), "7f");
    assert_eq!(format!("{:X}", INative::from(0x7Fi8)), "7F");

    assert_eq!(format!("{:5}", INative::from(-42i8)), "  -42");
    assert_eq!(format!("{:<5}", INative::from(-42i8)), "-42  ");
    assert_eq!(format!("{:06}", INative::from(-42i8)), "-00042");
    assert_eq!(format!("{:*^5}", INative::from(42i8)), "*42**");
    assert_eq!(format!("{:+}", INative::from(42i8)), "+42");
    assert_eq!(format!("{:#b}", INative::from(5i8)), "0b101");
    assert_eq!(format!("{:#x}", INative::from(0x7Fi8)), "0x7f");
}

#[test]
fn from_str() {
    assert_eq!("42".parse::<INative>(), Ok(INative::from(42i8)));
    assert_eq!("-42".parse::<INative>(), Ok(INative::from(-42i8)));
    assert_eq!("0".parse::<INative>(), Ok(INative::ZERO));
    assert_eq!(
        format!("{}", INative::MAX).parse::<INative>(),
        Ok(INative::MAX),
    );
    assert_eq!(
        format!("{}", INative::MIN).parse::<INative>(),
        Ok(INative::MIN),
    );
    assert!("abc".parse::<INative>().is_err());
    assert!("".parse::<INative>().is_err());
}

#[test]
fn checked_add() {
    assert_eq!(
        INative::from(2i8).checked_add(INative::from(3i8)),
        Some(INative::from(5i8)),
    );
    assert_eq!(INative::MAX.checked_add(INative::from(1i8)), None);
    assert_eq!(INative::MIN.checked_add(INative::from(-1i8)), None);
    assert_eq!(
        INative::from(-1i8).checked_add(INative::from(1i8)),
        Some(INative::ZERO),
    );
}

#[test]
fn checked_add_unsigned() {
    assert_eq!(
        INative::from(5i8).checked_add_unsigned(UNative::from(3u8)),
        Some(INative::from(8i8)),
    );
    assert_eq!(
        INative::from(-5i8).checked_add_unsigned(UNative::from(3u8)),
        Some(INative::from(-2i8)),
    );
    assert_eq!(INative::MAX.checked_add_unsigned(UNative::from(1u8)), None);
}

#[test]
fn checked_sub() {
    assert_eq!(
        INative::from(5i8).checked_sub(INative::from(3i8)),
        Some(INative::from(2i8)),
    );
    assert_eq!(
        INative::from(3i8).checked_sub(INative::from(5i8)),
        Some(INative::from(-2i8)),
    );
    assert_eq!(INative::MIN.checked_sub(INative::from(1i8)), None);
    assert_eq!(INative::MAX.checked_sub(INative::from(-1i8)), None);
}

#[test]
fn checked_sub_unsigned() {
    assert_eq!(
        INative::from(5i8).checked_sub_unsigned(UNative::from(3u8)),
        Some(INative::from(2i8)),
    );
    assert_eq!(
        INative::from(-5i8).checked_sub_unsigned(UNative::from(3u8)),
        Some(INative::from(-8i8)),
    );
    assert_eq!(INative::MIN.checked_sub_unsigned(UNative::from(1u8)), None);
}

#[test]
fn checked_neg() {
    assert_eq!(INative::ZERO.checked_neg(), Some(INative::ZERO));
    assert_eq!(INative::from(5i8).checked_neg(), Some(INative::from(-5i8)),);
    assert_eq!(INative::from(-5i8).checked_neg(), Some(INative::from(5i8)),);
    assert_eq!(INative::MIN.checked_neg(), None);
}

#[test]
fn checked_mul() {
    assert_eq!(
        INative::from(4i8).checked_mul(INative::from(3i8)),
        Some(INative::from(12i8)),
    );
    assert_eq!(
        INative::from(-4i8).checked_mul(INative::from(3i8)),
        Some(INative::from(-12i8)),
    );
    assert_eq!(INative::MAX.checked_mul(INative::from(2i8)), None);
    assert_eq!(INative::MIN.checked_mul(INative::from(2i8)), None);
}

#[test]
fn checked_div() {
    assert_eq!(
        INative::from(23i8).checked_div(INative::from(10i8)),
        Some(INative::from(2i8)),
    );
    assert_eq!(
        INative::from(-23i8).checked_div(INative::from(10i8)),
        Some(INative::from(-2i8)),
    );
    assert_eq!(INative::from(5i8).checked_div(INative::ZERO), None);
    assert_eq!(INative::MIN.checked_div(INative::from(-1i8)), None);
}

#[test]
fn checked_div_euclid() {
    assert_eq!(
        INative::from(23i8).checked_div_euclid(INative::from(10i8)),
        Some(INative::from(2i8)),
    );
    assert_eq!(
        INative::from(-23i8).checked_div_euclid(INative::from(10i8)),
        Some(INative::from(-3i8)),
    );
    assert_eq!(INative::from(5i8).checked_div_euclid(INative::ZERO), None);
    assert_eq!(INative::MIN.checked_div_euclid(INative::from(-1i8)), None);
}

#[test]
fn checked_rem() {
    assert_eq!(
        INative::from(23i8).checked_rem(INative::from(10i8)),
        Some(INative::from(3i8)),
    );
    assert_eq!(
        INative::from(-23i8).checked_rem(INative::from(10i8)),
        Some(INative::from(-3i8)),
    );
    assert_eq!(INative::from(5i8).checked_rem(INative::ZERO), None);
    assert_eq!(INative::MIN.checked_rem(INative::from(-1i8)), None);
}

#[test]
fn checked_rem_euclid() {
    assert_eq!(
        INative::from(23i8).checked_rem_euclid(INative::from(10i8)),
        Some(INative::from(3i8)),
    );
    assert_eq!(
        INative::from(-23i8).checked_rem_euclid(INative::from(10i8)),
        Some(INative::from(7i8)),
    );
    assert_eq!(INative::from(5i8).checked_rem_euclid(INative::ZERO), None);
    assert_eq!(INative::MIN.checked_rem_euclid(INative::from(-1i8)), None);
}

#[test]
fn checked_shl() {
    assert_eq!(INative::from(1i8).checked_shl(3), Some(INative::from(8i8)),);
    assert_eq!(INative::from(1i8).checked_shl(INative::BITS), None);
}

#[test]
fn checked_shr() {
    assert_eq!(
        INative::from(-16i8).checked_shr(3),
        Some(INative::from(-2i8)),
    );
    assert_eq!(INative::from(8i8).checked_shr(INative::BITS), None);
}

#[test]
fn checked_ilog() {
    assert_eq!(
        INative::from(100i8).checked_ilog(INative::from(10i8)),
        Some(2),
    );
    assert_eq!(INative::ZERO.checked_ilog(INative::from(10i8)), None);
    assert_eq!(INative::from(-1i8).checked_ilog(INative::from(10i8)), None,);
    assert_eq!(INative::from(10i8).checked_ilog(INative::from(1i8)), None);
}

#[test]
fn checked_ilog2() {
    assert_eq!(INative::from(4i8).checked_ilog2(), Some(2));
    assert_eq!(INative::ZERO.checked_ilog2(), None);
    assert_eq!(INative::from(-1i8).checked_ilog2(), None);
}

#[test]
fn checked_ilog10() {
    assert_eq!(INative::from(100i8).checked_ilog10(), Some(2));
    assert_eq!(INative::ZERO.checked_ilog10(), None);
    assert_eq!(INative::from(-1i8).checked_ilog10(), None);
}

#[test]
fn checked_pow() {
    assert_eq!(
        INative::from(2i8).checked_pow(10),
        Some(INative::from(1024i16)),
    );
    assert_eq!(
        INative::from(-2i8).checked_pow(3),
        Some(INative::from(-8i8)),
    );
    assert_eq!(INative::MAX.checked_pow(2), None);
    assert_eq!(INative::MIN.checked_pow(2), None);
}

#[test]
fn checked_isqrt() {
    assert_eq!(INative::ZERO.checked_isqrt(), Some(INative::ZERO));
    assert_eq!(
        INative::from(100i8).checked_isqrt(),
        Some(INative::from(10i8)),
    );
    assert_eq!(INative::from(-1i8).checked_isqrt(), None);
    assert_eq!(INative::MIN.checked_isqrt(), None);
}

#[test]
fn checked_abs() {
    assert_eq!(INative::from(5i8).checked_abs(), Some(INative::from(5i8)));
    assert_eq!(INative::from(-5i8).checked_abs(), Some(INative::from(5i8)),);
    assert_eq!(INative::ZERO.checked_abs(), Some(INative::ZERO));
    assert_eq!(INative::MIN.checked_abs(), None);
}

#[test]
fn overflowing_add() {
    assert_eq!(
        INative::from(2i8).overflowing_add(INative::from(3i8)),
        (INative::from(5i8), false),
    );
    assert_eq!(
        INative::from(-1i8).overflowing_add(INative::from(1i8)),
        (INative::ZERO, false),
    );
    assert_eq!(
        INative::MAX.overflowing_add(INative::from(1i8)),
        (INative::MIN, true),
    );
    assert_eq!(
        INative::MIN.overflowing_add(INative::from(-1i8)),
        (INative::MAX, true),
    );
}

#[test]
fn overflowing_add_unsigned() {
    assert_eq!(
        INative::from(5i8).overflowing_add_unsigned(UNative::from(3u8)),
        (INative::from(8i8), false),
    );
    assert_eq!(
        INative::from(-5i8).overflowing_add_unsigned(UNative::from(3u8)),
        (INative::from(-2i8), false),
    );
    assert_eq!(
        INative::MAX.overflowing_add_unsigned(UNative::from(1u8)),
        (INative::MIN, true),
    );
}

#[test]
fn overflowing_sub() {
    assert_eq!(
        INative::from(5i8).overflowing_sub(INative::from(3i8)),
        (INative::from(2i8), false),
    );
    assert_eq!(
        INative::from(3i8).overflowing_sub(INative::from(5i8)),
        (INative::from(-2i8), false),
    );
    assert_eq!(
        INative::MIN.overflowing_sub(INative::from(1i8)),
        (INative::MAX, true),
    );
    assert_eq!(
        INative::MAX.overflowing_sub(INative::from(-1i8)),
        (INative::MIN, true),
    );
}

#[test]
fn overflowing_sub_unsigned() {
    assert_eq!(
        INative::from(5i8).overflowing_sub_unsigned(UNative::from(3u8)),
        (INative::from(2i8), false),
    );
    assert_eq!(
        INative::from(-5i8).overflowing_sub_unsigned(UNative::from(3u8)),
        (INative::from(-8i8), false),
    );
    assert_eq!(
        INative::MIN.overflowing_sub_unsigned(UNative::from(1u8)),
        (INative::MAX, true),
    );
}

#[test]
fn overflowing_neg() {
    assert_eq!(INative::ZERO.overflowing_neg(), (INative::ZERO, false));
    assert_eq!(
        INative::from(5i8).overflowing_neg(),
        (INative::from(-5i8), false),
    );
    assert_eq!(
        INative::from(-5i8).overflowing_neg(),
        (INative::from(5i8), false),
    );
    assert_eq!(INative::MIN.overflowing_neg(), (INative::MIN, true));
}

#[test]
fn overflowing_abs() {
    assert_eq!(
        INative::from(5i8).overflowing_abs(),
        (INative::from(5i8), false),
    );
    assert_eq!(
        INative::from(-5i8).overflowing_abs(),
        (INative::from(5i8), false),
    );
    assert_eq!(INative::ZERO.overflowing_abs(), (INative::ZERO, false));
    assert_eq!(INative::MIN.overflowing_abs(), (INative::MIN, true));
}

#[test]
fn overflowing_mul() {
    assert_eq!(
        INative::from(4i8).overflowing_mul(INative::from(3i8)),
        (INative::from(12i8), false),
    );
    assert_eq!(
        INative::from(-4i8).overflowing_mul(INative::from(3i8)),
        (INative::from(-12i8), false),
    );
    assert!(INative::MAX.overflowing_mul(INative::from(2i8)).1);
    assert_eq!(
        INative::MIN.overflowing_mul(INative::from(-1i8)),
        (INative::MIN, true),
    );
}

#[test]
fn overflowing_div() {
    assert_eq!(
        INative::from(7i8).overflowing_div(INative::from(2i8)),
        (INative::from(3i8), false),
    );
    assert_eq!(
        INative::from(-7i8).overflowing_div(INative::from(2i8)),
        (INative::from(-3i8), false),
    );
    assert_eq!(
        INative::MIN.overflowing_div(INative::from(-1i8)),
        (INative::MIN, true),
    );
}

#[test]
#[should_panic]
fn overflowing_div_by_zero() {
    let _ = INative::from(5i8).overflowing_div(INative::ZERO);
}

#[test]
fn overflowing_div_euclid() {
    assert_eq!(
        INative::from(-7i8).overflowing_div_euclid(INative::from(2i8)),
        (INative::from(-4i8), false),
    );
    assert_eq!(
        INative::MIN.overflowing_div_euclid(INative::from(-1i8)),
        (INative::MIN, true),
    );
}

#[test]
#[should_panic]
fn overflowing_div_euclid_by_zero() {
    let _ = INative::from(5i8).overflowing_div_euclid(INative::ZERO);
}

#[test]
fn overflowing_rem() {
    assert_eq!(
        INative::from(-7i8).overflowing_rem(INative::from(2i8)),
        (INative::from(-1i8), false),
    );
    assert_eq!(
        INative::MIN.overflowing_rem(INative::from(-1i8)),
        (INative::ZERO, true),
    );
}

#[test]
#[should_panic]
fn overflowing_rem_by_zero() {
    let _ = INative::from(5i8).overflowing_rem(INative::ZERO);
}

#[test]
fn overflowing_rem_euclid() {
    assert_eq!(
        INative::from(-7i8).overflowing_rem_euclid(INative::from(2i8)),
        (INative::from(1i8), false),
    );
    assert_eq!(
        INative::MIN.overflowing_rem_euclid(INative::from(-1i8)),
        (INative::ZERO, true),
    );
}

#[test]
#[should_panic]
fn overflowing_rem_euclid_by_zero() {
    let _ = INative::from(5i8).overflowing_rem_euclid(INative::ZERO);
}

#[test]
fn overflowing_shl() {
    assert_eq!(
        INative::from(1i8).overflowing_shl(3),
        (INative::from(8i8), false),
    );
    assert_eq!(
        INative::from(1i8).overflowing_shl(INative::BITS),
        (INative::from(1i8), true),
    );
}

#[test]
fn overflowing_shr() {
    assert_eq!(
        INative::from(-16i8).overflowing_shr(3),
        (INative::from(-2i8), false),
    );
    assert_eq!(
        INative::from(-16i8).overflowing_shr(INative::BITS),
        (INative::from(-16i8), true),
    );
}

#[test]
fn overflowing_pow() {
    assert_eq!(
        INative::from(2i8).overflowing_pow(10),
        (INative::from(1024i16), false),
    );
    assert_eq!(
        INative::from(-2i8).overflowing_pow(3),
        (INative::from(-8i8), false),
    );
    // MAX^2 = (2^(BITS-1) - 1)^2 = 2^(2*BITS-2) - 2^BITS + 1, which wraps to 1.
    assert_eq!(INative::MAX.overflowing_pow(2), (INative::from(1i8), true),);
    // MIN^2 = 2^(2*BITS-2), which wraps to 0.
    assert_eq!(INative::MIN.overflowing_pow(2), (INative::ZERO, true),);
}

#[test]
fn saturating_add() {
    assert_eq!(
        INative::from(2i8).saturating_add(INative::from(3i8)),
        INative::from(5i8),
    );
    assert_eq!(
        INative::from(-1i8).saturating_add(INative::from(1i8)),
        INative::ZERO,
    );
    assert_eq!(
        INative::MAX.saturating_add(INative::from(1i8)),
        INative::MAX
    );
    assert_eq!(
        INative::MIN.saturating_add(INative::from(-1i8)),
        INative::MIN
    );
}

#[test]
fn saturating_add_unsigned() {
    assert_eq!(
        INative::from(5i8).saturating_add_unsigned(UNative::from(3u8)),
        INative::from(8i8),
    );
    assert_eq!(
        INative::from(-5i8).saturating_add_unsigned(UNative::from(3u8)),
        INative::from(-2i8),
    );
    assert_eq!(
        INative::MAX.saturating_add_unsigned(UNative::from(1u8)),
        INative::MAX,
    );
    assert_eq!(
        INative::MIN.saturating_add_unsigned(UNative::MAX),
        INative::MAX
    );
}

#[test]
fn saturating_sub() {
    assert_eq!(
        INative::from(5i8).saturating_sub(INative::from(3i8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(3i8).saturating_sub(INative::from(5i8)),
        INative::from(-2i8),
    );
    assert_eq!(
        INative::MIN.saturating_sub(INative::from(1i8)),
        INative::MIN
    );
    assert_eq!(
        INative::MAX.saturating_sub(INative::from(-1i8)),
        INative::MAX
    );
}

#[test]
fn saturating_sub_unsigned() {
    assert_eq!(
        INative::from(5i8).saturating_sub_unsigned(UNative::from(3u8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(-5i8).saturating_sub_unsigned(UNative::from(3u8)),
        INative::from(-8i8),
    );
    assert_eq!(
        INative::MIN.saturating_sub_unsigned(UNative::from(1u8)),
        INative::MIN,
    );
    assert_eq!(
        INative::MAX.saturating_sub_unsigned(UNative::MAX),
        INative::MIN
    );
}

#[test]
fn saturating_neg() {
    assert_eq!(INative::ZERO.saturating_neg(), INative::ZERO);
    assert_eq!(INative::from(5i8).saturating_neg(), INative::from(-5i8));
    assert_eq!(INative::from(-5i8).saturating_neg(), INative::from(5i8));
    assert_eq!(INative::MIN.saturating_neg(), INative::MAX);
}

#[test]
fn saturating_abs() {
    assert_eq!(INative::from(5i8).saturating_abs(), INative::from(5i8));
    assert_eq!(INative::from(-5i8).saturating_abs(), INative::from(5i8));
    assert_eq!(INative::ZERO.saturating_abs(), INative::ZERO);
    assert_eq!(INative::MIN.saturating_abs(), INative::MAX);
}

#[test]
fn saturating_mul() {
    assert_eq!(
        INative::from(4i8).saturating_mul(INative::from(3i8)),
        INative::from(12i8),
    );
    assert_eq!(
        INative::from(-4i8).saturating_mul(INative::from(3i8)),
        INative::from(-12i8),
    );
    assert_eq!(
        INative::MAX.saturating_mul(INative::from(2i8)),
        INative::MAX
    );
    assert_eq!(
        INative::MIN.saturating_mul(INative::from(2i8)),
        INative::MIN
    );
    assert_eq!(
        INative::MIN.saturating_mul(INative::from(-1i8)),
        INative::MAX
    );
}

#[test]
fn saturating_div() {
    assert_eq!(
        INative::from(7i8).saturating_div(INative::from(2i8)),
        INative::from(3i8),
    );
    assert_eq!(
        INative::from(-7i8).saturating_div(INative::from(2i8)),
        INative::from(-3i8),
    );
    // MIN / -1 saturates to MAX.
    assert_eq!(
        INative::MIN.saturating_div(INative::from(-1i8)),
        INative::MAX
    );
}

#[test]
#[should_panic]
fn saturating_div_by_zero() {
    let _ = INative::from(5i8).saturating_div(INative::ZERO);
}

#[test]
fn saturating_pow() {
    assert_eq!(
        INative::from(2i8).saturating_pow(10),
        INative::from(1024i16),
    );
    assert_eq!(INative::from(-2i8).saturating_pow(3), INative::from(-8i8),);
    assert_eq!(INative::MAX.saturating_pow(2), INative::MAX);
    assert_eq!(INative::MIN.saturating_pow(3), INative::MIN);
}

#[test]
fn wrapping_add() {
    assert_eq!(
        INative::from(2i8).wrapping_add(INative::from(3i8)),
        INative::from(5i8),
    );
    assert_eq!(
        INative::from(-1i8).wrapping_add(INative::from(1i8)),
        INative::ZERO,
    );
    assert_eq!(INative::MAX.wrapping_add(INative::from(1i8)), INative::MIN);
    assert_eq!(INative::MIN.wrapping_add(INative::from(-1i8)), INative::MAX);
}

#[test]
fn wrapping_add_unsigned() {
    assert_eq!(
        INative::from(5i8).wrapping_add_unsigned(UNative::from(3u8)),
        INative::from(8i8),
    );
    assert_eq!(
        INative::from(-5i8).wrapping_add_unsigned(UNative::from(3u8)),
        INative::from(-2i8),
    );
    assert_eq!(
        INative::MAX.wrapping_add_unsigned(UNative::from(1u8)),
        INative::MIN,
    );
}

#[test]
fn wrapping_sub() {
    assert_eq!(
        INative::from(5i8).wrapping_sub(INative::from(3i8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(3i8).wrapping_sub(INative::from(5i8)),
        INative::from(-2i8),
    );
    assert_eq!(INative::MIN.wrapping_sub(INative::from(1i8)), INative::MAX);
    assert_eq!(INative::MAX.wrapping_sub(INative::from(-1i8)), INative::MIN);
}

#[test]
fn wrapping_sub_unsigned() {
    assert_eq!(
        INative::from(5i8).wrapping_sub_unsigned(UNative::from(3u8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(-5i8).wrapping_sub_unsigned(UNative::from(3u8)),
        INative::from(-8i8),
    );
    assert_eq!(
        INative::MIN.wrapping_sub_unsigned(UNative::from(1u8)),
        INative::MAX,
    );
}

#[test]
fn wrapping_neg() {
    assert_eq!(INative::ZERO.wrapping_neg(), INative::ZERO);
    assert_eq!(INative::from(5i8).wrapping_neg(), INative::from(-5i8));
    assert_eq!(INative::from(-5i8).wrapping_neg(), INative::from(5i8));
    assert_eq!(INative::MIN.wrapping_neg(), INative::MIN);
}

#[test]
fn wrapping_abs() {
    assert_eq!(INative::from(5i8).wrapping_abs(), INative::from(5i8));
    assert_eq!(INative::from(-5i8).wrapping_abs(), INative::from(5i8));
    assert_eq!(INative::ZERO.wrapping_abs(), INative::ZERO);
    assert_eq!(INative::MIN.wrapping_abs(), INative::MIN);
}

#[test]
fn wrapping_mul() {
    assert_eq!(
        INative::from(4i8).wrapping_mul(INative::from(3i8)),
        INative::from(12i8),
    );
    assert_eq!(
        INative::from(-4i8).wrapping_mul(INative::from(3i8)),
        INative::from(-12i8),
    );
    assert_eq!(INative::MIN.wrapping_mul(INative::from(-1i8)), INative::MIN);
}

#[test]
fn wrapping_div() {
    assert_eq!(
        INative::from(7i8).wrapping_div(INative::from(2i8)),
        INative::from(3i8),
    );
    assert_eq!(
        INative::from(-7i8).wrapping_div(INative::from(2i8)),
        INative::from(-3i8),
    );
    // MIN / -1 wraps to MIN.
    assert_eq!(INative::MIN.wrapping_div(INative::from(-1i8)), INative::MIN);
}

#[test]
#[should_panic]
fn wrapping_div_by_zero() {
    let _ = INative::from(5i8).wrapping_div(INative::ZERO);
}

#[test]
fn wrapping_div_euclid() {
    assert_eq!(
        INative::from(-7i8).wrapping_div_euclid(INative::from(2i8)),
        INative::from(-4i8),
    );
    assert_eq!(
        INative::MIN.wrapping_div_euclid(INative::from(-1i8)),
        INative::MIN,
    );
}

#[test]
#[should_panic]
fn wrapping_div_euclid_by_zero() {
    let _ = INative::from(5i8).wrapping_div_euclid(INative::ZERO);
}

#[test]
fn wrapping_rem() {
    assert_eq!(
        INative::from(-7i8).wrapping_rem(INative::from(2i8)),
        INative::from(-1i8),
    );
    // MIN % -1 wraps to 0.
    assert_eq!(
        INative::MIN.wrapping_rem(INative::from(-1i8)),
        INative::ZERO
    );
}

#[test]
#[should_panic]
fn wrapping_rem_by_zero() {
    let _ = INative::from(5i8).wrapping_rem(INative::ZERO);
}

#[test]
fn wrapping_rem_euclid() {
    assert_eq!(
        INative::from(-7i8).wrapping_rem_euclid(INative::from(2i8)),
        INative::from(1i8),
    );
    assert_eq!(
        INative::MIN.wrapping_rem_euclid(INative::from(-1i8)),
        INative::ZERO,
    );
}

#[test]
#[should_panic]
fn wrapping_rem_euclid_by_zero() {
    let _ = INative::from(5i8).wrapping_rem_euclid(INative::ZERO);
}

#[test]
fn wrapping_shl() {
    assert_eq!(INative::from(1i8).wrapping_shl(3), INative::from(8i8));
    assert_eq!(
        INative::from(1i8).wrapping_shl(INative::BITS),
        INative::from(1i8),
    );
}

#[test]
fn wrapping_shr() {
    assert_eq!(INative::from(-16i8).wrapping_shr(3), INative::from(-2i8));
    assert_eq!(
        INative::from(-16i8).wrapping_shr(INative::BITS),
        INative::from(-16i8),
    );
}

#[test]
fn wrapping_pow() {
    assert_eq!(INative::from(2i8).wrapping_pow(10), INative::from(1024i16),);
    assert_eq!(INative::from(-2i8).wrapping_pow(3), INative::from(-8i8));
    // MAX^2 wraps to 1; MIN^2 wraps to 0.
    assert_eq!(INative::MAX.wrapping_pow(2), INative::from(1i8));
    assert_eq!(INative::MIN.wrapping_pow(2), INative::ZERO);
}

#[test]
fn strict_add() {
    assert_eq!(
        INative::from(2i8).strict_add(INative::from(3i8)),
        INative::from(5i8),
    );
    assert_eq!(
        INative::from(-1i8).strict_add(INative::from(1i8)),
        INative::ZERO,
    );
}

#[test]
#[should_panic]
fn strict_add_overflow() {
    let _ = INative::MAX.strict_add(INative::from(1i8));
}

#[test]
fn strict_add_unsigned() {
    assert_eq!(
        INative::from(5i8).strict_add_unsigned(UNative::from(3u8)),
        INative::from(8i8),
    );
    assert_eq!(
        INative::from(-5i8).strict_add_unsigned(UNative::from(3u8)),
        INative::from(-2i8),
    );
}

#[test]
#[should_panic]
fn strict_add_unsigned_overflow() {
    let _ = INative::MAX.strict_add_unsigned(UNative::from(1u8));
}

#[test]
fn strict_sub() {
    assert_eq!(
        INative::from(5i8).strict_sub(INative::from(3i8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(3i8).strict_sub(INative::from(5i8)),
        INative::from(-2i8),
    );
}

#[test]
#[should_panic]
fn strict_sub_overflow() {
    let _ = INative::MIN.strict_sub(INative::from(1i8));
}

#[test]
fn strict_sub_unsigned() {
    assert_eq!(
        INative::from(5i8).strict_sub_unsigned(UNative::from(3u8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(-5i8).strict_sub_unsigned(UNative::from(3u8)),
        INative::from(-8i8),
    );
}

#[test]
#[should_panic]
fn strict_sub_unsigned_overflow() {
    let _ = INative::MIN.strict_sub_unsigned(UNative::from(1u8));
}

#[test]
fn strict_abs() {
    assert_eq!(INative::from(5i8).strict_abs(), INative::from(5i8));
    assert_eq!(INative::from(-5i8).strict_abs(), INative::from(5i8));
    assert_eq!(INative::ZERO.strict_abs(), INative::ZERO);
}

#[test]
#[should_panic]
fn strict_abs_overflow() {
    let _ = INative::MIN.strict_abs();
}

#[test]
fn strict_neg() {
    assert_eq!(INative::ZERO.strict_neg(), INative::ZERO);
    assert_eq!(INative::from(5i8).strict_neg(), INative::from(-5i8));
    assert_eq!(INative::from(-5i8).strict_neg(), INative::from(5i8));
}

#[test]
#[should_panic]
fn strict_neg_overflow() {
    let _ = INative::MIN.strict_neg();
}

#[test]
fn strict_mul() {
    assert_eq!(
        INative::from(4i8).strict_mul(INative::from(3i8)),
        INative::from(12i8),
    );
    assert_eq!(
        INative::from(-4i8).strict_mul(INative::from(3i8)),
        INative::from(-12i8),
    );
}

#[test]
#[should_panic]
fn strict_mul_overflow() {
    let _ = INative::MAX.strict_mul(INative::from(2i8));
}

#[test]
fn strict_div() {
    assert_eq!(
        INative::from(23i8).strict_div(INative::from(10i8)),
        INative::from(2i8),
    );
    assert_eq!(
        INative::from(-23i8).strict_div(INative::from(10i8)),
        INative::from(-2i8),
    );
}

#[test]
#[should_panic]
fn strict_div_by_zero() {
    let _ = INative::from(5i8).strict_div(INative::ZERO);
}

#[test]
#[should_panic]
fn strict_div_overflow() {
    let _ = INative::MIN.strict_div(INative::from(-1i8));
}

#[test]
fn strict_div_euclid() {
    assert_eq!(
        INative::from(-23i8).strict_div_euclid(INative::from(10i8)),
        INative::from(-3i8),
    );
}

#[test]
#[should_panic]
fn strict_div_euclid_by_zero() {
    let _ = INative::from(5i8).strict_div_euclid(INative::ZERO);
}

#[test]
fn strict_rem() {
    assert_eq!(
        INative::from(23i8).strict_rem(INative::from(10i8)),
        INative::from(3i8),
    );
    assert_eq!(
        INative::from(-23i8).strict_rem(INative::from(10i8)),
        INative::from(-3i8),
    );
}

#[test]
#[should_panic]
fn strict_rem_by_zero() {
    let _ = INative::from(5i8).strict_rem(INative::ZERO);
}

#[test]
#[should_panic]
fn strict_rem_overflow() {
    let _ = INative::MIN.strict_rem(INative::from(-1i8));
}

#[test]
fn strict_rem_euclid() {
    assert_eq!(
        INative::from(-23i8).strict_rem_euclid(INative::from(10i8)),
        INative::from(7i8),
    );
}

#[test]
#[should_panic]
fn strict_rem_euclid_by_zero() {
    let _ = INative::from(5i8).strict_rem_euclid(INative::ZERO);
}

#[test]
#[should_panic]
fn strict_rem_euclid_overflow() {
    let _ = INative::MIN.strict_rem_euclid(INative::from(-1i8));
}

#[test]
fn strict_shl() {
    assert_eq!(INative::from(1i8).strict_shl(3), INative::from(8i8),);
}

#[test]
#[should_panic]
fn strict_shl_overflow() {
    let _ = INative::from(1i8).strict_shl(INative::BITS);
}

#[test]
fn strict_shr() {
    assert_eq!(INative::from(-16i8).strict_shr(3), INative::from(-2i8),);
}

#[test]
#[should_panic]
fn strict_shr_overflow() {
    let _ = INative::from(8i8).strict_shr(INative::BITS);
}

#[test]
fn strict_pow() {
    assert_eq!(INative::from(2i8).strict_pow(10), INative::from(1024i16));
    assert_eq!(INative::from(-2i8).strict_pow(3), INative::from(-8i8));
}

#[test]
#[should_panic]
fn strict_pow_overflow() {
    let _ = INative::MAX.strict_pow(2);
}

#[test]
fn unchecked_add() {
    // SAFETY: 2 + 3 doesn't overflow.
    let result = unsafe { INative::from(2i8).unchecked_add(INative::from(3i8)) };
    assert_eq!(result, INative::from(5i8));
}

#[test]
fn unchecked_sub() {
    // SAFETY: 5 - 3 doesn't overflow.
    let result = unsafe { INative::from(5i8).unchecked_sub(INative::from(3i8)) };
    assert_eq!(result, INative::from(2i8));
}

#[test]
fn unchecked_neg() {
    // SAFETY: -5 doesn't overflow.
    let result = unsafe { INative::from(5i8).unchecked_neg() };
    assert_eq!(result, INative::from(-5i8));
}

#[test]
fn unchecked_mul() {
    // SAFETY: 4 * 3 doesn't overflow.
    let result = unsafe { INative::from(4i8).unchecked_mul(INative::from(3i8)) };
    assert_eq!(result, INative::from(12i8));
}

#[test]
fn unchecked_shl() {
    // SAFETY: 3 < BITS.
    let result = unsafe { INative::from(1i8).unchecked_shl(3) };
    assert_eq!(result, INative::from(8i8));
}

#[test]
fn unchecked_shr() {
    // SAFETY: 3 < BITS.
    let result = unsafe { INative::from(-16i8).unchecked_shr(3) };
    assert_eq!(result, INative::from(-2i8));
}

#[test]
fn unbounded_shl() {
    assert_eq!(INative::from(1i8).unbounded_shl(3), INative::from(8i8));
    assert_eq!(
        INative::from(1i8).unbounded_shl(INative::BITS),
        INative::ZERO
    );
    assert_eq!(
        INative::from(1i8).unbounded_shl(INative::BITS + 100),
        INative::ZERO,
    );
}

#[test]
fn unbounded_shr() {
    assert_eq!(INative::from(8i8).unbounded_shr(3), INative::from(1i8));
    assert_eq!(
        INative::from(8i8).unbounded_shr(INative::BITS),
        INative::ZERO
    );
    assert_eq!(
        INative::from(-1i8).unbounded_shr(INative::BITS),
        INative::from(-1i8),
    );
    assert_eq!(
        INative::from(-1i8).unbounded_shr(INative::BITS + 100),
        INative::from(-1i8),
    );
}

#[test]
fn from_str_radix() {
    assert_eq!(INative::from_str_radix("42", 10), Ok(INative::from(42i8)));
    assert_eq!(INative::from_str_radix("+42", 10), Ok(INative::from(42i8)));
    assert_eq!(INative::from_str_radix("-42", 10), Ok(INative::from(-42i8)));
    assert_eq!(INative::from_str_radix("0", 10), Ok(INative::ZERO));
    assert_eq!(INative::from_str_radix("7f", 16), Ok(INative::from(127i8)));
    assert_eq!(
        INative::from_str_radix("-80", 16),
        Ok(INative::from(-128i8))
    );
    assert_eq!(INative::from_str_radix("101", 2), Ok(INative::from(5i8)));
    assert_eq!(INative::from_str_radix("z", 36), Ok(INative::from(35i8)));
    assert_eq!(
        INative::from_str_radix(&format!("{}", INative::MAX), 10),
        Ok(INative::MAX),
    );
    assert_eq!(
        INative::from_str_radix(&format!("{}", INative::MIN), 10),
        Ok(INative::MIN),
    );
    assert!(INative::from_str_radix("abc", 10).is_err());
    assert!(INative::from_str_radix("", 10).is_err());
    assert!(INative::from_str_radix("g", 16).is_err());
    assert!(INative::from_str_radix(" 1", 10).is_err());
}

#[test]
fn const_eq() {
    // Bind to locals so the `const` evaluation is exercised without tripping
    // clippy's `assertions_on_constants` lint.
    const A: bool = INative::ZERO.const_eq(INative::ZERO);
    const B: bool = INative::ZERO.const_eq(INative::MAX);
    let (a, b) = (A, B);
    assert!(a);
    assert!(!b);
    assert!(INative::from(-7i8).const_eq(INative::from(-7i8)));
    assert!(!INative::from(-7i8).const_eq(INative::from(7i8)));
}

#[test]
fn const_cmp() {
    const C: Ordering = INative::MIN.const_cmp(INative::MAX);
    let c = C;
    assert_eq!(c, Ordering::Less);
    assert_eq!(
        INative::from(-7i8).const_cmp(INative::from(-7i8)),
        Ordering::Equal
    );
    assert_eq!(
        INative::from(7i8).const_cmp(INative::from(-7i8)),
        Ordering::Greater
    );
}

#[test]
fn const_ordering_predicates() {
    let lo = INative::from(-7i8);
    let hi = INative::from(3i8);
    assert!(lo.const_lt(hi) && !hi.const_lt(lo) && !lo.const_lt(lo));
    assert!(lo.const_le(hi) && lo.const_le(lo) && !hi.const_le(lo));
    assert!(hi.const_gt(lo) && !lo.const_gt(hi) && !hi.const_gt(hi));
    assert!(hi.const_ge(lo) && hi.const_ge(hi) && !lo.const_ge(hi));
}

#[test]
fn const_min_max() {
    const MIN: INative = INative::MIN.const_min(INative::MAX);
    const MAX: INative = INative::MIN.const_max(INative::MAX);
    assert_eq!(MIN, INative::MIN);
    assert_eq!(MAX, INative::MAX);
    let x = INative::from(-7i8);
    assert_eq!(x.const_min(x), x);
    assert_eq!(x.const_max(x), x);
}

#[test]
fn const_clamp() {
    const C: INative = INative::from_i8(-20).const_clamp(INative::from_i8(-5), INative::from_i8(5));
    assert_eq!(C, INative::from(-5i8));
    let lo = INative::from(-5i8);
    let hi = INative::from(5i8);
    assert_eq!(INative::from(-9i8).const_clamp(lo, hi), lo);
    assert_eq!(INative::from(0i8).const_clamp(lo, hi), INative::ZERO);
    assert_eq!(INative::from(9i8).const_clamp(lo, hi), hi);
}

#[test]
#[should_panic]
fn const_clamp_min_greater_than_max() {
    let _ = INative::from(0i8).const_clamp(INative::from(5i8), INative::from(-5i8));
}

#[test]
fn const_bit_ops() {
    let a = INative::from(0b1100i8);
    let b = INative::from(0b1010i8);
    assert_eq!(a.const_bitand(b), INative::from(0b1000i8));
    assert_eq!(a.const_bitor(b), INative::from(0b1110i8));
    assert_eq!(a.const_bitxor(b), INative::from(0b0110i8));
    assert_eq!(INative::ZERO.const_not(), INative::from(-1i8));
    assert_eq!(INative::from(-1i8).const_not(), INative::ZERO);
}

#[test]
#[should_panic]
fn from_str_radix_invalid_radix_low() {
    let _ = INative::from_str_radix("0", 1);
}

#[test]
#[should_panic]
fn from_str_radix_invalid_radix_high() {
    let _ = INative::from_str_radix("0", 37);
}
