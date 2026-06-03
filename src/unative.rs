use crate::INative;
use crate::inner::UNativeInner;
use crate::native::{
    define_native, delegate_from_native_prim, delegate_from_prim_native,
    delegate_try_from_native_native, delegate_try_from_native_prim, delegate_try_from_prim_native,
};

define_native! {
    /// Native unsigned integer type.
    pub struct UNative(UNativeInner);
}

impl UNative {
    /// Converts a [`u8`] to a [`UNative`].
    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        Self(value as UNativeInner)
    }

    /// Converts a [`u16`] to a [`UNative`].
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    pub const fn from_u16(value: u16) -> Self {
        Self(value as UNativeInner)
    }

    /// Converts a [`u32`] to a [`UNative`], returning `None` if the value is out of range.
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_from_u32(value: u32) -> Option<Self> {
        let inner = value as UNativeInner;
        if inner as u32 == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts a [`u64`] to a [`UNative`], returning `None` if the value is out of range.
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_from_u64(value: u64) -> Option<Self> {
        let inner = value as UNativeInner;
        if inner as u64 == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts a [`u128`] to a [`UNative`], returning `None` if the value is out of range.
    #[inline]
    pub const fn try_from_u128(value: u128) -> Option<Self> {
        let inner = value as UNativeInner;
        if inner as u128 == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts a [`usize`] to a [`UNative`], returning `None` if the value is out of range.
    #[inline]
    pub const fn try_from_usize(value: usize) -> Option<Self> {
        let inner = value as UNativeInner;
        if inner as usize == value {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Converts `self` to a [`u8`], returning `None` if the value is out of range.
    #[inline]
    pub const fn try_to_u8(self) -> Option<u8> {
        let value = self.0 as u8;
        if value as UNativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Converts `self` to a [`u16`], returning `None` if the value is out of range.
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_to_u16(self) -> Option<u16> {
        let value = self.0 as u16;
        if value as UNativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Converts `self` to a [`u32`], returning `None` if the value is out of range.
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    pub const fn try_to_u32(self) -> Option<u32> {
        let value = self.0 as u32;
        if value as UNativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Converts `self` to a [`u64`].
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    pub const fn to_u64(self) -> u64 {
        self.0 as u64
    }

    /// Converts `self` to a [`u128`].
    #[inline]
    pub const fn to_u128(self) -> u128 {
        self.0 as u128
    }

    /// Converts `self` to a [`usize`], returning `None` if the value is out of range.
    #[inline]
    pub const fn try_to_usize(self) -> Option<usize> {
        let value = self.0 as usize;
        if value as UNativeInner == self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the bit pattern of `self` reinterpreted as a signed integer of the same size.
    #[inline]
    pub const fn cast_signed(self) -> INative {
        INative(self.0.cast_signed())
    }

    /// Calculates the quotient of `self` and `rhs`, rounded up toward positive infinity.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline]
    pub const fn div_ceil(self, rhs: Self) -> Self {
        Self(self.0.div_ceil(rhs.0))
    }

    /// Calculates the smallest value greater than or equal to `self` that is a multiple of
    /// `rhs`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero, or if the result overflows.
    #[inline]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
        Self(self.0.next_multiple_of(rhs.0))
    }

    /// Returns `true` if `self` is an integer multiple of `rhs`, and `false` otherwise.
    ///
    /// This function is equivalent to `self % rhs == 0`, except that it will not panic for
    /// `rhs == 0`. Instead, `0.is_multiple_of(0) == true`, and for any non-zero `n`,
    /// `n.is_multiple_of(0) == false`.
    #[inline]
    pub const fn is_multiple_of(self, rhs: Self) -> bool {
        self.0.is_multiple_of(rhs.0)
    }

    /// Returns the smallest power of two greater than or equal to `self`.
    ///
    /// # Panics
    ///
    /// This function will panic if the next power of two is greater than `Self::MAX`.
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        Self(self.0.next_power_of_two())
    }

    /// Returns `true` if and only if `self == 2^k` for some `k`.
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
        self.0.is_power_of_two()
    }

    /// Checked addition of a signed integer. Computes `self + rhs`, returning `None` if
    /// overflow occurred.
    #[inline]
    pub const fn checked_add_signed(self, rhs: INative) -> Option<Self> {
        match self.0.checked_add_signed(rhs.0) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Checked subtraction of a signed integer. Computes `self - rhs`, returning `None` if
    /// overflow occurred.
    #[inline]
    pub const fn checked_sub_signed(self, rhs: INative) -> Option<Self> {
        match self.0.checked_sub_signed(rhs.0) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Computes the signed difference `self - rhs`, returning `None` if the result does not
    /// fit in [`INative`].
    #[inline]
    pub const fn checked_signed_diff(self, rhs: UNative) -> Option<INative> {
        match self.0.checked_signed_diff(rhs.0) {
            Some(x) => Some(INative(x)),
            None => None,
        }
    }

    /// Calculates the smallest value greater than or equal to `self` that is a multiple of
    /// `rhs`, returning `None` if `rhs` is zero or the result overflows.
    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        match self.0.checked_next_multiple_of(rhs.0) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Returns the smallest power of two greater than or equal to `self`, returning `None`
    /// if the result overflows.
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        match self.0.checked_next_power_of_two() {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Calculates `self + rhs` with a signed `rhs`. Returns a tuple of the sum along with a
    /// boolean indicating whether an arithmetic overflow occurred. If an overflow occurred
    /// then the wrapped value is returned.
    #[inline]
    pub const fn overflowing_add_signed(self, rhs: INative) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_add_signed(rhs.0);
        (Self(x), overflow)
    }

    /// Calculates `self - rhs` with a signed `rhs`. Returns a tuple of the difference along
    /// with a boolean indicating whether an arithmetic overflow occurred. If an overflow
    /// occurred then the wrapped value is returned.
    #[inline]
    pub const fn overflowing_sub_signed(self, rhs: INative) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_sub_signed(rhs.0);
        (Self(x), overflow)
    }

    /// Saturating addition of a signed integer. Computes `self + rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    #[inline]
    pub const fn saturating_add_signed(self, rhs: INative) -> Self {
        Self(self.0.saturating_add_signed(rhs.0))
    }

    /// Saturating subtraction of a signed integer. Computes `self - rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    #[inline]
    pub const fn saturating_sub_signed(self, rhs: INative) -> Self {
        Self(self.0.saturating_sub_signed(rhs.0))
    }

    /// Wrapping (modular) addition of a signed integer. Computes `self + rhs`, wrapping
    /// around at the boundary of the type.
    #[inline]
    pub const fn wrapping_add_signed(self, rhs: INative) -> Self {
        Self(self.0.wrapping_add_signed(rhs.0))
    }

    /// Wrapping (modular) subtraction of a signed integer. Computes `self - rhs`, wrapping
    /// around at the boundary of the type.
    #[inline]
    pub const fn wrapping_sub_signed(self, rhs: INative) -> Self {
        Self(self.0.wrapping_sub_signed(rhs.0))
    }

    /// Strict addition of a signed integer. Computes `self + rhs`, panicking if overflow
    /// occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether overflow checks are
    /// enabled.
    #[inline]
    pub const fn strict_add_signed(self, rhs: INative) -> Self {
        Self(self.0.strict_add_signed(rhs.0))
    }

    /// Strict subtraction of a signed integer. Computes `self - rhs`, panicking if overflow
    /// occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether overflow checks are
    /// enabled.
    #[inline]
    pub const fn strict_sub_signed(self, rhs: INative) -> Self {
        Self(self.0.strict_sub_signed(rhs.0))
    }

    /// Calculates `self - rhs - borrow` and returns a tuple containing the difference and the
    /// output borrow.
    #[inline]
    pub fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
        let (diff, borrow) = self.0.borrowing_sub(rhs.0, borrow);
        (Self(diff), borrow)
    }

    /// Calculates `self + rhs + carry` and returns a tuple containing the sum and the output
    /// carry.
    #[inline]
    pub fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
        let (sum, carry) = self.0.carrying_add(rhs.0, carry);
        (Self(sum), carry)
    }

    /// Calculates the "full multiplication" `self * rhs + carry` without the possibility to
    /// overflow.
    ///
    /// Returns a tuple containing the low and high bits of the result.
    #[inline]
    pub fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
        let (low, high) = self.0.carrying_mul(rhs.0, carry.0);
        (Self(low), Self(high))
    }

    /// Calculates `self * rhs + carry + add` without the possibility to overflow.
    ///
    /// Returns a tuple containing the low and high bits of the result.
    #[inline]
    pub fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (Self, Self) {
        let (low, high) = self.0.carrying_mul_add(rhs.0, carry.0, add.0);
        (Self(low), Self(high))
    }
}

delegate_from_native_prim!(UNative, bool);
delegate_from_native_prim!(UNative, u8);
delegate_from_native_prim!(UNative, u16);

delegate_from_prim_native!(u64, UNative);
delegate_from_prim_native!(u128, UNative);
delegate_from_prim_native!(i128, UNative);

delegate_try_from_native_prim!(UNative, u32);
delegate_try_from_native_prim!(UNative, u64);
delegate_try_from_native_prim!(UNative, u128);
delegate_try_from_native_prim!(UNative, usize);
delegate_try_from_native_prim!(UNative, i8);
delegate_try_from_native_prim!(UNative, i16);
delegate_try_from_native_prim!(UNative, i32);
delegate_try_from_native_prim!(UNative, i64);
delegate_try_from_native_prim!(UNative, i128);
delegate_try_from_native_prim!(UNative, isize);

delegate_try_from_prim_native!(bool, UNative);
delegate_try_from_prim_native!(u8, UNative);
delegate_try_from_prim_native!(u16, UNative);
delegate_try_from_prim_native!(u32, UNative);
delegate_try_from_prim_native!(usize, UNative);
delegate_try_from_prim_native!(i8, UNative);
delegate_try_from_prim_native!(i16, UNative);
delegate_try_from_prim_native!(i32, UNative);
delegate_try_from_prim_native!(i64, UNative);
delegate_try_from_prim_native!(isize, UNative);

delegate_try_from_native_native!(UNative, INative);
