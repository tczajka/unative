use crate::inner::UNativeInner;
use crate::native::define_native;

define_native! {
    /// Native unsigned integer type.
    pub struct UNative(UNativeInner);
}
