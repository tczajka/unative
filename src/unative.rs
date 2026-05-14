use crate::inner::UNativeInner;
use crate::native::{define_native, delegate_from, delegate_into};

define_native! {
    /// Native unsigned integer type.
    pub struct UNative(UNativeInner);
}

delegate_from!(UNative, UNativeInner, bool);
delegate_from!(UNative, UNativeInner, u8);
delegate_from!(UNative, UNativeInner, u16);

delegate_into!(UNative, u128);
