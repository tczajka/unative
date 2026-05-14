cfg_select! {
    target_pointer_width = "16" => {
        pub(crate) type UNativeInner = u16;
        pub(crate) type INativeInner = i16;
    }
    target_pointer_width = "32" => {
        pub(crate) type UNativeInner = u32;
        pub(crate) type INativeInner = i32;
    }
    _ => {
        pub(crate) type UNativeInner = u64;
        pub(crate) type INativeInner = i64;
    }
}
