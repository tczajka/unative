//! `proptest` integration.
//!
//! Implements `Arbitrary` for [`UNative`] and [`INative`], and provides [`unative_in_range`] /
//! [`inative_in_range`] for generating values confined to a range.

use core::ops::{Bound, RangeBounds};

use ::proptest::arbitrary::{Arbitrary, any};
use ::proptest::strategy::{BoxedStrategy, Strategy};

use crate::inner::{INativeInner, UNativeInner};
use crate::{INative, UNative};

impl Arbitrary for UNative {
    type Parameters = ();
    type Strategy = BoxedStrategy<UNative>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        any::<UNativeInner>().prop_map(UNative).boxed()
    }
}

impl Arbitrary for INative {
    type Parameters = ();
    type Strategy = BoxedStrategy<INative>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        any::<INativeInner>().prop_map(INative).boxed()
    }
}

/// Returns a `proptest` strategy that generates [`UNative`] values within `bounds`.
///
/// # Panics
///
/// Panics during generation if the range is empty.
pub fn unative_in_range(bounds: impl RangeBounds<UNative>) -> BoxedStrategy<UNative> {
    let lo: UNativeInner = match bounds.start_bound() {
        Bound::Included(v) => v.0,
        Bound::Excluded(v) => v.0.strict_add(1),
        Bound::Unbounded => UNativeInner::MIN,
    };
    match bounds.end_bound() {
        Bound::Included(v) => (lo..=v.0).prop_map(UNative).boxed(),
        Bound::Excluded(v) => (lo..v.0).prop_map(UNative).boxed(),
        Bound::Unbounded => (lo..=UNativeInner::MAX).prop_map(UNative).boxed(),
    }
}

/// Returns a `proptest` strategy that generates [`INative`] values within `bounds`.
///
/// # Panics
///
/// Panics during generation if the range is empty.
pub fn inative_in_range(bounds: impl RangeBounds<INative>) -> BoxedStrategy<INative> {
    let lo: INativeInner = match bounds.start_bound() {
        Bound::Included(v) => v.0,
        Bound::Excluded(v) => v.0.strict_add(1),
        Bound::Unbounded => INativeInner::MIN,
    };
    match bounds.end_bound() {
        Bound::Included(v) => (lo..=v.0).prop_map(INative).boxed(),
        Bound::Excluded(v) => (lo..v.0).prop_map(INative).boxed(),
        Bound::Unbounded => (lo..=INativeInner::MAX).prop_map(INative).boxed(),
    }
}
