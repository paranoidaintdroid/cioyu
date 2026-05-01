use crate::SeedableRng;
use crate::traits::Rng;

/// PCG32
///
/// Permuted Congruential Generator producing 32-bit output.
/// Designed by Melissa O'Neill.
///
/// Period:
/// 2^64
///
/// Properties:
/// - Very small state
/// - Excellent statistical quality
/// - Extremely fast
///
/// Not cryptographically secure.
#[derive(Clone, Debug)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    /// Creates a new PCG32 generator.
    ///
    /// `seed` initializes the internal state.
    /// `stream` selects the random stream.
    ///
    /// Different stream values produce independent sequences.
    #[inline]
    pub fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (stream << 1) | 1,
        };

        rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32();

        rng
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        let oldstate = self.state;

        self.state = oldstate
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);

        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;

        xorshifted.rotate_right(rot)
    }
}

impl Rng for Pcg32 {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;

        (hi << 32) | lo
    }
}

impl Iterator for Pcg32 {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}

impl SeedableRng for Pcg32 {
    type Seed = u64;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed, 1)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        Self::from_seed(seed)
    }
}
