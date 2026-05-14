use crate::inner::INativeInner;
use crate::native::{define_native, delegate_from, delegate_into, delegate_unary_op};

define_native! {
    /// Native signed integer type.
    pub struct INative(INativeInner);
}

delegate_unary_op!(INative, Neg, neg, -);

delegate_from!(INative, INativeInner, bool);
delegate_from!(INative, INativeInner, i8);
delegate_from!(INative, INativeInner, i16);
delegate_from!(INative, INativeInner, u8);

delegate_into!(INative, i128);
