pub(crate) use bits::{INativeInner, UNativeInner};

#[allow(dead_code)]
mod bits16 {
    pub(crate) type UNativeInner = u16;
    pub(crate) type INativeInner = i16;
}

#[allow(dead_code)]
mod bits32 {
    pub(crate) type UNativeInner = u32;
    pub(crate) type INativeInner = i32;
}

#[allow(dead_code)]
mod bits64 {
    pub(crate) type UNativeInner = u64;
    pub(crate) type INativeInner = i64;
}

// `UNative`/`INative` are guaranteed to be 16, 32, or 64 bits wide. The catch-all
// branch deliberately picks 64 bits even on hypothetical targets with wider native
// arithmetic, in order to uphold this invariant.
cfg_select! {
    // x86_64 and aarch64 always have 64-bit hardware arithmetic, including
    // on the 32-bit-pointer ABIs (x32, ILP32).
    any(target_arch = "x86_64", target_arch = "aarch64") => {
        use bits64 as bits;
    }
    target_pointer_width = "16" => {
        use bits16 as bits;
    }
    target_pointer_width = "32" => {
        use bits32 as bits;
    }
    _ => {
        use bits64 as bits;
    }
}
