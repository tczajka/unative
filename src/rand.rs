//! `rand` integration.
//!
//! Implements sampling of [`UNative`] and [`INative`] via `StandardUniform` and uniform ranges.

use rand::Rng;
use rand::distr::uniform::{Error, SampleBorrow, SampleUniform, UniformInt, UniformSampler};
use rand::distr::{Distribution, StandardUniform};

use crate::inner::{INativeInner, UNativeInner};
use crate::{INative, UNative};

impl Distribution<UNative> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UNative {
        UNative(StandardUniform::sample(self, rng))
    }
}

impl Distribution<INative> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> INative {
        INative(StandardUniform::sample(self, rng))
    }
}

// Always delegates to `UniformInt<u64>` rather than `UniformInt<UNativeInner>`
// so that, for a deterministic seeded RNG, sampling the same range produces
// the same value and consumes the same bytes regardless of `UNative`'s width.
// `UniformInt::sample` reads a number of bytes proportional to its type's
// width, so using `UNativeInner` directly would make a value sampled from
// `0..100` differ between a 16-bit and a 64-bit build.
/// Uniform-range sampler for [`UNative`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformUNative(UniformInt<u64>);

impl UniformSampler for UniformUNative {
    type X = UNative;

    #[allow(clippy::useless_conversion)] // `u64::from` is a no-op when the inner is already u64.
    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<UNative> + Sized,
        B2: SampleBorrow<UNative> + Sized,
    {
        UniformInt::<u64>::new(u64::from(low.borrow().0), u64::from(high.borrow().0))
            .map(UniformUNative)
    }

    #[allow(clippy::useless_conversion)] // `u64::from` is a no-op when the inner is already u64.
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<UNative> + Sized,
        B2: SampleBorrow<UNative> + Sized,
    {
        UniformInt::<u64>::new_inclusive(u64::from(low.borrow().0), u64::from(high.borrow().0))
            .map(UniformUNative)
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UNative {
        // The sampled u64 lies in the same range the caller specified via
        // `UNative` bounds, so it always fits in `UNativeInner`.
        UNative(self.0.sample(rng) as UNativeInner)
    }
}

impl SampleUniform for UNative {
    type Sampler = UniformUNative;
}

// Always delegates to `UniformInt<i64>` rather than `UniformInt<INativeInner>`
// so that, for a deterministic seeded RNG, sampling the same range produces
// the same value and consumes the same bytes regardless of `INative`'s width.
// `UniformInt::sample` reads a number of bytes proportional to its type's
// width, so using `INativeInner` directly would make a value sampled from
// `-50..50` differ between a 16-bit and a 64-bit build.
/// Uniform-range sampler for [`INative`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformINative(UniformInt<i64>);

impl UniformSampler for UniformINative {
    type X = INative;

    #[allow(clippy::useless_conversion)] // `i64::from` is a no-op when the inner is already i64.
    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<INative> + Sized,
        B2: SampleBorrow<INative> + Sized,
    {
        UniformInt::<i64>::new(i64::from(low.borrow().0), i64::from(high.borrow().0))
            .map(UniformINative)
    }

    #[allow(clippy::useless_conversion)] // `i64::from` is a no-op when the inner is already i64.
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<INative> + Sized,
        B2: SampleBorrow<INative> + Sized,
    {
        UniformInt::<i64>::new_inclusive(i64::from(low.borrow().0), i64::from(high.borrow().0))
            .map(UniformINative)
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> INative {
        INative(self.0.sample(rng) as INativeInner)
    }
}

impl SampleUniform for INative {
    type Sampler = UniformINative;
}
