#![cfg(feature = "rand")]

use rand::Rng;
use rand::RngExt;
use rand::SeedableRng;
use rand::distr::uniform::SampleUniform;
use rand::distr::{Distribution, StandardUniform, Uniform};
use rand::rngs::ChaCha20Rng;
use unative::{INative, UNative};

const SEED: [u8; 32] = [0x42; 32];

fn require_sample_uniform<T: SampleUniform>() {}

fn require_standard_distributed<T>()
where
    StandardUniform: Distribution<T>,
{
}

#[test]
fn marker_traits() {
    require_sample_uniform::<UNative>();
    require_sample_uniform::<INative>();
    require_standard_distributed::<UNative>();
    require_standard_distributed::<INative>();
}

// Sampling a `UNative` from a range that fits in any native width must produce
// the same value, and consume the same bytes from the RNG, as the equivalent
// `u64` sampler. Since `ChaCha20Rng` is itself portable, this proves that the
// `UNative` sampler is portable across native widths.
#[test]
fn unative_range_matches_u64_sampler() {
    let mut rng_unative = ChaCha20Rng::from_seed(SEED);
    let mut rng_u64 = ChaCha20Rng::from_seed(SEED);

    let unative_dist = Uniform::new(UNative::from(0u8), UNative::from(100u8)).unwrap();
    let u64_dist = Uniform::new(0u64, 100u64).unwrap();

    for _ in 0..256 {
        let v_unative = unative_dist.sample(&mut rng_unative);
        let v_u64 = u64_dist.sample(&mut rng_u64);
        assert_eq!(u64::from(v_unative), v_u64);
    }

    // The RNGs must also be in identical states — same bytes consumed.
    assert_eq!(rng_unative.next_u64(), rng_u64.next_u64());
}

// Mirror of the above for the signed sampler.
#[test]
fn inative_range_matches_i64_sampler() {
    let mut rng_inative = ChaCha20Rng::from_seed(SEED);
    let mut rng_i64 = ChaCha20Rng::from_seed(SEED);

    let inative_dist = Uniform::new(INative::from(-50i8), INative::from(50i8)).unwrap();
    let i64_dist = Uniform::new(-50i64, 50i64).unwrap();

    for _ in 0..256 {
        let v_inative = inative_dist.sample(&mut rng_inative);
        let v_i64 = i64_dist.sample(&mut rng_i64);
        assert_eq!(i64::from(v_inative), v_i64);
    }

    assert_eq!(rng_inative.next_u64(), rng_i64.next_u64());
}

#[test]
fn unative_inclusive_range_matches_u64_sampler() {
    let mut rng_unative = ChaCha20Rng::from_seed(SEED);
    let mut rng_u64 = ChaCha20Rng::from_seed(SEED);

    let unative_dist = Uniform::new_inclusive(UNative::from(1u8), UNative::from(255u8)).unwrap();
    let u64_dist = Uniform::new_inclusive(1u64, 255u64).unwrap();

    for _ in 0..256 {
        let v_unative = unative_dist.sample(&mut rng_unative);
        let v_u64 = u64_dist.sample(&mut rng_u64);
        assert_eq!(u64::from(v_unative), v_u64);
    }
}

#[test]
fn inative_inclusive_range_matches_i64_sampler() {
    let mut rng_inative = ChaCha20Rng::from_seed(SEED);
    let mut rng_i64 = ChaCha20Rng::from_seed(SEED);

    let inative_dist = Uniform::new_inclusive(INative::from(-127i8), INative::from(127i8)).unwrap();
    let i64_dist = Uniform::new_inclusive(-127i64, 127i64).unwrap();

    for _ in 0..256 {
        let v_inative = inative_dist.sample(&mut rng_inative);
        let v_i64 = i64_dist.sample(&mut rng_i64);
        assert_eq!(i64::from(v_inative), v_i64);
    }
}

#[test]
fn random_range_sugar() {
    let mut rng = ChaCha20Rng::from_seed(SEED);
    let lo = UNative::from(0u8);
    let hi = UNative::from(10u8);
    for _ in 0..64 {
        let v: UNative = rng.random_range(lo..hi);
        assert!(v >= lo && v < hi);
    }
}

#[test]
fn singleton_inclusive() {
    let mut rng = ChaCha20Rng::from_seed(SEED);
    let only = UNative::from(7u8);
    let dist = Uniform::new_inclusive(only, only).unwrap();
    for _ in 0..16 {
        assert_eq!(dist.sample(&mut rng), only);
    }
}

#[test]
fn standard_uniform_runs() {
    // Not portable across widths, but must not panic on any platform.
    let mut rng = ChaCha20Rng::from_seed(SEED);
    let _: UNative = rng.random();
    let _: INative = rng.random();
}

#[test]
fn inative_negative_range_in_bounds() {
    let mut rng = ChaCha20Rng::from_seed(SEED);
    let lo = INative::from(-100i8);
    let hi = INative::from(-1i8);
    let dist = Uniform::new(lo, hi).unwrap();
    for _ in 0..256 {
        let v = dist.sample(&mut rng);
        assert!(v >= lo && v < hi);
    }
}

#[test]
fn unative_full_native_range() {
    // Sampling from the full UNative range delegates to the u64 sampler with
    // the widened bounds and must remain in-range on every platform.
    let mut rng = ChaCha20Rng::from_seed(SEED);
    let dist = Uniform::new_inclusive(UNative::MIN, UNative::MAX).unwrap();
    for _ in 0..64 {
        let _ = dist.sample(&mut rng);
    }
}

#[test]
fn inative_full_native_range() {
    let mut rng = ChaCha20Rng::from_seed(SEED);
    let dist = Uniform::new_inclusive(INative::MIN, INative::MAX).unwrap();
    for _ in 0..64 {
        let _ = dist.sample(&mut rng);
    }
}
