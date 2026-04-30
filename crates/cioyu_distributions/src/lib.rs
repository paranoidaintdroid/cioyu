use cioyu_rng::Rng;
/// Uniform distribution over the interval [low, high).
///
/// Sampling formula:
/// x = low + U(0,1) * (high - low)


pub trait Distribution {
    type Output;
    fn sample(&mut self, rng: &mut impl Rng) -> Self::Output;
}


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

impl Distribution for Uniform {
    type Output = f64;
    #[inline]
    fn sample(&mut self, rng: &mut impl Rng) -> Self::Output {
        self.low + rng.next_f64() * self.range
    }
}

/// Normal (Gaussian) distribution using the Box–Muller transform.
///
/// This implementation caches the second generated sample,
/// making it roughly twice as fast as the naive implementation.
#[derive(Clone, Debug)]
pub struct Normal {
    mean: f64,
    std_dev: f64,
    cached: Option<f64>,
}

impl Normal {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        assert!(
            std_dev > 0.0,
            "Normal : Standard Deviation should be greater than 0"
        );

        Self {
            mean: mean,
            std_dev: std_dev,
            cached: None,
        }
    }
}



impl Distribution for Normal {
    type Output = f64;
    fn sample(&mut self, rng: &mut impl Rng) -> Self::Output {
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

/// Exponential distribution using the inverse CDF method.
///
/// x = -ln(U) / λ
#[derive(Clone, Debug)]
pub struct Exponential {
    lambda: f64,
}

impl Exponential {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0, "Exponential: lambda must be positive");

        Self { lambda: lambda }
    }

}

impl Distribution for Exponential {
    type Output = f64;
    fn sample(&mut self, rng: &mut impl Rng) -> Self::Output {
        let u = rng.next_f64().max(f64::EPSILON);
        -u.ln() / self.lambda
    }

}

/// Poisson distribution using Knuth's algorithm.
///
/// Efficient for λ < ~10.
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Poisson {
    lambda: f64,
    l: f64,
}

impl Poisson {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0, "Poisson: lambda must be positive");

        Self {
            lambda : lambda,
            l: (-lambda).exp(),
        }
    }

    
}

impl Distribution for Poisson {
    type Output = u64;
    fn sample(&mut self, rng: &mut impl Rng) -> Self::Output {
        let mut k: u64 = 0;
        let mut p: f64 = 1.0;

        loop {
            k += 1;
            p *= rng.next_f64();

            if p <= self.l {
                return k - 1;
            }
        }
    }
}


/// Statistical validation tests for probability distributions.
///
/// These tests verify that generated samples approximate the theoretical
/// properties of each distribution (mean, variance, sigma coverage, etc).
///
/// Because sampling is stochastic, tolerances are used instead of exact equality.
///
/// Sample sizes are chosen to balance:
/// - statistical reliability
/// - reasonable test runtime
///
/// These tests are *not* proofs of correctness, but sanity checks that the
/// implementations behave as expected.
#[cfg(test)]
mod tests {
    use super::*;
    use cioyu_rng::SplitMix64;

    #[test]
    fn test_uniform_range() {
        let low = 0.0;
        let high = 1000.0;
        let mut uni = Uniform::new(low, high);
        let mut rng = SplitMix64::new(4);

        for _ in 0..1000 {
            let uni_smpl = uni.sample(&mut rng);
            assert!(uni_smpl >= low && uni_smpl < high);
        }
    }

    #[test]
    fn test_uniform_determinism() {
        let low = 0.0;
        let high = 1000.0;

        let mut uni1 = Uniform::new(low, high);
        let mut uni2 = Uniform::new(low, high);

        let mut rng1 = SplitMix64::new(4);
        let mut rng2 = SplitMix64::new(4);

        let s1 = uni1.sample(&mut rng1);
        let s2 = uni2.sample(&mut rng2);

        assert_eq!(s1, s2);
    }

    #[test]
    fn normal_sigma_tests() {
        let mut rng = SplitMix64::new(12345);
        let mean = 0.0;
        let std_dev = 1.0;
        let mut norm = Normal::new(mean, std_dev);

        let samples = 1_000_000;

        let mut count_1 = 0;
        let mut count_2 = 0;
        let mut count_3 = 0;

        for _ in 0..samples {
            let x = norm.sample(&mut rng);

            if x > -1.0 && x < 1.0 {
                count_1 += 1;
            }

            if x > -2.0 && x < 2.0 {
                count_2 += 1;
            }

            if x > -3.0 && x < 3.0 {
                count_3 += 1;
            }
        }

        let frac_1 = count_1 as f64 / samples as f64;
        let frac_2 = count_2 as f64 / samples as f64;
        let frac_3 = count_3 as f64 / samples as f64;

        println!("within 1σ = {}", frac_1);
        println!("within 2σ = {}", frac_2);
        println!("within 3σ = {}", frac_3);

        assert!(
            frac_1 > 0.65 && frac_1 < 0.71,
            "Expected ~68% within 1σ, got {}",
            frac_1
        );

        assert!(
            frac_2 > 0.93 && frac_2 < 0.97,
            "Expected ~95% within 2σ, got {}",
            frac_2
        );

        assert!(
            frac_3 > 0.995 && frac_3 < 0.999,
            "Expected ~99.7% within 3σ, got {}",
            frac_3
        );
    }

    #[test]
    fn exponential_mean_test() {
        let mut rng = SplitMix64::new(12345);

        let lambda = 2.0;
        let expected_mean = 1.0 / lambda;

        let mut exp = Exponential::new(lambda);

        let samples = 10_000;
        let mut sum = 0.0;

        for _ in 0..samples {
            let x = exp.sample(&mut rng);
            sum += x;
        }

        let mean = sum / samples as f64;

        println!("exp mean = {}", mean);

        let tolerance = expected_mean * 0.05; // 5%

        assert!(
            (mean - expected_mean).abs() < tolerance,
            "Expected mean ≈ {}, got {}",
            expected_mean,
            mean
        );
    }

    #[test]
    fn poisson_mean_variance_test() {
        let mut rng = SplitMix64::new(12345);

        let lambda = 4.0;
        let mut pois = Poisson::new(lambda);

        let samples = 10_000;

        let mut sum = 0.0;
        let mut sum_sq = 0.0;

        for _ in 0..samples {
            let x = pois.sample(&mut rng) as f64;

            sum += x;
            sum_sq += x * x;
        }

        let mean = sum / samples as f64;
        let variance = (sum_sq / samples as f64) - mean * mean;

        println!("poisson mean = {}", mean);
        println!("poisson variance = {}", variance);

        let mean_tol = lambda * 0.05;

        assert!(
            (mean - lambda).abs() < mean_tol,
            "Expected mean ≈ {}, got {}",
            lambda,
            mean
        );

        let var_tol = lambda * 0.10;

        assert!(
            (variance - lambda).abs() < var_tol,
            "Expected variance ≈ {}, got {}",
            lambda,
            variance
        );
    }
}
