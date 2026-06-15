use crate::UNative;
use crate::inner::INativeInner;
use crate::native::{
    define_native, delegate_from_native_prim, delegate_from_prim_native,
    delegate_try_from_native_native, delegate_try_from_native_prim, delegate_try_from_prim_native,
    delegate_unary_op,
};

define_native! {
    /// Native signed integer type.
    pub struct INative(INativeInner);
}

impl INative {
    /// Converts an [`i8`] to an [`INative`].
    pub const fn from_i8(value: i8) -> Self {
        Self(value as INativeInner)
    }

    /// Converts an [`i16`] to an [`INative`].
    #[allow(clippy::unnecessary_cast)]
    pub const fn from_i16(value: i16) -> Self {
        Self(value as INativeInner)
    }

    /// Converts an [`i32`] to an [`INative`], returning `None` if the value is out of range.
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_from_i32(value: i32) -> Option<Self> {
        let inner = value as INativeInner;
        if inner as i32 == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts an [`i64`] to an [`INative`], returning `None` if the value is out of range.
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_from_i64(value: i64) -> Option<Self> {
        let inner = value as INativeInner;
        if inner as i64 == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts an [`i128`] to an [`INative`], returning `None` if the value is out of range.
    pub const fn try_from_i128(value: i128) -> Option<Self> {
        let inner = value as INativeInner;
        if inner as i128 == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts an [`isize`] to an [`INative`], returning `None` if the value is out of range.
    pub const fn try_from_isize(value: isize) -> Option<Self> {
        let inner = value as INativeInner;
        if inner as isize == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts `self` to an [`i8`], returning `None` if the value is out of range.
    pub const fn try_to_i8(self) -> Option<i8> {
        let value = self.0 as i8;
        if value as INativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Converts `self` to an [`i16`], returning `None` if the value is out of range.
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_to_i16(self) -> Option<i16> {
        let value = self.0 as i16;
        if value as INativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Converts `self` to an [`i32`], returning `None` if the value is out of range.
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_to_i32(self) -> Option<i32> {
        let value = self.0 as i32;
        if value as INativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Converts `self` to an [`i64`].
    #[allow(clippy::unnecessary_cast)]
    pub const fn to_i64(self) -> i64 {
        self.0 as i64
    }

    /// Converts `self` to an [`i128`].
    pub const fn to_i128(self) -> i128 {
        self.0 as i128
    }

    /// Converts `self` to an [`isize`], returning `None` if the value is out of range.
    pub const fn try_to_isize(self) -> Option<isize> {
        let value = self.0 as isize;
        if value as INativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.
    pub const fn cast_unsigned(self) -> UNative {
        UNative(self.0.cast_unsigned())
    }

    /// Computes the absolute value of `self`.
    ///
    /// # Panics
    ///
    /// This function will panic if `self == INative::MIN`, because the result cannot be
    /// represented.
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Computes the absolute value of `self` without any wrapping or panicking, returning
    /// the result as an unsigned integer.
    pub const fn unsigned_abs(self) -> UNative {
        UNative(self.0.unsigned_abs())
    }

    /// Returns a number representing the sign of `self`: `0` if zero, `1` if positive, and
    /// `-1` if negative.
    pub const fn signum(self) -> Self {
        Self(self.0.signum())
    }

    /// Returns `true` if `self` is positive (greater than zero) and `false` if it is zero
    /// or negative.
    pub const fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    /// Returns `true` if `self` is negative (less than zero) and `false` if it is zero or
    /// positive.
    pub const fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// Checked addition of an unsigned integer. Computes `self + rhs`, returning `None` if
    /// overflow occurred.
    pub const fn checked_add_unsigned(self, rhs: UNative) -> Option<Self> {
        match self.0.checked_add_unsigned(rhs.0) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Checked subtraction of an unsigned integer. Computes `self - rhs`, returning `None` if
    /// overflow occurred.
    pub const fn checked_sub_unsigned(self, rhs: UNative) -> Option<Self> {
        match self.0.checked_sub_unsigned(rhs.0) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Returns the square root of `self`, rounded down. Returns `None` if `self` is negative.
    pub const fn checked_isqrt(self) -> Option<Self> {
        match self.0.checked_isqrt() {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Checked absolute value. Returns `None` if `self == INative::MIN`.
    pub const fn checked_abs(self) -> Option<Self> {
        match self.0.checked_abs() {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Calculates `self + rhs` with an unsigned `rhs`. Returns a tuple of the sum along
    /// with a boolean indicating whether an arithmetic overflow occurred. If an overflow
    /// occurred then the wrapped value is returned.
    pub const fn overflowing_add_unsigned(self, rhs: UNative) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_add_unsigned(rhs.0);
        (Self(x), overflow)
    }

    /// Calculates `self - rhs` with an unsigned `rhs`. Returns a tuple of the difference
    /// along with a boolean indicating whether an arithmetic overflow occurred. If an
    /// overflow occurred then the wrapped value is returned.
    pub const fn overflowing_sub_unsigned(self, rhs: UNative) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_sub_unsigned(rhs.0);
        (Self(x), overflow)
    }

    /// Computes the absolute value of `self`. Returns a tuple of the absolute value along
    /// with a boolean indicating whether an arithmetic overflow occurred. If an overflow
    /// occurred (that is, `self == INative::MIN`) then the wrapped value is returned.
    pub const fn overflowing_abs(self) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_abs();
        (Self(x), overflow)
    }

    /// Saturating addition of an unsigned integer. Computes `self + rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    pub const fn saturating_add_unsigned(self, rhs: UNative) -> Self {
        Self(self.0.saturating_add_unsigned(rhs.0))
    }

    /// Saturating subtraction of an unsigned integer. Computes `self - rhs`, saturating at
    /// the numeric bounds instead of overflowing.
    pub const fn saturating_sub_unsigned(self, rhs: UNative) -> Self {
        Self(self.0.saturating_sub_unsigned(rhs.0))
    }

    /// Saturating absolute value. Computes `self.abs()`, returning `INative::MAX` if
    /// `self == INative::MIN` instead of overflowing.
    pub const fn saturating_abs(self) -> Self {
        Self(self.0.saturating_abs())
    }

    /// Saturating negation. Computes `-self`, returning `INative::MAX` if
    /// `self == INative::MIN` instead of overflowing.
    pub const fn saturating_neg(self) -> Self {
        Self(self.0.saturating_neg())
    }

    /// Wrapping (modular) addition of an unsigned integer. Computes `self + rhs`, wrapping
    /// around at the boundary of the type.
    pub const fn wrapping_add_unsigned(self, rhs: UNative) -> Self {
        Self(self.0.wrapping_add_unsigned(rhs.0))
    }

    /// Wrapping (modular) subtraction of an unsigned integer. Computes `self - rhs`,
    /// wrapping around at the boundary of the type.
    pub const fn wrapping_sub_unsigned(self, rhs: UNative) -> Self {
        Self(self.0.wrapping_sub_unsigned(rhs.0))
    }

    /// Wrapping (modular) absolute value. Computes `self.abs()`, wrapping around at the
    /// boundary of the type. The only case where this wraps is when `self == INative::MIN`,
    /// in which case the result is `INative::MIN` itself.
    pub const fn wrapping_abs(self) -> Self {
        Self(self.0.wrapping_abs())
    }

    /// Strict addition of an unsigned integer. Computes `self + rhs`, panicking if overflow
    /// occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether overflow checks are
    /// enabled.
    pub const fn strict_add_unsigned(self, rhs: UNative) -> Self {
        Self(self.0.strict_add_unsigned(rhs.0))
    }

    /// Strict subtraction of an unsigned integer. Computes `self - rhs`, panicking if
    /// overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether overflow checks are
    /// enabled.
    pub const fn strict_sub_unsigned(self, rhs: UNative) -> Self {
        Self(self.0.strict_sub_unsigned(rhs.0))
    }

    /// Strict absolute value. Computes `self.abs()`, panicking if `self == INative::MIN`.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether overflow checks
    /// are enabled.
    pub const fn strict_abs(self) -> Self {
        Self(self.0.strict_abs())
    }

    /// Unchecked negation. Computes `-self`, assuming overflow cannot occur.
    ///
    /// # Safety
    ///
    /// This results in undefined behavior when the result would overflow, i.e. when
    /// [`checked_neg`](Self::checked_neg) would return `None`.
    pub const unsafe fn unchecked_neg(self) -> Self {
        // SAFETY: Caller guarantees no overflow.
        Self(unsafe { self.0.unchecked_neg() })
    }
}

delegate_unary_op!(INative, Neg, neg, -);

delegate_from_native_prim!(INative, bool);
delegate_from_native_prim!(INative, u8);
delegate_from_native_prim!(INative, i8);
delegate_from_native_prim!(INative, i16);

delegate_from_prim_native!(i64, INative);
delegate_from_prim_native!(i128, INative);

delegate_try_from_native_prim!(INative, u16);
delegate_try_from_native_prim!(INative, u32);
delegate_try_from_native_prim!(INative, u64);
delegate_try_from_native_prim!(INative, u128);
delegate_try_from_native_prim!(INative, usize);
delegate_try_from_native_prim!(INative, i32);
delegate_try_from_native_prim!(INative, i64);
delegate_try_from_native_prim!(INative, i128);
delegate_try_from_native_prim!(INative, isize);

delegate_try_from_prim_native!(bool, INative);
delegate_try_from_prim_native!(u8, INative);
delegate_try_from_prim_native!(u16, INative);
delegate_try_from_prim_native!(u32, INative);
delegate_try_from_prim_native!(u64, INative);
delegate_try_from_prim_native!(u128, INative);
delegate_try_from_prim_native!(usize, INative);
delegate_try_from_prim_native!(i8, INative);
delegate_try_from_prim_native!(i16, INative);
delegate_try_from_prim_native!(i32, INative);
delegate_try_from_prim_native!(isize, INative);

delegate_try_from_native_native!(INative, UNative);
