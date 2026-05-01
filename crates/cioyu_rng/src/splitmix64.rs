use crate::SeedableRng;
use crate::traits::Rng;

/// SplitMix64
///
/// Extremely fast 64-bit generator designed by Sebastiano Vigna.
/// Mostly used as a **seeding generator** for larger RNGs.
///
/// Period: 2^64
///
/// Properties:
/// - Very fast
/// - Good statistical quality for seeding
/// - Not suitable for cryptography
#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Rng for SplitMix64 {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let mut z = self.state.wrapping_add(0x9E3779B97F4A7C15);
        self.state = z;

        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);

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

impl SeedableRng for SplitMix64 {
    type Seed = u64;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        Self::new(seed)
    }
}
