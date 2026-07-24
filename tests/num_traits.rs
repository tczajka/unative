//! Tests for the `num-traits` integration.

#![cfg(feature = "num-traits")]

use num_traits::{
    AsPrimitive, Bounded, ConstOne, ConstZero, FromPrimitive, Num, One, Pow, PrimInt, Signed,
    ToPrimitive, Unsigned, Zero,
    bounds::{LowerBounded, UpperBounded},
    ops::{
        bytes::{FromBytes, ToBytes},
        checked::{
            CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr,
            CheckedSub,
        },
        euclid::{CheckedEuclid, Euclid},
        mul_add::{MulAdd, MulAddAssign},
        overflowing::{OverflowingAdd, OverflowingMul, OverflowingSub},
        saturating::{Saturating, SaturatingAdd, SaturatingMul, SaturatingSub},
        wrapping::{WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub},
    },
};
use unative::{INative, UNative};

// Marker-trait bound checks. These will only compile if the impls exist.
fn require_unsigned<T: Unsigned>() {}
fn require_signed<T: Signed>() {}
fn require_num<T: Num>() {}
fn require_prim_int<T: PrimInt>() {}

#[test]
fn marker_traits() {
    require_unsigned::<UNative>();
    require_signed::<INative>();
    require_num::<UNative>();
    require_num::<INative>();
    require_prim_int::<UNative>();
    require_prim_int::<INative>();
}

#[test]
fn zero_one() {
    assert_eq!(<UNative as Zero>::zero(), UNative::ZERO);
    assert_eq!(<INative as Zero>::zero(), INative::ZERO);
    assert!(<UNative as Zero>::is_zero(&UNative::ZERO));
    assert!(!<UNative as Zero>::is_zero(&UNative::from(1u8)));

    assert_eq!(<UNative as One>::one(), UNative::from(1u8));
    assert_eq!(<INative as One>::one(), INative::from(1i8));
    assert!(<UNative as One>::is_one(&UNative::from(1u8)));
    assert!(!<UNative as One>::is_one(&UNative::ZERO));
}

#[test]
fn const_zero_one() {
    assert_eq!(<UNative as ConstZero>::ZERO, UNative::ZERO);
    assert_eq!(<INative as ConstZero>::ZERO, INative::ZERO);
    assert_eq!(<UNative as ConstOne>::ONE, UNative::from(1u8));
    assert_eq!(<INative as ConstOne>::ONE, INative::from(1i8));
}

#[test]
fn bounded() {
    assert_eq!(<UNative as Bounded>::min_value(), UNative::MIN);
    assert_eq!(<UNative as Bounded>::max_value(), UNative::MAX);
    assert_eq!(<INative as Bounded>::min_value(), INative::MIN);
    assert_eq!(<INative as Bounded>::max_value(), INative::MAX);

    assert_eq!(<UNative as LowerBounded>::min_value(), UNative::MIN);
    assert_eq!(<UNative as UpperBounded>::max_value(), UNative::MAX);
    assert_eq!(<INative as LowerBounded>::min_value(), INative::MIN);
    assert_eq!(<INative as UpperBounded>::max_value(), INative::MAX);
}

#[test]
fn signed_methods() {
    let neg3 = INative::from(-3i8);
    let pos3 = INative::from(3i8);
    let zero = INative::ZERO;

    assert_eq!(<INative as Signed>::abs(&neg3), pos3);
    assert_eq!(<INative as Signed>::abs(&pos3), pos3);
    assert_eq!(<INative as Signed>::signum(&neg3), INative::from(-1i8));
    assert_eq!(<INative as Signed>::signum(&pos3), INative::from(1i8));
    assert_eq!(<INative as Signed>::signum(&zero), zero);
    assert!(<INative as Signed>::is_positive(&pos3));
    assert!(!<INative as Signed>::is_positive(&zero));
    assert!(<INative as Signed>::is_negative(&neg3));
    assert!(!<INative as Signed>::is_negative(&zero));

    assert_eq!(
        <INative as Signed>::abs_sub(&pos3, &INative::from(1i8)),
        INative::from(2i8)
    );
    assert_eq!(
        <INative as Signed>::abs_sub(&INative::from(1i8), &pos3),
        zero
    );
}

#[test]
fn num_from_str_radix() {
    assert_eq!(
        <UNative as Num>::from_str_radix("ff", 16).unwrap(),
        UNative::from(255u8)
    );
    assert_eq!(
        <UNative as Num>::from_str_radix("101", 2).unwrap(),
        UNative::from(5u8)
    );
    assert!(<UNative as Num>::from_str_radix("-1", 10).is_err());
    assert_eq!(
        <INative as Num>::from_str_radix("-7f", 16).unwrap(),
        INative::from(-127i8)
    );
}

#[test]
fn to_primitive() {
    let x = UNative::from(42u8);
    assert_eq!(<UNative as ToPrimitive>::to_i64(&x), Some(42));
    assert_eq!(<UNative as ToPrimitive>::to_u64(&x), Some(42));
    assert_eq!(<UNative as ToPrimitive>::to_i128(&x), Some(42));
    assert_eq!(<UNative as ToPrimitive>::to_u128(&x), Some(42));
    assert_eq!(<UNative as ToPrimitive>::to_u8(&x), Some(42));
    assert_eq!(<UNative as ToPrimitive>::to_i8(&x), Some(42));

    let big = UNative::MAX;
    assert_eq!(<UNative as ToPrimitive>::to_u8(&big), None);

    let neg = INative::from(-1i8);
    assert_eq!(<INative as ToPrimitive>::to_i64(&neg), Some(-1));
    assert_eq!(<INative as ToPrimitive>::to_u64(&neg), None);
    assert_eq!(<INative as ToPrimitive>::to_u128(&neg), None);
    assert_eq!(<INative as ToPrimitive>::to_i128(&neg), Some(-1));
}

#[test]
fn from_primitive() {
    assert_eq!(
        <UNative as FromPrimitive>::from_u64(42),
        Some(UNative::from(42u8))
    );
    assert_eq!(<UNative as FromPrimitive>::from_i64(-1), None);
    assert_eq!(
        <INative as FromPrimitive>::from_i64(-3),
        Some(INative::from(-3i8))
    );
    assert_eq!(<INative as FromPrimitive>::from_u64(u64::MAX), None);
    assert_eq!(<UNative as FromPrimitive>::from_u128(u128::MAX), None);

    assert_eq!(
        <UNative as FromPrimitive>::from_i128(42),
        Some(UNative::from(42u8))
    );
    assert_eq!(<UNative as FromPrimitive>::from_i128(-1), None);
    assert_eq!(
        <INative as FromPrimitive>::from_i128(-100),
        Some(INative::from(-100i8))
    );
    assert_eq!(<INative as FromPrimitive>::from_u128(u128::MAX), None);
}

#[test]
fn num_cast() {
    let from_i32 = <UNative as num_traits::NumCast>::from(42i32);
    assert_eq!(from_i32, Some(UNative::from(42u8)));

    let from_neg = <UNative as num_traits::NumCast>::from(-1i32);
    assert_eq!(from_neg, None);

    let from_u8 = <INative as num_traits::NumCast>::from(7u8);
    assert_eq!(from_u8, Some(INative::from(7i8)));
}

#[test]
fn as_primitive() {
    let x = UNative::from(255u8);
    let as_u8: u8 = x.as_();
    assert_eq!(as_u8, 255);

    let as_i64: i64 = x.as_();
    assert_eq!(as_i64, 255);

    let from_u8: UNative = 7u8.as_();
    assert_eq!(from_u8, UNative::from(7u8));

    let from_f64: INative = (-3.5f64).as_();
    assert_eq!(from_f64, INative::from(-3i8));

    let from_u_to_i: INative = UNative::from(5u8).as_();
    assert_eq!(from_u_to_i, INative::from(5i8));

    let from_i_to_u: UNative = INative::from(5i8).as_();
    assert_eq!(from_i_to_u, UNative::from(5u8));
}

#[test]
fn checked_ops() {
    let one = UNative::from(1u8);
    assert_eq!(
        <UNative as CheckedAdd>::checked_add(&UNative::MAX, &one),
        None
    );
    assert_eq!(
        <UNative as CheckedAdd>::checked_add(&one, &one),
        Some(UNative::from(2u8))
    );
    assert_eq!(
        <UNative as CheckedSub>::checked_sub(&UNative::ZERO, &one),
        None
    );
    assert_eq!(
        <UNative as CheckedMul>::checked_mul(&UNative::MAX, &UNative::from(2u8)),
        None
    );
    assert_eq!(
        <UNative as CheckedDiv>::checked_div(&one, &UNative::ZERO),
        None
    );
    assert_eq!(
        <UNative as CheckedRem>::checked_rem(&one, &UNative::ZERO),
        None
    );

    assert_eq!(<UNative as CheckedNeg>::checked_neg(&one), None);
    assert_eq!(<INative as CheckedNeg>::checked_neg(&INative::MIN), None);
    assert_eq!(
        <INative as CheckedNeg>::checked_neg(&INative::from(3i8)),
        Some(INative::from(-3i8))
    );

    assert_eq!(
        <UNative as CheckedShl>::checked_shl(&one, UNative::BITS),
        None
    );
    assert_eq!(
        <UNative as CheckedShl>::checked_shl(&one, 1),
        Some(UNative::from(2u8))
    );
    assert_eq!(
        <UNative as CheckedShr>::checked_shr(&one, UNative::BITS),
        None
    );
}

#[test]
fn checked_euclid() {
    let a = INative::from(-7i8);
    let b = INative::from(3i8);
    assert_eq!(
        CheckedEuclid::checked_div_euclid(&a, &b),
        Some(INative::from(-3i8))
    );
    assert_eq!(
        CheckedEuclid::checked_rem_euclid(&a, &b),
        Some(INative::from(2i8))
    );
    assert_eq!(CheckedEuclid::checked_div_euclid(&a, &INative::ZERO), None);
    assert_eq!(CheckedEuclid::checked_rem_euclid(&a, &INative::ZERO), None);
}

#[test]
fn wrapping_ops() {
    let one = UNative::from(1u8);
    assert_eq!(
        <UNative as WrappingAdd>::wrapping_add(&UNative::MAX, &one),
        UNative::ZERO
    );
    assert_eq!(
        <UNative as WrappingSub>::wrapping_sub(&UNative::ZERO, &one),
        UNative::MAX
    );
    assert_eq!(
        <UNative as WrappingMul>::wrapping_mul(&UNative::MAX, &UNative::from(2u8)),
        UNative::MAX - one
    );

    assert_eq!(
        <UNative as WrappingNeg>::wrapping_neg(&UNative::ZERO),
        UNative::ZERO
    );
    assert_eq!(
        <INative as WrappingNeg>::wrapping_neg(&INative::MIN),
        INative::MIN
    );

    assert_eq!(
        <UNative as WrappingShl>::wrapping_shl(&one, UNative::BITS),
        one
    );
    assert_eq!(
        <UNative as WrappingShr>::wrapping_shr(&one, UNative::BITS),
        one
    );
}

#[test]
fn saturating_ops() {
    let one = UNative::from(1u8);
    assert_eq!(
        <UNative as SaturatingAdd>::saturating_add(&UNative::MAX, &one),
        UNative::MAX
    );
    assert_eq!(
        <UNative as SaturatingSub>::saturating_sub(&UNative::ZERO, &one),
        UNative::ZERO
    );
    assert_eq!(
        <UNative as SaturatingMul>::saturating_mul(&UNative::MAX, &UNative::from(2u8)),
        UNative::MAX
    );

    assert_eq!(
        <INative as SaturatingAdd>::saturating_add(&INative::MAX, &INative::from(1i8)),
        INative::MAX
    );
    assert_eq!(
        <INative as SaturatingSub>::saturating_sub(&INative::MIN, &INative::from(1i8)),
        INative::MIN
    );
}

#[test]
#[allow(deprecated)]
fn saturating_deprecated_trait() {
    let one_u = UNative::from(1u8);
    assert_eq!(
        <UNative as Saturating>::saturating_add(UNative::MAX, one_u),
        UNative::MAX
    );
    assert_eq!(
        <UNative as Saturating>::saturating_sub(UNative::ZERO, one_u),
        UNative::ZERO
    );

    let one_i = INative::from(1i8);
    assert_eq!(
        <INative as Saturating>::saturating_add(INative::MAX, one_i),
        INative::MAX
    );
    assert_eq!(
        <INative as Saturating>::saturating_sub(INative::MIN, one_i),
        INative::MIN
    );
}

#[test]
fn overflowing_ops() {
    let one = UNative::from(1u8);
    assert_eq!(
        <UNative as OverflowingAdd>::overflowing_add(&UNative::MAX, &one),
        (UNative::ZERO, true)
    );
    assert_eq!(
        <UNative as OverflowingAdd>::overflowing_add(&one, &one),
        (UNative::from(2u8), false)
    );
    assert_eq!(
        <UNative as OverflowingSub>::overflowing_sub(&UNative::ZERO, &one),
        (UNative::MAX, true)
    );
    assert_eq!(
        <UNative as OverflowingMul>::overflowing_mul(&UNative::MAX, &UNative::from(2u8)),
        (UNative::MAX - one, true)
    );
}

#[test]
fn euclid_ops() {
    let a = INative::from(-7i8);
    let b = INative::from(3i8);
    assert_eq!(Euclid::div_euclid(&a, &b), INative::from(-3i8));
    assert_eq!(Euclid::rem_euclid(&a, &b), INative::from(2i8));

    let u = UNative::from(7u8);
    let v = UNative::from(3u8);
    assert_eq!(Euclid::div_euclid(&u, &v), UNative::from(2u8));
    assert_eq!(Euclid::rem_euclid(&u, &v), UNative::from(1u8));
}

#[test]
fn pow_ops() {
    assert_eq!(
        <UNative as Pow<u32>>::pow(UNative::from(2u8), 5),
        UNative::from(32u8)
    );
    assert_eq!(
        <UNative as Pow<u8>>::pow(UNative::from(3u8), 4),
        UNative::from(81u8)
    );
    assert_eq!(
        <UNative as Pow<u16>>::pow(UNative::from(2u8), 8),
        UNative::from(256u16)
    );

    assert_eq!(
        <INative as Pow<u32>>::pow(INative::from(-2i8), 3),
        INative::from(-8i8)
    );
    assert_eq!(
        <INative as Pow<u32>>::pow(INative::from(-2i8), 4),
        INative::from(16i8)
    );
}

#[test]
fn mul_add() {
    let x = UNative::from(3u8);
    let a = UNative::from(4u8);
    let b = UNative::from(5u8);
    assert_eq!(<UNative as MulAdd>::mul_add(x, a, b), UNative::from(17u8));

    let mut y = UNative::from(3u8);
    <UNative as MulAddAssign>::mul_add_assign(&mut y, a, b);
    assert_eq!(y, UNative::from(17u8));
}

#[test]
fn prim_int_bit_counts() {
    let x = UNative::from(0b1011_0101u8);
    assert_eq!(<UNative as PrimInt>::count_ones(x), 5);
    assert_eq!(<UNative as PrimInt>::count_zeros(x), UNative::BITS - 5);
    assert_eq!(<UNative as PrimInt>::trailing_ones(x), 1);
    assert_eq!(<UNative as PrimInt>::trailing_zeros(x), 0);

    assert_eq!(
        <UNative as PrimInt>::leading_zeros(UNative::ZERO),
        UNative::BITS
    );
    assert_eq!(
        <UNative as PrimInt>::leading_ones(UNative::MAX),
        UNative::BITS
    );
}

#[test]
fn prim_int_rotate() {
    let x = UNative::from(1u8);
    assert_eq!(<UNative as PrimInt>::rotate_left(x, 1), UNative::from(2u8));
    assert_eq!(<UNative as PrimInt>::rotate_right(UNative::from(2u8), 1), x);
}

#[test]
fn prim_int_swap_bytes_reverse_bits() {
    let x = UNative::from(0xffu8);
    let swapped = <UNative as PrimInt>::swap_bytes(x);
    assert_eq!(<UNative as PrimInt>::swap_bytes(swapped), x);

    let reversed = <UNative as PrimInt>::reverse_bits(x);
    assert_eq!(<UNative as PrimInt>::reverse_bits(reversed), x);
}

#[test]
fn prim_int_endian() {
    let x = UNative::from(1u8);
    assert_eq!(
        <UNative as PrimInt>::from_be(<UNative as PrimInt>::to_be(x)),
        x
    );
    assert_eq!(
        <UNative as PrimInt>::from_le(<UNative as PrimInt>::to_le(x)),
        x
    );
}

#[test]
fn prim_int_pow() {
    assert_eq!(
        <UNative as PrimInt>::pow(UNative::from(2u8), 4),
        UNative::from(16u8)
    );
    assert_eq!(
        <INative as PrimInt>::pow(INative::from(-2i8), 3),
        INative::from(-8i8)
    );
}

#[test]
fn prim_int_shifts() {
    let one = UNative::from(1u8);

    // Left shifts (signed and unsigned produce identical bit patterns).
    assert_eq!(
        <UNative as PrimInt>::unsigned_shl(one, 3),
        UNative::from(8u8)
    );
    assert_eq!(<UNative as PrimInt>::signed_shl(one, 3), UNative::from(8u8));
    assert_eq!(
        <INative as PrimInt>::signed_shl(INative::from(-1i8), 1),
        INative::from(-2i8)
    );
    assert_eq!(
        <INative as PrimInt>::unsigned_shl(INative::from(-1i8), 1),
        INative::from(-2i8)
    );

    // Right shifts: logical vs arithmetic.
    assert_eq!(
        <UNative as PrimInt>::unsigned_shr(UNative::from(8u8), 3),
        one
    );

    // signed_shr on unsigned with high bit set: arithmetic shift extends the sign bit.
    let high_bit = UNative::from(1u8) << (UNative::BITS - 1);
    assert_eq!(
        <UNative as PrimInt>::signed_shr(high_bit, 1),
        high_bit | (high_bit >> 1u32)
    );

    // On signed types: signed_shr fills with sign bit.
    let neg = INative::from(-2i8);
    assert_eq!(
        <INative as PrimInt>::signed_shr(neg, 1),
        INative::from(-1i8)
    );

    // unsigned_shr on negative INative fills with zeros (logical).
    let logical = <INative as PrimInt>::unsigned_shr(INative::from(-1i8), 1);
    assert_eq!(logical.cast_unsigned(), UNative::MAX >> 1u32);
}

#[test]
fn from_to_bytes_roundtrip() {
    let x = UNative::from(0x1234u16);
    let be = <UNative as ToBytes>::to_be_bytes(&x);
    assert_eq!(<UNative as FromBytes>::from_be_bytes(&be), x);

    let le = <UNative as ToBytes>::to_le_bytes(&x);
    assert_eq!(<UNative as FromBytes>::from_le_bytes(&le), x);

    let ne = <UNative as ToBytes>::to_ne_bytes(&x);
    assert_eq!(<UNative as FromBytes>::from_ne_bytes(&ne), x);

    // Endianness sanity check: BE and LE differ for multi-byte values.
    assert_ne!(be, le);
}

#[test]
fn from_to_bytes_signed() {
    let x = INative::from(-1i8);
    let be = <INative as ToBytes>::to_be_bytes(&x);
    assert_eq!(<INative as FromBytes>::from_be_bytes(&be), x);
    // All bits set for -1.
    assert!(be.iter().all(|&b| b == 0xff));
}
