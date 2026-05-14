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
        pub struct $name($inner);

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
    };
}

pub(crate) use {
    define_native, delegate_assign_op, delegate_binop, delegate_fmt, delegate_unary_op,
};
