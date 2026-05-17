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
    /// Returns the bit pattern of `self` reinterpreted as a signed integer of the same size.
    #[inline]
    pub const fn cast_signed(self) -> INative {
        INative(self.0.cast_signed())
    }
}

delegate_from_native_prim!(UNative, bool);
delegate_from_native_prim!(UNative, u8);
delegate_from_native_prim!(UNative, u16);

delegate_from_prim_native!(u128, UNative);

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

delegate_try_from_prim_native!(u8, UNative);
delegate_try_from_prim_native!(u16, UNative);
delegate_try_from_prim_native!(u32, UNative);
delegate_try_from_prim_native!(u64, UNative);
delegate_try_from_prim_native!(usize, UNative);
delegate_try_from_prim_native!(i8, UNative);
delegate_try_from_prim_native!(i16, UNative);
delegate_try_from_prim_native!(i32, UNative);
delegate_try_from_prim_native!(i64, UNative);
delegate_try_from_prim_native!(i128, UNative);
delegate_try_from_prim_native!(isize, UNative);

delegate_try_from_native_native!(UNative, INative);
