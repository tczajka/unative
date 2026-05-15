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
