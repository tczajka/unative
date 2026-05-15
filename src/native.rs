macro_rules! delegate_binop {
    ($name:ident, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$name> for $name {
            type Output = $name;

            #[inline]
            fn $method(self, rhs: $name) -> $name {
                $name(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<&$name> for $name {
            type Output = $name;

            #[inline]
            fn $method(self, rhs: &$name) -> $name {
                $name(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<$name> for &$name {
            type Output = $name;

            #[inline]
            fn $method(self, rhs: $name) -> $name {
                $name(self.0 $op rhs.0)
            }
        }

        impl ::core::ops::$trait<&$name> for &$name {
            type Output = $name;

            #[inline]
            fn $method(self, rhs: &$name) -> $name {
                $name(self.0 $op rhs.0)
            }
        }
    };
}

macro_rules! delegate_unary_op {
    ($name:ident, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait for $name {
            type Output = $name;

            #[inline]
            fn $method(self) -> $name {
                $name($op self.0)
            }
        }

        impl ::core::ops::$trait for &$name {
            type Output = $name;

            #[inline]
            fn $method(self) -> $name {
                $name($op self.0)
            }
        }
    };
}

macro_rules! delegate_from {
    ($name:ident, $from:ty) => {
        impl From<$from> for $name {
            #[inline]
            fn from(value: $from) -> Self {
                Self(From::from(value))
            }
        }
    };
}

macro_rules! delegate_into {
    ($name:ident, $into:ty) => {
        impl From<$name> for $into {
            #[inline]
            fn from(value: $name) -> Self {
                From::from(value.0)
            }
        }
    };
}

macro_rules! delegate_try_from {
    ($name:ident, $from:ty) => {
        impl TryFrom<$from> for $name {
            type Error = ::core::num::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> Result<Self, Self::Error> {
                TryFrom::try_from(value)
                    .map(Self)
                    .map_err(Into::into)
            }
        }
    };
}

macro_rules! delegate_try_from_native {
    ($name:ident, $from:ident) => {
        impl TryFrom<$from> for $name {
            type Error = ::core::num::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> Result<Self, Self::Error> {
                TryFrom::try_from(value.0).map(Self)
            }
        }
    };
}

macro_rules! delegate_try_into {
    ($name:ident, $into:ty) => {
        impl TryFrom<$name> for $into {
            type Error = ::core::num::TryFromIntError;

            #[inline]
            fn try_from(value: $name) -> Result<Self, Self::Error> {
                TryFrom::try_from(value.0).map_err(Into::into)
            }
        }
    };
}

macro_rules! delegate_fmt {
    ($name:ident, $trait:ident) => {
        impl ::core::fmt::$trait for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.0, f)
            }
        }
    };
}

macro_rules! delegate_assign_op {
    ($name:ident, $trait:ident, $method:ident, $op:tt) => {
        impl ::core::ops::$trait<$name> for $name {
            #[inline]
            fn $method(&mut self, rhs: $name) {
                self.0 $op rhs.0;
            }
        }

        impl ::core::ops::$trait<&$name> for $name {
            #[inline]
            fn $method(&mut self, rhs: &$name) {
                self.0 $op rhs.0;
            }
        }
    };
}

macro_rules! define_native {
    ($(#[$attr:meta])* pub struct $name:ident($inner:ident);) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(pub(crate) $inner);

        impl $name {
            pub const MIN: $name = $name(<$inner>::MIN);
            pub const MAX: $name = $name(<$inner>::MAX);
            pub const BITS: u32 = <$inner>::BITS;
        }

        $crate::native::delegate_binop!($name, Add, add, +);
        $crate::native::delegate_binop!($name, Sub, sub, -);
        $crate::native::delegate_binop!($name, Mul, mul, *);
        $crate::native::delegate_binop!($name, Div, div, /);
        $crate::native::delegate_binop!($name, Rem, rem, %);
        $crate::native::delegate_binop!($name, BitAnd, bitand, &);
        $crate::native::delegate_binop!($name, BitOr, bitor, |);
        $crate::native::delegate_binop!($name, BitXor, bitxor, ^);

        $crate::native::delegate_assign_op!($name, AddAssign, add_assign, +=);
        $crate::native::delegate_assign_op!($name, SubAssign, sub_assign, -=);
        $crate::native::delegate_assign_op!($name, MulAssign, mul_assign, *=);
        $crate::native::delegate_assign_op!($name, DivAssign, div_assign, /=);
        $crate::native::delegate_assign_op!($name, RemAssign, rem_assign, %=);
        $crate::native::delegate_assign_op!($name, BitAndAssign, bitand_assign, &=);
        $crate::native::delegate_assign_op!($name, BitOrAssign, bitor_assign, |=);
        $crate::native::delegate_assign_op!($name, BitXorAssign, bitxor_assign, ^=);

        $crate::native::delegate_unary_op!($name, Not, not, !);

        $crate::native::delegate_fmt!($name, Debug);
        $crate::native::delegate_fmt!($name, Display);
        $crate::native::delegate_fmt!($name, Binary);
        $crate::native::delegate_fmt!($name, Octal);
        $crate::native::delegate_fmt!($name, LowerHex);
        $crate::native::delegate_fmt!($name, UpperHex);

        impl ::core::str::FromStr for $name {
            type Err = ::core::num::ParseIntError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                ::core::str::FromStr::from_str(s).map(Self)
            }
        }
    };
}

pub(crate) use {
    define_native, delegate_assign_op, delegate_binop, delegate_fmt, delegate_from, delegate_into,
    delegate_try_from, delegate_try_from_native, delegate_try_into, delegate_unary_op,
};
