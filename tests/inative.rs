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
fn default() {
    assert_eq!(INative::default(), INative::ZERO);
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
