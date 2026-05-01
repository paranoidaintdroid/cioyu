use crate::seed;
use crate::traits::Rng;
use crate::SeedableRng;

/// Xoshiro256++
///
/// Fast, high-quality general purpose RNG designed by
/// David Blackman and Sebastiano Vigna.
///
/// Period:
/// 2^256 − 1
///
/// Recommended uses:
/// - simulations
/// - Monte Carlo
/// - randomized algorithms
///
/// Not cryptographically secure.
#[derive(Clone, Debug)]
pub struct Xoshiro256pp {
    state: [u64; 4],
}

impl Xoshiro256pp {
    /// Create a new RNG from a `u64` seed.
    ///
    /// The seed is expanded into the 256-bit internal state
    /// using SplitMix64 to ensure good bit diffusion.
    #[inline]
    pub fn new(seed: u64) -> Self {
        let state = seed::expand_4(seed);

        debug_assert!(state != [0, 0, 0, 0]);

        Self { state }
    }
}

impl Rng for Xoshiro256pp {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let s = &mut self.state;

        let result = (s[0].wrapping_add(s[3]))
            .rotate_left(23)
            .wrapping_add(s[0]);

        let t = s[1] << 17;

        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];

        s[2] ^= t;
        s[3] = s[3].rotate_left(45);

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

impl SeedableRng for Xoshiro256pp {
    type Seed = u64;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            state: seed::expand_4(seed),
        }
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        Self::from_seed(seed)
    }
}