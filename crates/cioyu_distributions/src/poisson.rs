use cioyu_rng::Rng;
use crate::Distribution;
#[allow(dead_code)]
/// Poisson distribution using Knuth's algorithm
///
/// Efficient for λ < 10
#[derive(Clone, Debug)]
pub struct Poisson {
    lambda: f64,
    l: f64,
}

impl Poisson {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0, "Poisson: lambda must be positive");

        Self {
            lambda,
            l: (-lambda).exp(),
        }
    }
}

impl Distribution<u64> for Poisson {
    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> u64 {
        let mut k = 0;
        let mut p = 1.0;

        loop {
            k += 1;
            p *= rng.next_f64();

            if p <= self.l {
                return k - 1;
            }
        }
    }
}