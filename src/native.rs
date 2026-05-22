macro_rules! delegate_binop {
    ($t:ident, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$t> for $t {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self {
                Self(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<&$t> for $t {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: &Self) -> Self {
                Self(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: $t) -> $t {
                $t(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<&$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: &$t) -> $t {
                $t(self.0 $op rhs.0)
            }
        }
    };
}

macro_rules! delegate_assign_op {
    ($t:ident, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$t> for $t {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                self.0 $op rhs.0;
            }
        }

        impl ::core::ops::$trait<&$t> for $t {
            #[inline]
            fn $method(&mut self, rhs: &Self) {
                self.0 $op rhs.0;
            }
        }
    };
}

macro_rules! delegate_unary_op {
    ($t:ident, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait for $t {
            type Output = Self;

            #[inline]
            fn $method(self) -> Self {
                Self($op self.0)
            }
        }

        impl ::core::ops::$trait for &$t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                $t($op self.0)
            }
        }
    };
}

macro_rules! delegate_shift_op_native_prim {
    ($lhs:ident, $rhs:ty, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$rhs> for $lhs {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: $rhs) -> Self {
                Self(self.0 $op rhs)
            }
        }

        impl ::core::ops::$trait<&$rhs> for $lhs {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: &$rhs) -> Self {
                Self(self.0 $op rhs)
            }
        }

        impl ::core::ops::$trait<$rhs> for &$lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: $rhs) -> $lhs {
                $lhs(self.0 $op rhs)
            }
        }

        impl ::core::ops::$trait<&$rhs> for &$lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: &$rhs) -> $lhs {
                $lhs(self.0 $op rhs)
            }
        }
    };
}

macro_rules! delegate_shift_assign_op_native_prim {
    ($lhs:ty, $rhs:ty, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: $rhs) {
                self.0 $op rhs;
            }
        }

        impl ::core::ops::$trait<&$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: &$rhs) {
                self.0 $op rhs;
            }
        }
    };
}

macro_rules! delegate_shift_op_native_native {
    ($lhs:ident, $rhs:ty, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$rhs> for $lhs {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: $rhs) -> Self {
                Self(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<&$rhs> for $lhs {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: &$rhs) -> Self {
                Self(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<$rhs> for &$lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: $rhs) -> $lhs {
                $lhs(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<&$rhs> for &$lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: &$rhs) -> $lhs {
                $lhs(self.0 $op rhs.0)
            }
        }
    };
}

macro_rules! delegate_shift_assign_op_native_native {
    ($lhs:ty, $rhs:ty, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: $rhs) {
                self.0 $op rhs.0;
            }
        }

        impl ::core::ops::$trait<&$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: &$rhs) {
                self.0 $op rhs.0;
            }
        }
    };
}

macro_rules! delegate_shift_op_prim_native {
    ($lhs:ty, $rhs:ty, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$rhs> for $lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: $rhs) -> $lhs {
                self $op rhs.0
            }
        }

        impl ::core::ops::$trait<&$rhs> for $lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: &$rhs) -> $lhs {
                self $op rhs.0
            }
        }

        impl ::core::ops::$trait<$rhs> for &$lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: $rhs) -> $lhs {
                *self $op rhs.0
            }
        }

        impl ::core::ops::$trait<&$rhs> for &$lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: &$rhs) -> $lhs {
                *self $op rhs.0
            }
        }
    };
}

macro_rules! delegate_shift_assign_op_prim_native {
    ($lhs:ty, $rhs:ty, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: $rhs) {
                *self $op rhs.0;
            }
        }

        impl ::core::ops::$trait<&$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: &$rhs) {
                *self $op rhs.0;
            }
        }
    };
}

macro_rules! delegate_shifts_native_prim {
    ($lhs:ident, $rhs:ty) => {
        $crate::native::delegate_shift_op_native_prim!($lhs, $rhs, Shl, shl, <<);
        $crate::native::delegate_shift_op_native_prim!($lhs, $rhs, Shr, shr, >>);
        $crate::native::delegate_shift_assign_op_native_prim!(
            $lhs, $rhs, ShlAssign, shl_assign, <<=);
        $crate::native::delegate_shift_assign_op_native_prim!(
            $lhs, $rhs, ShrAssign, shr_assign, >>=);
    };
}

macro_rules! delegate_shifts_native_native {
    ($lhs:ident, $rhs:ty) => {
        $crate::native::delegate_shift_op_native_native!($lhs, $rhs, Shl, shl, <<);
        $crate::native::delegate_shift_op_native_native!($lhs, $rhs, Shr, shr, >>);
        $crate::native::delegate_shift_assign_op_native_native!(
            $lhs, $rhs, ShlAssign, shl_assign, <<=);
        $crate::native::delegate_shift_assign_op_native_native!(
            $lhs, $rhs, ShrAssign, shr_assign, >>=);
    };
}

macro_rules! delegate_shifts_prim_native {
    ($lhs:ty, $rhs:ty) => {
        $crate::native::delegate_shift_op_prim_native!($lhs, $rhs, Shl, shl, <<);
        $crate::native::delegate_shift_op_prim_native!($lhs, $rhs, Shr, shr, >>);
        $crate::native::delegate_shift_assign_op_prim_native!(
            $lhs, $rhs, ShlAssign, shl_assign, <<=);
        $crate::native::delegate_shift_assign_op_prim_native!(
            $lhs, $rhs, ShrAssign, shr_assign, >>=);
    };
}

macro_rules! delegate_iter_op {
    ($t:ident, $trait:ident, $method:ident) => {
        impl ::core::iter::$trait for $t {
            #[inline]
            fn $method<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self(iter.map(|x| x.0).$method())
            }
        }

        impl<'a> ::core::iter::$trait<&'a $t> for $t {
            #[inline]
            fn $method<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                Self(iter.map(|x| x.0).$method())
            }
        }
    };
}

macro_rules! delegate_from_native_prim {
    ($into:ident, $from:ty) => {
        impl From<$from> for $into {
            #[inline]
            fn from(value: $from) -> Self {
                Self(From::from(value))
            }
        }
    };
}

macro_rules! delegate_from_prim_native {
    ($into:ty, $from:ty) => {
        impl From<$from> for $into {
            #[inline]
            fn from(value: $from) -> Self {
                From::from(value.0)
            }
        }
    };
}

macro_rules! delegate_try_from_native_prim {
    ($into:ty, $from:ty) => {
        impl TryFrom<$from> for $into {
            type Error = ::core::num::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> Result<Self, Self::Error> {
                TryFrom::try_from(value).map(Self).map_err(Into::into)
            }
        }
    };
}

macro_rules! delegate_try_from_native_native {
    ($into:ty, $from:ty) => {
        impl TryFrom<$from> for $into {
            type Error = ::core::num::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> Result<Self, Self::Error> {
                TryFrom::try_from(value.0).map(Self)
            }
        }
    };
}

macro_rules! delegate_try_from_prim_native {
    ($into:ty, $from:ty) => {
        impl TryFrom<$from> for $into {
            type Error = ::core::num::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> Result<Self, Self::Error> {
                TryFrom::try_from(value.0).map_err(Into::into)
            }
        }
    };
}

macro_rules! delegate_fmt {
    ($t:ident, $trait:ident) => {
        impl ::core::fmt::$trait for $t {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.0, f)
            }
        }
    };
}

macro_rules! define_native {
    ($(#[$attr:meta])* pub struct $t:ident($inner:ident);) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $t(pub(crate) $inner);

        impl $t {
            /// The smallest value that can be represented by this integer type.
            pub const MIN: Self = Self(<$inner>::MIN);

            /// The largest value that can be represented by this integer type.
            pub const MAX: Self = Self(<$inner>::MAX);

            /// The zero value of this integer type.
            pub const ZERO: Self = Self(0);

            /// The size of this integer type in bits.
            pub const BITS: u32 = <$inner>::BITS;

            /// The size of this integer type in bytes.
            pub const BYTES: usize = <$inner>::BITS as usize / 8;

            /// Returns the number of ones in the binary representation of `self`.
            #[inline]
            pub const fn count_ones(self) -> u32 {
                self.0.count_ones()
            }

            /// Returns the number of zeros in the binary representation of `self`.
            #[inline]
            pub const fn count_zeros(self) -> u32 {
                self.0.count_zeros()
            }

            /// Returns the number of leading zeros in the binary representation of `self`.
            #[inline]
            pub const fn leading_zeros(self) -> u32 {
                self.0.leading_zeros()
            }

            /// Returns the number of trailing zeros in the binary representation of `self`.
            #[inline]
            pub const fn trailing_zeros(self) -> u32 {
                self.0.trailing_zeros()
            }

            /// Returns the number of leading ones in the binary representation of `self`.
            #[inline]
            pub const fn leading_ones(self) -> u32 {
                self.0.leading_ones()
            }

            /// Returns the number of trailing ones in the binary representation of `self`.
            #[inline]
            pub const fn trailing_ones(self) -> u32 {
                self.0.trailing_ones()
            }

            /// Shifts the bits to the left by a specified amount, `n`, wrapping the truncated
            /// bits to the end of the resulting integer.
            #[inline]
            pub const fn rotate_left(self, n: u32) -> Self {
                Self(self.0.rotate_left(n))
            }

            /// Shifts the bits to the right by a specified amount, `n`, wrapping the truncated
            /// bits to the beginning of the resulting integer.
            #[inline]
            pub const fn rotate_right(self, n: u32) -> Self {
                Self(self.0.rotate_right(n))
            }

            /// Reverses the byte order of `self`.
            #[inline]
            pub const fn swap_bytes(self) -> Self {
                Self(self.0.swap_bytes())
            }

            /// Reverses the order of bits in `self`.
            #[inline]
            pub const fn reverse_bits(self) -> Self {
                Self(self.0.reverse_bits())
            }

            /// Converts an integer from big endian to the target's endianness.
            #[inline]
            pub const fn from_be(x: Self) -> Self {
                Self(<$inner>::from_be(x.0))
            }

            /// Converts an integer from little endian to the target's endianness.
            #[inline]
            pub const fn from_le(x: Self) -> Self {
                Self(<$inner>::from_le(x.0))
            }

            /// Converts `self` to big endian from the target's endianness.
            #[inline]
            pub const fn to_be(self) -> Self {
                Self(self.0.to_be())
            }

            /// Converts `self` to little endian from the target's endianness.
            #[inline]
            pub const fn to_le(self) -> Self {
                Self(self.0.to_le())
            }

            /// Creates an integer value from its representation as a byte array in big
            /// endian.
            #[inline]
            pub const fn from_be_bytes(bytes: [u8; Self::BYTES]) -> Self {
                Self(<$inner>::from_be_bytes(bytes))
            }

            /// Creates an integer value from its representation as a byte array in little
            /// endian.
            #[inline]
            pub const fn from_le_bytes(bytes: [u8; Self::BYTES]) -> Self {
                Self(<$inner>::from_le_bytes(bytes))
            }

            /// Creates an integer value from its memory representation as a byte array in
            /// native endianness.
            #[inline]
            pub const fn from_ne_bytes(bytes: [u8; Self::BYTES]) -> Self {
                Self(<$inner>::from_ne_bytes(bytes))
            }

            /// Returns the memory representation of `self` as a byte array in big-endian
            /// byte order.
            #[inline]
            pub const fn to_be_bytes(self) -> [u8; Self::BYTES] {
                self.0.to_be_bytes()
            }

            /// Returns the memory representation of `self` as a byte array in little-endian
            /// byte order.
            #[inline]
            pub const fn to_le_bytes(self) -> [u8; Self::BYTES] {
                self.0.to_le_bytes()
            }

            /// Returns the memory representation of `self` as a byte array in native byte
            /// order.
            #[inline]
            pub const fn to_ne_bytes(self) -> [u8; Self::BYTES] {
                self.0.to_ne_bytes()
            }

            /// Calculates the midpoint of `self` and `rhs`, rounded toward zero.
            #[inline]
            pub const fn midpoint(self, rhs: Self) -> Self {
                Self(self.0.midpoint(rhs.0))
            }

            /// Computes the absolute difference between `self` and `other`.
            #[inline]
            pub const fn abs_diff(self, other: Self) -> UNative {
                UNative(self.0.abs_diff(other.0))
            }

            /// Returns the logarithm of `self` with respect to an arbitrary `base`, rounded
            /// down.
            ///
            /// # Panics
            ///
            /// This function will panic if `self` is non-positive, or if `base` is less
            /// than 2.
            #[inline]
            pub const fn ilog(self, base: Self) -> u32 {
                self.0.ilog(base.0)
            }

            /// Returns the base-2 logarithm of `self`, rounded down.
            ///
            /// # Panics
            ///
            /// This function will panic if `self` is non-positive.
            #[inline]
            pub const fn ilog2(self) -> u32 {
                self.0.ilog2()
            }

            /// Returns the base-10 logarithm of `self`, rounded down.
            ///
            /// # Panics
            ///
            /// This function will panic if `self` is non-positive.
            #[inline]
            pub const fn ilog10(self) -> u32 {
                self.0.ilog10()
            }

            /// Raises `self` to the power of `exp`.
            #[inline]
            pub const fn pow(self, exp: u32) -> Self {
                Self(self.0.pow(exp))
            }

            /// Returns the square root of `self`, rounded down.
            ///
            /// # Panics
            ///
            /// For signed types, this function will panic if `self` is negative.
            #[inline]
            pub const fn isqrt(self) -> Self {
                Self(self.0.isqrt())
            }

            /// Calculates the quotient of Euclidean division of `self` by `rhs`.
            ///
            /// For unsigned types, this is exactly equal to `self / rhs`. For signed types,
            /// the result is rounded toward negative infinity.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero, or if the division overflows.
            #[inline]
            pub const fn div_euclid(self, rhs: Self) -> Self {
                Self(self.0.div_euclid(rhs.0))
            }

            /// Calculates the least nonnegative remainder of `self (mod rhs)`.
            ///
            /// For unsigned types, this is exactly equal to `self % rhs`. For signed types,
            /// the result is always nonnegative.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero, or if the division overflows.
            #[inline]
            pub const fn rem_euclid(self, rhs: Self) -> Self {
                Self(self.0.rem_euclid(rhs.0))
            }

            /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow
            /// occurred.
            #[inline]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                match self.0.checked_add(rhs.0) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow
            /// occurred.
            #[inline]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                match self.0.checked_sub(rhs.0) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked negation. Computes `-self`, returning `None` if overflow occurred.
            #[inline]
            pub const fn checked_neg(self) -> Option<Self> {
                match self.0.checked_neg() {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked integer multiplication. Computes `self * rhs`, returning `None` if
            /// overflow occurred.
            #[inline]
            pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
                match self.0.checked_mul(rhs.0) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs` is
            /// zero or the division overflows.
            #[inline]
            pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                match self.0.checked_div(rhs.0) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked Euclidean division. Computes `self.div_euclid(rhs)`, returning `None`
            /// if `rhs` is zero or the division overflows.
            #[inline]
            pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                match self.0.checked_div_euclid(rhs.0) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs` is
            /// zero or the division overflows.
            #[inline]
            pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                match self.0.checked_rem(rhs.0) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked Euclidean remainder. Computes `self.rem_euclid(rhs)`, returning `None`
            /// if `rhs` is zero or the division overflows.
            #[inline]
            pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                match self.0.checked_rem_euclid(rhs.0) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked left shift. Computes `self << rhs`, returning `None` if `rhs` is
            /// larger than or equal to the number of bits in `self`.
            #[inline]
            pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
                match self.0.checked_shl(rhs) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Checked right shift. Computes `self >> rhs`, returning `None` if `rhs` is
            /// larger than or equal to the number of bits in `self`.
            #[inline]
            pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
                match self.0.checked_shr(rhs) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Returns the logarithm of `self` with respect to an arbitrary `base`, rounded
            /// down. Returns `None` if `self` is non-positive or `base` is less than 2.
            #[inline]
            pub const fn checked_ilog(self, base: Self) -> Option<u32> {
                self.0.checked_ilog(base.0)
            }

            /// Returns the base-2 logarithm of `self`, rounded down. Returns `None` if
            /// `self` is non-positive.
            #[inline]
            pub const fn checked_ilog2(self) -> Option<u32> {
                self.0.checked_ilog2()
            }

            /// Returns the base-10 logarithm of `self`, rounded down. Returns `None` if
            /// `self` is non-positive.
            #[inline]
            pub const fn checked_ilog10(self) -> Option<u32> {
                self.0.checked_ilog10()
            }

            /// Checked exponentiation. Raises `self` to the power of `exp`, returning `None`
            /// if overflow occurred.
            #[inline]
            pub const fn checked_pow(self, exp: u32) -> Option<Self> {
                match self.0.checked_pow(exp) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            /// Calculates `self + rhs`. Returns a tuple of the sum along with a boolean
            /// indicating whether an arithmetic overflow occurred. If an overflow occurred
            /// then the wrapped value is returned.
            #[inline]
            pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_add(rhs.0);
                (Self(x), overflow)
            }

            /// Calculates `self - rhs`. Returns a tuple of the difference along with a
            /// boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred then the wrapped value is returned.
            #[inline]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_sub(rhs.0);
                (Self(x), overflow)
            }

            /// Negates `self`. Returns a tuple of the negated value along with a boolean
            /// indicating whether an arithmetic overflow occurred. If an overflow occurred
            /// then the wrapped value is returned.
            #[inline]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_neg();
                (Self(x), overflow)
            }

            /// Calculates `self * rhs`. Returns a tuple of the product along with a boolean
            /// indicating whether an arithmetic overflow occurred. If an overflow occurred
            /// then the wrapped value is returned.
            #[inline]
            pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_mul(rhs.0);
                (Self(x), overflow)
            }

            /// Calculates `self / rhs`. Returns a tuple of the quotient along with a boolean
            /// indicating whether an arithmetic overflow occurred. If an overflow occurred
            /// then the wrapped value is returned.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero.
            #[inline]
            pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_div(rhs.0);
                (Self(x), overflow)
            }

            /// Calculates `self.div_euclid(rhs)`. Returns a tuple of the quotient along with
            /// a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred then the wrapped value is returned.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero.
            #[inline]
            pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_div_euclid(rhs.0);
                (Self(x), overflow)
            }

            /// Calculates `self % rhs`. Returns a tuple of the remainder along with a boolean
            /// indicating whether an arithmetic overflow occurred. If an overflow occurred
            /// then the wrapped value is returned.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero.
            #[inline]
            pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_rem(rhs.0);
                (Self(x), overflow)
            }

            /// Calculates `self.rem_euclid(rhs)`. Returns a tuple of the remainder along with
            /// a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred then the wrapped value is returned.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero.
            #[inline]
            pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_rem_euclid(rhs.0);
                (Self(x), overflow)
            }

            /// Shifts `self` left by `rhs` bits. Returns a tuple of the shifted value along
            /// with a boolean indicating whether the shift amount was larger than or equal
            /// to the number of bits. If the shift amount is too large, then it is wrapped
            /// modulo the number of bits and the shift is performed with that value.
            #[inline]
            pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_shl(rhs);
                (Self(x), overflow)
            }

            /// Shifts `self` right by `rhs` bits. Returns a tuple of the shifted value along
            /// with a boolean indicating whether the shift amount was larger than or equal
            /// to the number of bits. If the shift amount is too large, then it is wrapped
            /// modulo the number of bits and the shift is performed with that value.
            #[inline]
            pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_shr(rhs);
                (Self(x), overflow)
            }

            /// Raises `self` to the power of `exp`. Returns a tuple of the result along with
            /// a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred then the wrapped value is returned.
            #[inline]
            pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                let (x, overflow) = self.0.overflowing_pow(exp);
                (Self(x), overflow)
            }

            /// Strict integer addition. Computes `self + rhs`, panicking if overflow occurred.
            ///
            /// # Panics
            ///
            /// This function will always panic on overflow, regardless of whether overflow
            /// checks are enabled.
            #[inline]
            pub const fn strict_add(self, rhs: Self) -> Self {
                Self(self.0.strict_add(rhs.0))
            }

            /// Strict integer subtraction. Computes `self - rhs`, panicking if overflow
            /// occurred.
            ///
            /// # Panics
            ///
            /// This function will always panic on overflow, regardless of whether overflow
            /// checks are enabled.
            #[inline]
            pub const fn strict_sub(self, rhs: Self) -> Self {
                Self(self.0.strict_sub(rhs.0))
            }

            /// Strict negation. Computes `-self`, panicking if overflow occurred.
            ///
            /// # Panics
            ///
            /// This function will always panic on overflow, regardless of whether overflow
            /// checks are enabled.
            #[inline]
            pub const fn strict_neg(self) -> Self {
                Self(self.0.strict_neg())
            }

            /// Strict integer multiplication. Computes `self * rhs`, panicking if overflow
            /// occurred.
            ///
            /// # Panics
            ///
            /// This function will always panic on overflow, regardless of whether overflow
            /// checks are enabled.
            #[inline]
            pub const fn strict_mul(self, rhs: Self) -> Self {
                Self(self.0.strict_mul(rhs.0))
            }

            /// Strict integer division. Computes `self / rhs`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero, or if the division overflows
            /// regardless of whether overflow checks are enabled.
            #[inline]
            pub const fn strict_div(self, rhs: Self) -> Self {
                Self(self.0.strict_div(rhs.0))
            }

            /// Strict Euclidean division. Computes `self.div_euclid(rhs)`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero, or if the division overflows
            /// regardless of whether overflow checks are enabled.
            #[inline]
            pub const fn strict_div_euclid(self, rhs: Self) -> Self {
                Self(self.0.strict_div_euclid(rhs.0))
            }

            /// Strict integer remainder. Computes `self % rhs`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero, or if the division overflows
            /// regardless of whether overflow checks are enabled.
            #[inline]
            pub const fn strict_rem(self, rhs: Self) -> Self {
                Self(self.0.strict_rem(rhs.0))
            }

            /// Strict Euclidean remainder. Computes `self.rem_euclid(rhs)`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero, or if the division overflows
            /// regardless of whether overflow checks are enabled.
            #[inline]
            pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
                Self(self.0.strict_rem_euclid(rhs.0))
            }

            /// Strict left shift. Computes `self << rhs`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is larger than or equal to the number of
            /// bits in `self`.
            #[inline]
            pub const fn strict_shl(self, rhs: u32) -> Self {
                Self(self.0.strict_shl(rhs))
            }

            /// Strict right shift. Computes `self >> rhs`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is larger than or equal to the number of
            /// bits in `self`.
            #[inline]
            pub const fn strict_shr(self, rhs: u32) -> Self {
                Self(self.0.strict_shr(rhs))
            }

            /// Strict exponentiation. Raises `self` to the power of `exp`, panicking if
            /// overflow occurred.
            ///
            /// # Panics
            ///
            /// This function will always panic on overflow, regardless of whether overflow
            /// checks are enabled.
            #[inline]
            pub const fn strict_pow(self, exp: u32) -> Self {
                Self(self.0.strict_pow(exp))
            }

            /// Unchecked integer addition. Computes `self + rhs`, assuming overflow cannot
            /// occur.
            ///
            /// # Safety
            ///
            /// This results in undefined behavior when the result would overflow, i.e. when
            /// [`checked_add`](Self::checked_add) would return `None`.
            #[inline]
            pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
                // SAFETY: Caller guarantees no overflow.
                Self(unsafe { self.0.unchecked_add(rhs.0) })
            }

            /// Unchecked integer subtraction. Computes `self - rhs`, assuming overflow cannot
            /// occur.
            ///
            /// # Safety
            ///
            /// This results in undefined behavior when the result would overflow, i.e. when
            /// [`checked_sub`](Self::checked_sub) would return `None`.
            #[inline]
            pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
                // SAFETY: Caller guarantees no overflow.
                Self(unsafe { self.0.unchecked_sub(rhs.0) })
            }

            /// Unchecked integer multiplication. Computes `self * rhs`, assuming overflow
            /// cannot occur.
            ///
            /// # Safety
            ///
            /// This results in undefined behavior when the result would overflow, i.e. when
            /// [`checked_mul`](Self::checked_mul) would return `None`.
            #[inline]
            pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
                // SAFETY: Caller guarantees no overflow.
                Self(unsafe { self.0.unchecked_mul(rhs.0) })
            }

            /// Unchecked left shift. Computes `self << rhs`, assuming `rhs` is less than the
            /// number of bits in `self`.
            ///
            /// # Safety
            ///
            /// This results in undefined behavior if `rhs` is larger than or equal to the
            /// number of bits in `self`, i.e. when [`checked_shl`](Self::checked_shl) would
            /// return `None`.
            #[inline]
            pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
                // SAFETY: Caller guarantees `rhs` is less than `Self::BITS`.
                Self(unsafe { self.0.unchecked_shl(rhs) })
            }

            /// Unchecked right shift. Computes `self >> rhs`, assuming `rhs` is less than the
            /// number of bits in `self`.
            ///
            /// # Safety
            ///
            /// This results in undefined behavior if `rhs` is larger than or equal to the
            /// number of bits in `self`, i.e. when [`checked_shr`](Self::checked_shr) would
            /// return `None`.
            #[inline]
            pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
                // SAFETY: Caller guarantees `rhs` is less than `Self::BITS`.
                Self(unsafe { self.0.unchecked_shr(rhs) })
            }

            /// Unbounded left shift. Computes `self << rhs`, treating `self` as if it had
            /// infinitely many high bits, so that shifts by `rhs` greater than or equal to
            /// the number of bits in `self` return zero.
            #[inline]
            pub const fn unbounded_shl(self, rhs: u32) -> Self {
                Self(self.0.unbounded_shl(rhs))
            }

            /// Unbounded right shift. Computes `self >> rhs`, treating `self` as if it had
            /// infinitely many high bits, so that shifts by `rhs` greater than or equal to
            /// the number of bits in `self` return zero (for `UNative` and non-negative
            /// `INative`) or `-1` (for negative `INative`).
            #[inline]
            pub const fn unbounded_shr(self, rhs: u32) -> Self {
                Self(self.0.unbounded_shr(rhs))
            }

            /// Converts a string slice in a given base to an integer.
            ///
            /// The string is expected to be an optional sign followed by digits. For signed
            /// types both `+` and `-` are accepted; for unsigned types only `+` is accepted.
            /// Leading and trailing whitespace represent an error. Digits are a subset of
            /// these characters, depending on `radix`: `0-9`, `a-z`, `A-Z`.
            ///
            /// # Panics
            ///
            /// This function panics if `radix` is not in the range from 2 to 36.
            #[inline]
            pub const fn from_str_radix(
                src: &str,
                radix: u32,
            ) -> Result<Self, ::core::num::ParseIntError> {
                match <$inner>::from_str_radix(src, radix) {
                    Ok(x) => Ok(Self(x)),
                    Err(e) => Err(e),
                }
            }
        }

        $crate::native::delegate_binop!($t, Add, add, +);
        $crate::native::delegate_binop!($t, Sub, sub, -);
        $crate::native::delegate_binop!($t, Mul, mul, *);
        $crate::native::delegate_binop!($t, Div, div, /);
        $crate::native::delegate_binop!($t, Rem, rem, %);
        $crate::native::delegate_binop!($t, BitAnd, bitand, &);
        $crate::native::delegate_binop!($t, BitOr, bitor, |);
        $crate::native::delegate_binop!($t, BitXor, bitxor, ^);

        $crate::native::delegate_assign_op!($t, AddAssign, add_assign, +=);
        $crate::native::delegate_assign_op!($t, SubAssign, sub_assign, -=);
        $crate::native::delegate_assign_op!($t, MulAssign, mul_assign, *=);
        $crate::native::delegate_assign_op!($t, DivAssign, div_assign, /=);
        $crate::native::delegate_assign_op!($t, RemAssign, rem_assign, %=);
        $crate::native::delegate_assign_op!($t, BitAndAssign, bitand_assign, &=);
        $crate::native::delegate_assign_op!($t, BitOrAssign, bitor_assign, |=);
        $crate::native::delegate_assign_op!($t, BitXorAssign, bitxor_assign, ^=);

        $crate::native::delegate_unary_op!($t, Not, not, !);

        $crate::native::delegate_shifts_native_prim!($t, u8);
        $crate::native::delegate_shifts_native_prim!($t, u16);
        $crate::native::delegate_shifts_native_prim!($t, u32);
        $crate::native::delegate_shifts_native_prim!($t, u64);
        $crate::native::delegate_shifts_native_prim!($t, u128);
        $crate::native::delegate_shifts_native_prim!($t, usize);
        $crate::native::delegate_shifts_native_prim!($t, i8);
        $crate::native::delegate_shifts_native_prim!($t, i16);
        $crate::native::delegate_shifts_native_prim!($t, i32);
        $crate::native::delegate_shifts_native_prim!($t, i64);
        $crate::native::delegate_shifts_native_prim!($t, i128);
        $crate::native::delegate_shifts_native_prim!($t, isize);

        $crate::native::delegate_shifts_prim_native!(u8, $t);
        $crate::native::delegate_shifts_prim_native!(u16, $t);
        $crate::native::delegate_shifts_prim_native!(u32, $t);
        $crate::native::delegate_shifts_prim_native!(u64, $t);
        $crate::native::delegate_shifts_prim_native!(u128, $t);
        $crate::native::delegate_shifts_prim_native!(usize, $t);
        $crate::native::delegate_shifts_prim_native!(i8, $t);
        $crate::native::delegate_shifts_prim_native!(i16, $t);
        $crate::native::delegate_shifts_prim_native!(i32, $t);
        $crate::native::delegate_shifts_prim_native!(i64, $t);
        $crate::native::delegate_shifts_prim_native!(i128, $t);
        $crate::native::delegate_shifts_prim_native!(isize, $t);

        $crate::native::delegate_shifts_native_native!($t, $crate::UNative);
        $crate::native::delegate_shifts_native_native!($t, $crate::INative);

        $crate::native::delegate_iter_op!($t, Sum, sum);
        $crate::native::delegate_iter_op!($t, Product, product);

        $crate::native::delegate_fmt!($t, Debug);
        $crate::native::delegate_fmt!($t, Display);
        $crate::native::delegate_fmt!($t, Binary);
        $crate::native::delegate_fmt!($t, Octal);
        $crate::native::delegate_fmt!($t, LowerHex);
        $crate::native::delegate_fmt!($t, UpperHex);

        impl ::core::str::FromStr for $t {
            type Err = ::core::num::ParseIntError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                ::core::str::FromStr::from_str(s).map(Self)
            }
        }
    };
}

pub(crate) use {
    define_native, delegate_assign_op, delegate_binop, delegate_fmt, delegate_from_native_prim,
    delegate_from_prim_native, delegate_iter_op, delegate_shift_assign_op_native_native,
    delegate_shift_assign_op_native_prim, delegate_shift_assign_op_prim_native,
    delegate_shift_op_native_native, delegate_shift_op_native_prim, delegate_shift_op_prim_native,
    delegate_shifts_native_native, delegate_shifts_native_prim, delegate_shifts_prim_native,
    delegate_try_from_native_native, delegate_try_from_native_prim, delegate_try_from_prim_native,
    delegate_unary_op,
};
