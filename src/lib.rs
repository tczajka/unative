/// Native unsigned integer type.
pub struct UNative(UNativeInner);

/// Native signed integer type.
pub struct INative(INativeInner);

cfg_select! {
    target_pointer_width = "16" => {
        type UNativeInner = u16;
        type INativeInner = i16;
    }
    target_pointer_width = "32" => {
        type UNativeInner = u32;
        type INativeInner = i32;
    }
    _ => {
        type UNativeInner = u64;
        type INativeInner = i64;
    }
}
