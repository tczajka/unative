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

    /// Checked addition of a signed integer. Computes `self + rhs`, returning `None` if
    /// overflow occurred.
    #[inline]
    pub const fn checked_add_signed(self, rhs: INative) -> Option<Self> {
        match self.0.checked_add_signed(rhs.0) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Checked subtraction of a signed integer. Computes `self - rhs`, returning `None` if
    /// overflow occurred.
    #[inline]
    pub const fn checked_sub_signed(self, rhs: INative) -> Option<Self> {
        match self.0.checked_sub_signed(rhs.0) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    /// Computes the signed difference `self - rhs`, returning `None` if the result does not
    /// fit in [`INative`].
    #[inline]
    pub const fn checked_signed_diff(self, rhs: UNative) -> Option<INative> {
        match self.0.checked_signed_diff(rhs.0) {
            Some(x) => Some(INative(x)),
            None => None,
        }
    }

    /// Strict addition of a signed integer. Computes `self + rhs`, panicking if overflow
    /// occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether overflow checks are
    /// enabled.
    #[inline]
    pub const fn strict_add_signed(self, rhs: INative) -> Self {
        Self(self.0.strict_add_signed(rhs.0))
    }

    /// Strict subtraction of a signed integer. Computes `self - rhs`, panicking if overflow
    /// occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether overflow checks are
    /// enabled.
    #[inline]
    pub const fn strict_sub_signed(self, rhs: INative) -> Self {
        Self(self.0.strict_sub_signed(rhs.0))
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
