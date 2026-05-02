use cioyu_rng::Rng;
use crate::Distribution;

/// Uniform distribution over [low, high)
#[derive(Clone, Debug)]
pub struct Uniform {
    low: f64,
    range: f64,
}

impl Uniform {
    pub fn new(low: f64, high: f64) -> Self {
        assert!(high > low, "Uniform: high must be greater than low");

        Self {
            low,
            range: high - low,
        }
    }
}

impl Distribution<f64> for Uniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> f64 {
        self.low + rng.next_f64() * self.range
    }
}