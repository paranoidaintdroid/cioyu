/*
This module implements several modern pseudorandom number generators.

Algorithms implemented here are NOT original work. They are reference
implementations based on publicly available specifications by their
respective authors.

Algorithms included:

SplitMix64
---------
Author: Sebastiano Vigna
Year: 2015
Used primarily as a fast generator and for seeding other RNGs.

Reference:
http://xoshiro.di.unimi.it/splitmix64.c


Xoshiro256++
-----------
Authors: David Blackman and Sebastiano Vigna
Year: 2018
A fast, high-quality general purpose PRNG.

Reference:
http://xoshiro.di.unimi.it/xoshiro256plusplus.c

Paper:
"Scrambled Linear Pseudorandom Number Generators"
Blackman & Vigna


PCG (Permuted Congruential Generator)
------------------------------------
Author: Melissa O'Neill
Year: 2014

Reference:
https://www.pcg-random.org/

Paper:
"PCG: A Family of Simple Fast Space-Efficient Statistically Good Algorithms for Random Number Generation"
Melissa E. O'Neill


This crate provides idiomatic Rust implementations and a unified
Rng trait interface for experimentation, learning, and simulation.
*/

mod pcg32;
mod seed;
mod splitmix64;
mod traits;
mod xoshiro256pp;

pub use pcg32::Pcg32;
pub use seed::expand_4;
pub use splitmix64::SplitMix64;
pub use traits::{Rng, SeedableRng};
pub use xoshiro256pp::Xoshiro256pp;

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 42;
    const N_SMALL: usize = 10_000;
    const N_LARGE: usize = 100_000;

    // -----------------------------
    // Determinism Tests
    // -----------------------------

    #[test]
    fn splitmix64_determinism() {
        let mut a = SplitMix64::new(SEED);
        let mut b = SplitMix64::new(SEED);

        for _ in 0..N_SMALL {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn xoshiro256pp_determinism() {
        let mut a = Xoshiro256pp::new(SEED);
        let mut b = Xoshiro256pp::new(SEED);

        for _ in 0..N_SMALL {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn pcg_determinism() {
        let mut a = Pcg32::new(SEED, 7);
        let mut b = Pcg32::new(SEED, 7);

        for _ in 0..N_SMALL {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    // -----------------------------
    // Range Test
    // -----------------------------

    #[test]
    fn f64_range_test() {
        let mut sm = SplitMix64::new(SEED);
        let mut xs = Xoshiro256pp::new(SEED);
        let mut pcg = Pcg32::new(SEED, 3);

        for _ in 0..N_SMALL {
            let a = sm.next_f64();
            let b = xs.next_f64();
            let c = pcg.next_f64();

            assert!(a >= 0.0 && a < 1.0);
            assert!(b >= 0.0 && b < 1.0);
            assert!(c >= 0.0 && c < 1.0);
        }
    }

    // -----------------------------
    // Iterator Tests
    // -----------------------------

    #[test]
    fn iterator_test() {
        let sm: Vec<u64> = SplitMix64::new(SEED).take(5).collect();
        let xs: Vec<u64> = Xoshiro256pp::new(SEED).take(5).collect();
        let pcg: Vec<u64> = Pcg32::new(SEED, 5).take(5).collect();

        assert_eq!(sm.len(), 5);
        assert_eq!(xs.len(), 5);
        assert_eq!(pcg.len(), 5);
    }

    // -----------------------------
    // Mean Test
    // Expected mean = 0.5
    // -----------------------------

    #[test]
    fn mean_test() {
        let mut rng = Xoshiro256pp::new(SEED);

        let mut sum = 0.0;

        for _ in 0..N_LARGE {
            sum += rng.next_f64();
        }

        let mean = sum / N_LARGE as f64;

        println!("mean = {}", mean);

        assert!((mean - 0.5).abs() < 0.005);
    }

    // -----------------------------
    // Variance Test
    // Uniform variance = 1/12
    // -----------------------------

    #[test]
    fn variance_test() {
        let mut rng = Xoshiro256pp::new(SEED);

        let mut values = Vec::with_capacity(N_LARGE);

        for _ in 0..N_LARGE {
            values.push(rng.next_f64());
        }

        let mean: f64 = values.iter().sum::<f64>() / N_LARGE as f64;

        let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / N_LARGE as f64;

        let expected = 1.0 / 12.0;

        println!("variance = {}", variance);

        assert!((variance - expected).abs() < 0.01);
    }

    // -----------------------------
    // Bit Balance Test
    // Expect ~50% ones
    // -----------------------------

    #[test]
    fn bit_balance_test() {
        let mut rng = SplitMix64::new(SEED);

        let mut ones = 0u64;
        let mut total = 0u64;

        for _ in 0..N_SMALL {
            let v = rng.next_u64();

            ones += v.count_ones() as u64;
            total += 64;
        }

        let ratio = ones as f64 / total as f64;

        println!("bit ratio = {}", ratio);

        assert!((ratio - 0.5).abs() < 0.02);
    }

    // -----------------------------
    // Histogram Test
    // -----------------------------

    #[test]
    fn histogram_test() {
        let mut rng = SplitMix64::new(SEED);

        let mut buckets = [0u32; 10];

        for _ in 0..N_LARGE {
            let v = rng.next_f64();
            let idx = (v * 10.0) as usize;
            buckets[idx] += 1;
        }

        let expected = N_LARGE / 10;

        for b in buckets {
            let diff = (b as i64 - expected as i64).abs();
            assert!(diff < (N_LARGE as i64 / 20));
        }
    }

    // -----------------------------
    // Different Seeds Produce Different Sequences
    // -----------------------------

    #[test]
    fn seed_independence_test() {
        let mut a = SplitMix64::new(1);
        let mut b = SplitMix64::new(2);

        let mut equal = 0;

        for _ in 0..1000 {
            if a.next_u64() == b.next_u64() {
                equal += 1;
            }
        }

        assert_eq!(equal, 0);
    }

    // -----------------------------
    // No Early Cycle Test
    // -----------------------------

    #[test]
    fn no_short_cycle_test() {
        let mut rng = SplitMix64::new(SEED);

        let first = rng.next_u64();

        for _ in 0..1_000_000 {
            if rng.next_u64() == first {
                panic!("cycle detected too early");
            }
        }
    }
}
