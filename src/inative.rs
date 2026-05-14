use crate::UNative;
use crate::inner::INativeInner;
use crate::native::{
    define_native, delegate_from, delegate_into, delegate_try_from, delegate_try_from_native,
    delegate_try_into, delegate_unary_op,
};

define_native! {
    /// Native signed integer type.
    pub struct INative(INativeInner);
}

delegate_unary_op!(INative, Neg, neg, -);

delegate_from!(INative, bool);
delegate_from!(INative, i8);
delegate_from!(INative, i16);
delegate_from!(INative, u8);

delegate_into!(INative, i128);

delegate_try_from!(INative, u16);
delegate_try_from!(INative, u32);
delegate_try_from!(INative, i32);
delegate_try_from!(INative, u64);
delegate_try_from!(INative, i64);
delegate_try_from!(INative, u128);
delegate_try_from!(INative, i128);
delegate_try_from!(INative, usize);
delegate_try_from!(INative, isize);

delegate_try_into!(INative, u8);
delegate_try_into!(INative, i8);
delegate_try_into!(INative, u16);
delegate_try_into!(INative, i16);
delegate_try_into!(INative, u32);
delegate_try_into!(INative, i32);
delegate_try_into!(INative, u64);
delegate_try_into!(INative, i64);
delegate_try_into!(INative, u128);
delegate_try_into!(INative, usize);
delegate_try_into!(INative, isize);

delegate_try_from_native!(INative, UNative);
