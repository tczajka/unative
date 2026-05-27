use core::num::ParseIntError;
use num_traits::{
    AsPrimitive, Bounded, ConstOne, ConstZero, FromPrimitive, Num, NumCast, One, Pow, PrimInt,
    Signed, ToPrimitive, Unsigned, Zero,
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

use crate::inner::{INativeInner, UNativeInner};
use crate::{INative, UNative};

// ---------------------------------------------------------------------------
// Shared impls
//
// `impl_common!` emits traits whose bodies are similar between `UNative`
// and `INative`.
// ---------------------------------------------------------------------------

macro_rules! impl_common {
    ($t:ident, $inner:ty, $to_wide:ident, $from_wide:ident) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                Self::ZERO
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }

        impl ConstZero for $t {
            const ZERO: Self = Self::ZERO;
        }

        impl One for $t {
            #[inline]
            fn one() -> Self {
                Self(1)
            }

            #[inline]
            fn is_one(&self) -> bool {
                self.0 == 1
            }
        }

        impl ConstOne for $t {
            const ONE: Self = Self(1);
        }

        impl Bounded for $t {
            #[inline]
            fn min_value() -> Self {
                Self::MIN
            }

            #[inline]
            fn max_value() -> Self {
                Self::MAX
            }
        }

        impl Num for $t {
            type FromStrRadixErr = ParseIntError;

            #[inline]
            fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
                Self::from_str_radix(s, radix)
            }
        }

        impl FromPrimitive for $t {
            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                <$inner>::try_from(n).ok().map(Self)
            }

            #[inline]
            fn from_u64(n: u64) -> Option<Self> {
                <$inner>::try_from(n).ok().map(Self)
            }

            #[inline]
            fn from_i128(n: i128) -> Option<Self> {
                <$inner>::try_from(n).ok().map(Self)
            }

            #[inline]
            fn from_u128(n: u128) -> Option<Self> {
                <$inner>::try_from(n).ok().map(Self)
            }
        }

        impl NumCast for $t {
            #[inline]
            fn from<T: ToPrimitive>(n: T) -> Option<Self> {
                n.$to_wide().and_then(<Self as FromPrimitive>::$from_wide)
            }
        }

        impl CheckedAdd for $t {
            #[inline]
            fn checked_add(&self, v: &Self) -> Option<Self> {
                Self::checked_add(*self, *v)
            }
        }

        impl CheckedSub for $t {
            #[inline]
            fn checked_sub(&self, v: &Self) -> Option<Self> {
                Self::checked_sub(*self, *v)
            }
        }

        impl CheckedMul for $t {
            #[inline]
            fn checked_mul(&self, v: &Self) -> Option<Self> {
                Self::checked_mul(*self, *v)
            }
        }

        impl CheckedDiv for $t {
            #[inline]
            fn checked_div(&self, v: &Self) -> Option<Self> {
                Self::checked_div(*self, *v)
            }
        }

        impl CheckedRem for $t {
            #[inline]
            fn checked_rem(&self, v: &Self) -> Option<Self> {
                Self::checked_rem(*self, *v)
            }
        }

        impl CheckedNeg for $t {
            #[inline]
            fn checked_neg(&self) -> Option<Self> {
                Self::checked_neg(*self)
            }
        }

        impl CheckedShl for $t {
            #[inline]
            fn checked_shl(&self, v: u32) -> Option<Self> {
                Self::checked_shl(*self, v)
            }
        }

        impl CheckedShr for $t {
            #[inline]
            fn checked_shr(&self, v: u32) -> Option<Self> {
                Self::checked_shr(*self, v)
            }
        }

        impl CheckedEuclid for $t {
            #[inline]
            fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
                Self::checked_div_euclid(*self, *v)
            }

            #[inline]
            fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
                Self::checked_rem_euclid(*self, *v)
            }
        }

        impl WrappingAdd for $t {
            #[inline]
            fn wrapping_add(&self, v: &Self) -> Self {
                Self::wrapping_add(*self, *v)
            }
        }

        impl WrappingSub for $t {
            #[inline]
            fn wrapping_sub(&self, v: &Self) -> Self {
                Self::wrapping_sub(*self, *v)
            }
        }

        impl WrappingMul for $t {
            #[inline]
            fn wrapping_mul(&self, v: &Self) -> Self {
                Self::wrapping_mul(*self, *v)
            }
        }

        impl WrappingNeg for $t {
            #[inline]
            fn wrapping_neg(&self) -> Self {
                Self::wrapping_neg(*self)
            }
        }

        impl WrappingShl for $t {
            #[inline]
            fn wrapping_shl(&self, v: u32) -> Self {
                Self::wrapping_shl(*self, v)
            }
        }

        impl WrappingShr for $t {
            #[inline]
            fn wrapping_shr(&self, v: u32) -> Self {
                Self::wrapping_shr(*self, v)
            }
        }

        impl SaturatingAdd for $t {
            #[inline]
            fn saturating_add(&self, v: &Self) -> Self {
                Self::saturating_add(*self, *v)
            }
        }

        impl SaturatingSub for $t {
            #[inline]
            fn saturating_sub(&self, v: &Self) -> Self {
                Self::saturating_sub(*self, *v)
            }
        }

        impl SaturatingMul for $t {
            #[inline]
            fn saturating_mul(&self, v: &Self) -> Self {
                Self::saturating_mul(*self, *v)
            }
        }

        #[allow(deprecated)]
        impl Saturating for $t {
            #[inline]
            fn saturating_add(self, v: Self) -> Self {
                Self::saturating_add(self, v)
            }

            #[inline]
            fn saturating_sub(self, v: Self) -> Self {
                Self::saturating_sub(self, v)
            }
        }

        impl OverflowingAdd for $t {
            #[inline]
            fn overflowing_add(&self, v: &Self) -> (Self, bool) {
                Self::overflowing_add(*self, *v)
            }
        }

        impl OverflowingSub for $t {
            #[inline]
            fn overflowing_sub(&self, v: &Self) -> (Self, bool) {
                Self::overflowing_sub(*self, *v)
            }
        }

        impl OverflowingMul for $t {
            #[inline]
            fn overflowing_mul(&self, v: &Self) -> (Self, bool) {
                Self::overflowing_mul(*self, *v)
            }
        }

        impl Euclid for $t {
            #[inline]
            fn div_euclid(&self, v: &Self) -> Self {
                Self::div_euclid(*self, *v)
            }
            #[inline]
            fn rem_euclid(&self, v: &Self) -> Self {
                Self::rem_euclid(*self, *v)
            }
        }

        impl Pow<u8> for $t {
            type Output = Self;
            #[inline]
            fn pow(self, exp: u8) -> Self {
                Self::pow(self, exp.into())
            }
        }

        impl Pow<u16> for $t {
            type Output = Self;
            #[inline]
            fn pow(self, exp: u16) -> Self {
                Self::pow(self, exp.into())
            }
        }

        impl Pow<u32> for $t {
            type Output = Self;
            #[inline]
            fn pow(self, exp: u32) -> Self {
                Self::pow(self, exp)
            }
        }

        impl MulAdd<Self, Self> for $t {
            type Output = Self;
            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                self * a + b
            }
        }

        impl MulAddAssign<Self, Self> for $t {
            #[inline]
            fn mul_add_assign(&mut self, a: Self, b: Self) {
                *self = *self * a + b;
            }
        }

        impl FromBytes for $t {
            type Bytes = [u8; <$t>::BYTES];

            #[inline]
            fn from_be_bytes(bytes: &Self::Bytes) -> Self {
                Self::from_be_bytes(*bytes)
            }

            #[inline]
            fn from_le_bytes(bytes: &Self::Bytes) -> Self {
                Self::from_le_bytes(*bytes)
            }

            #[inline]
            fn from_ne_bytes(bytes: &Self::Bytes) -> Self {
                Self::from_ne_bytes(*bytes)
            }
        }

        impl ToBytes for $t {
            type Bytes = [u8; <$t>::BYTES];

            #[inline]
            fn to_be_bytes(&self) -> Self::Bytes {
                Self::to_be_bytes(*self)
            }

            #[inline]
            fn to_le_bytes(&self) -> Self::Bytes {
                Self::to_le_bytes(*self)
            }

            #[inline]
            fn to_ne_bytes(&self) -> Self::Bytes {
                Self::to_ne_bytes(*self)
            }
        }
    };
}

impl_common!(UNative, UNativeInner, to_u128, from_u128);
impl_common!(INative, INativeInner, to_i128, from_i128);

impl Unsigned for UNative {}

impl Signed for INative {
    #[inline]
    fn abs(&self) -> Self {
        INative::abs(*self)
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if *self <= *other {
            Self::ZERO
        } else {
            *self - *other
        }
    }

    #[inline]
    fn signum(&self) -> Self {
        INative::signum(*self)
    }

    #[inline]
    fn is_positive(&self) -> bool {
        INative::is_positive(*self)
    }

    #[inline]
    fn is_negative(&self) -> bool {
        INative::is_negative(*self)
    }
}

impl ToPrimitive for UNative {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        i64::try_from(self.0).ok()
    }

    #[inline]
    #[allow(clippy::useless_conversion)] // No-op when the inner is already u64.
    fn to_u64(&self) -> Option<u64> {
        Some(self.0.into())
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        Some(self.0.into())
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        Some(self.0.into())
    }
}

impl ToPrimitive for INative {
    #[inline]
    #[allow(clippy::useless_conversion)] // No-op when the inner is already i64.
    fn to_i64(&self) -> Option<i64> {
        Some(self.0.into())
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        u64::try_from(self.0).ok()
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        Some(self.0.into())
    }
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        u128::try_from(self.0).ok()
    }
}

// `impl_prim_int_common!()` emits the `PrimInt` methods that are identical
// between `UNative` and `INative`.
macro_rules! impl_prim_int_common {
    () => {
        #[inline]
        fn count_ones(self) -> u32 {
            Self::count_ones(self)
        }

        #[inline]
        fn count_zeros(self) -> u32 {
            Self::count_zeros(self)
        }

        #[inline]
        fn leading_ones(self) -> u32 {
            Self::leading_ones(self)
        }

        #[inline]
        fn leading_zeros(self) -> u32 {
            Self::leading_zeros(self)
        }

        #[inline]
        fn trailing_ones(self) -> u32 {
            Self::trailing_ones(self)
        }

        #[inline]
        fn trailing_zeros(self) -> u32 {
            Self::trailing_zeros(self)
        }

        #[inline]
        fn rotate_left(self, n: u32) -> Self {
            Self::rotate_left(self, n)
        }

        #[inline]
        fn rotate_right(self, n: u32) -> Self {
            Self::rotate_right(self, n)
        }

        #[inline]
        fn swap_bytes(self) -> Self {
            Self::swap_bytes(self)
        }

        #[inline]
        fn reverse_bits(self) -> Self {
            Self::reverse_bits(self)
        }

        #[inline]
        fn from_be(x: Self) -> Self {
            Self::from_be(x)
        }

        #[inline]
        fn from_le(x: Self) -> Self {
            Self::from_le(x)
        }

        #[inline]
        fn to_be(self) -> Self {
            Self::to_be(self)
        }

        #[inline]
        fn to_le(self) -> Self {
            Self::to_le(self)
        }

        #[inline]
        fn pow(self, exp: u32) -> Self {
            Self::pow(self, exp)
        }
    };
}

impl PrimInt for UNative {
    impl_prim_int_common!();

    #[inline]
    fn signed_shl(self, n: u32) -> Self {
        Self((self.0.cast_signed() << n).cast_unsigned())
    }

    #[inline]
    fn signed_shr(self, n: u32) -> Self {
        Self((self.0.cast_signed() >> n).cast_unsigned())
    }

    #[inline]
    fn unsigned_shl(self, n: u32) -> Self {
        Self(self.0 << n)
    }

    #[inline]
    fn unsigned_shr(self, n: u32) -> Self {
        Self(self.0 >> n)
    }
}

impl PrimInt for INative {
    impl_prim_int_common!();

    #[inline]
    fn signed_shl(self, n: u32) -> Self {
        Self(self.0 << n)
    }

    #[inline]
    fn signed_shr(self, n: u32) -> Self {
        Self(self.0 >> n)
    }

    #[inline]
    fn unsigned_shl(self, n: u32) -> Self {
        Self((self.0.cast_unsigned() << n).cast_signed())
    }

    #[inline]
    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0.cast_unsigned() >> n).cast_signed())
    }
}

macro_rules! impl_as_primitive_native_to_prim {
    ($native:ty, $($prim:ty),+ $(,)?) => {
        $(
            impl AsPrimitive<$prim> for $native {
                #[inline]
                fn as_(self) -> $prim {
                    self.0 as $prim
                }
            }
        )+
    };
}

macro_rules! impl_as_primitive_prim_to_native {
    ($native:ident, $inner:ty, $($prim:ty),+ $(,)?) => {
        $(
            impl AsPrimitive<$native> for $prim {
                #[inline]
                fn as_(self) -> $native {
                    $native(self as $inner)
                }
            }
        )+
    };
}

impl_as_primitive_native_to_prim!(
    UNative, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);
impl_as_primitive_native_to_prim!(
    INative, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);

impl_as_primitive_prim_to_native!(
    UNative,
    UNativeInner,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
);
impl_as_primitive_prim_to_native!(
    INative,
    INativeInner,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
);

impl AsPrimitive<UNative> for UNative {
    #[inline]
    fn as_(self) -> UNative {
        self
    }
}

impl AsPrimitive<INative> for INative {
    #[inline]
    fn as_(self) -> INative {
        self
    }
}

impl AsPrimitive<INative> for UNative {
    #[inline]
    fn as_(self) -> INative {
        INative(self.0 as INativeInner)
    }
}

impl AsPrimitive<UNative> for INative {
    #[inline]
    fn as_(self) -> UNative {
        UNative(self.0 as UNativeInner)
    }
}
