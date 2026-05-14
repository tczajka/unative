use crate::inner::INativeInner;
use crate::native::{define_native, delegate_unary_op};

define_native! {
    /// Native signed integer type.
    pub struct INative(INativeInner);
}

delegate_unary_op!(INative, Neg, neg, -);
