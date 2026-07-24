//! Tests for the `proptest` integration.

#![cfg(feature = "proptest")]

use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::TestRunner;
use unative::proptest::{inative_in_range, unative_in_range};
use unative::{INative, UNative};

fn require_arbitrary<T: Arbitrary>() {}

#[test]
fn arbitrary_marker() {
    require_arbitrary::<UNative>();
    require_arbitrary::<INative>();
}

proptest! {
    #[test]
    fn any_unative_round_trips_through_u128(x in any::<UNative>()) {
        prop_assert_eq!(UNative::try_from(x.to_u128()).unwrap(), x);
    }

    #[test]
    fn any_inative_round_trips_through_i128(x in any::<INative>()) {
        prop_assert_eq!(INative::try_from(x.to_i128()).unwrap(), x);
    }

    #[test]
    fn unative_half_open_range_is_in_bounds(
        x in unative_in_range(UNative::from(10u8)..UNative::from(20u8)),
    ) {
        prop_assert!(x >= UNative::from(10u8));
        prop_assert!(x < UNative::from(20u8));
    }

    #[test]
    fn unative_inclusive_range_is_in_bounds(
        x in unative_in_range(UNative::from(10u8)..=UNative::from(20u8)),
    ) {
        prop_assert!(x >= UNative::from(10u8));
        prop_assert!(x <= UNative::from(20u8));
    }

    #[test]
    fn inative_half_open_range_is_in_bounds(
        x in inative_in_range(INative::from(-20i8)..INative::from(-10i8)),
    ) {
        prop_assert!(x >= INative::from(-20i8));
        prop_assert!(x < INative::from(-10i8));
    }

    #[test]
    fn inative_inclusive_range_is_in_bounds(
        x in inative_in_range(INative::from(-5i8)..=INative::from(5i8)),
    ) {
        prop_assert!(x >= INative::from(-5i8));
        prop_assert!(x <= INative::from(5i8));
    }

    #[test]
    fn unative_unbounded_range_stays_in_type(x in unative_in_range(..)) {
        // Exercises the unbounded-bounds path; the assertion is always true.
        prop_assert!(x <= UNative::MAX);
    }
}

// Shrinking should drive a value to the smallest representable in the requested
// range.
#[test]
fn unative_shrinks_to_low_bound() {
    let mut runner = TestRunner::deterministic();
    let strategy = unative_in_range(UNative::from(5u8)..UNative::from(200u8));
    let mut tree = strategy.new_tree(&mut runner).unwrap();
    while tree.simplify() {}
    assert_eq!(tree.current(), UNative::from(5u8));
}

#[test]
fn inative_shrinks_to_zero() {
    let mut runner = TestRunner::deterministic();
    let strategy = inative_in_range(INative::from(-100i8)..=INative::from(100i8));
    let mut tree = strategy.new_tree(&mut runner).unwrap();
    while tree.simplify() {}
    assert_eq!(tree.current(), INative::ZERO);
}
