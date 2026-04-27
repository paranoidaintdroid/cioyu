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


pub trait Rng {
    fn next_u64(&mut self) -> u64;

    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    ///Generate uniform values in [0,1)
    #[inline]
    fn next_f64(&mut self) -> f64 {
        const DOUBLE_UNIT: f64 = 1.0 / (1u64 << 53) as f64;
        let v = self.next_u64() >> 11;
        (v as f64) * DOUBLE_UNIT
    }
}

/// SplitMix64 - Fast Generator, primarily for seeding.
#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

const SPLITMIX_GAMMA: u64 = 0x9e3779b97f4a7c15;
const SPLITMIX_MUL1: u64 = 0xbf58476d1ce4e5b9;
const SPLITMIX_MUL2: u64 = 0x94d049bb133111eb;

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Rng for SplitMix64 {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(SPLITMIX_GAMMA);

        let mut z = self.state;

        z = (z ^ (z >> 30)).wrapping_mul(SPLITMIX_MUL1);
        z = (z ^ (z >> 27)).wrapping_mul(SPLITMIX_MUL2);

        z ^ (z >> 31)
    }
}

impl Iterator for SplitMix64 {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}

///Xoshiro256++ - Fast, high-quality general purpose RNG.
#[derive(Clone, Debug)]
pub struct Xoshiro256pp {
    state: [u64; 4],
}

impl Xoshiro256pp {
    pub fn new(seed: u64) -> Self {
        let mut rng = SplitMix64::new(seed);

        let state = [
            rng.next_u64(),
            rng.next_u64(),
            rng.next_u64(),
            rng.next_u64(),
        ];

        debug_assert!(state != [0, 0, 0, 0]);
        Self { state }
    }
}

impl Rng for Xoshiro256pp {

    #[inline]
    fn next_u64(&mut self) -> u64 {

        let result = (self.state[0].wrapping_add(self.state[3]))
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }
}

impl Iterator for Xoshiro256pp {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}

/// PCG32 - Produces 32-bit outputs, which are combined to form 64 bits.
#[derive(Clone, Debug)]
pub struct Pcg {
    state: u64,
    increment: u64,
}

const PCG_MULTIPLIER: u64 = 6364136223846793005;

impl Pcg {
    pub fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            increment: (stream << 1) | 1,
        };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32();

        rng
    }

    #[inline]
    fn next_u32_internal(&mut self) -> u32 {
        let old = self.state;

        self.state = old
            .wrapping_mul(PCG_MULTIPLIER)
            .wrapping_add(self.increment);

        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;

        xorshifted.rotate_right(rot)
    }

}


impl Rng for Pcg {
    
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u32_internal()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        ((self.next_u32_internal() as u64) << 32) | (self.next_u32_internal() as u64)
    }
}

impl Iterator for Pcg {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}

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
        let mut a = Pcg::new(SEED, 7);
        let mut b = Pcg::new(SEED, 7);

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
        let mut pcg = Pcg::new(SEED, 3);

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
        let pcg: Vec<u64> = Pcg::new(SEED, 5).take(5).collect();

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

        let variance: f64 = values
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / N_LARGE as f64;

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