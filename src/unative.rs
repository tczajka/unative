use crate::INative;
use crate::inner::UNativeInner;
use crate::native::{
    define_native, delegate_from, delegate_into, delegate_try_from, delegate_try_from_native,
    delegate_try_into,
};

define_native! {
    /// Native unsigned integer type.
    pub struct UNative(UNativeInner);
}

delegate_from!(UNative, bool);
delegate_from!(UNative, u8);
delegate_from!(UNative, u16);

delegate_into!(UNative, u128);

delegate_try_from!(UNative, i8);
delegate_try_from!(UNative, i16);
delegate_try_from!(UNative, u32);
delegate_try_from!(UNative, i32);
delegate_try_from!(UNative, u64);
delegate_try_from!(UNative, i64);
delegate_try_from!(UNative, u128);
delegate_try_from!(UNative, i128);
delegate_try_from!(UNative, usize);
delegate_try_from!(UNative, isize);

delegate_try_into!(UNative, u8);
delegate_try_into!(UNative, i8);
delegate_try_into!(UNative, u16);
delegate_try_into!(UNative, i16);
delegate_try_into!(UNative, u32);
delegate_try_into!(UNative, i32);
delegate_try_into!(UNative, u64);
delegate_try_into!(UNative, i64);
delegate_try_into!(UNative, i128);
delegate_try_into!(UNative, usize);
delegate_try_into!(UNative, isize);

delegate_try_from_native!(UNative, INative);
