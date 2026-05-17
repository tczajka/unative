#![allow(clippy::op_ref)]

use core::cmp::Ordering;
use unative::{INative, UNative};

#[test]
fn from() {
    assert_eq!(u128::from(UNative::from(false)), 0u128);
    assert_eq!(u128::from(UNative::from(true)), 1u128);
    assert_eq!(u128::from(UNative::from(0u8)), 0u128);
    assert_eq!(u128::from(UNative::from(u8::MAX)), u128::from(u8::MAX));
    assert_eq!(u128::from(UNative::from(u16::MAX)), u128::from(u16::MAX));
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
