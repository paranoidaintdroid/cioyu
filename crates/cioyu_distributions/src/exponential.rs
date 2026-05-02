use cioyu_rng::Rng;
use crate::Distribution;

/// Exponential distribution
///
/// x = -ln(U) / λ
#[derive(Clone, Debug)]
pub struct Exponential {
    lambda: f64,
}

impl Exponential {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0, "Exponential: lambda must be positive");

        Self { lambda }
    }
}

impl Distribution<f64> for Exponential {
    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> f64 {
        let u = rng.next_f64().max(f64::EPSILON);
        -u.ln() / self.lambda
    }
}