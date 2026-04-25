use cioyu_rng::Rng;

pub struct Uniform {
    low: f64,
    high: f64,
}

impl Uniform {
    pub fn new(low: f64, high: f64) -> Self {
        Self {
            low: low,
            high: high,
        }
    }

    pub fn sample(&mut self, rng: &mut impl Rng) -> f64 {
        self.low + rng.next_f64() * (self.high - self.low)
    }
}

pub struct Normal {
    mean: f64,
    std_dev: f64,
}

impl Normal {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Self {
            mean: mean,
            std_dev: std_dev,
        }
    }
    pub fn sample(&mut self, rng: &mut impl Rng) -> f64 {
        let u1 = rng.next_f64().max(f64::EPSILON);
        let u2 = rng.next_f64();

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();

        self.mean + self.std_dev * z
    }
}

pub struct Exponential {
    lambda: f64,
}

impl Exponential {
    pub fn new(lambda: f64) -> Self {
        Self { lambda: lambda }
    }

    pub fn sample(&mut self, rng: &mut impl Rng) -> f64 {
        let u = rng.next_f64().max(f64::EPSILON);
        -u.ln() / self.lambda
    }
}

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
}
