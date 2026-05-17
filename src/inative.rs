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
    /// Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.
    #[inline]
    pub const fn cast_unsigned(self) -> UNative {
        UNative(self.0.cast_unsigned())
    }
}

delegate_unary_op!(INative, Neg, neg, -);

delegate_from_native_prim!(INative, bool);
delegate_from_native_prim!(INative, u8);
delegate_from_native_prim!(INative, i8);
delegate_from_native_prim!(INative, i16);

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

delegate_try_from_prim_native!(u8, INative);
delegate_try_from_prim_native!(u16, INative);
delegate_try_from_prim_native!(u32, INative);
delegate_try_from_prim_native!(u64, INative);
delegate_try_from_prim_native!(u128, INative);
delegate_try_from_prim_native!(usize, INative);
delegate_try_from_prim_native!(i8, INative);
delegate_try_from_prim_native!(i16, INative);
delegate_try_from_prim_native!(i32, INative);
delegate_try_from_prim_native!(i64, INative);
delegate_try_from_prim_native!(isize, INative);

delegate_try_from_native_native!(INative, UNative);
