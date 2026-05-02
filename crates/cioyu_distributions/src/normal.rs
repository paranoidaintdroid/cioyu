use cioyu_rng::Rng;
use crate::Distribution;

/// Normal (Gaussian) distribution
/// using the Box–Muller transform.
#[derive(Clone, Debug)]
pub struct Normal {
    mean: f64,
    std_dev: f64,
    cached: Option<f64>,
}

impl Normal {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        assert!(std_dev > 0.0, "Normal: std_dev must be positive");

        Self {
            mean,
            std_dev,
            cached: None,
        }
    }
}

impl Distribution<f64> for Normal {
    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> f64 {
        if let Some(z) = self.cached.take() {
            return self.mean + self.std_dev * z;
        }

        let u1 = rng.next_f64().max(f64::EPSILON);
        let u2 = rng.next_f64();

        let r = (-2.0 * u1.ln()).sqrt();
        let theta = 2.0 * std::f64::consts::PI * u2;

        let z0 = r * theta.cos();
        let z1 = r * theta.sin();

        self.cached = Some(z1);

        self.mean + self.std_dev * z0
    }
}