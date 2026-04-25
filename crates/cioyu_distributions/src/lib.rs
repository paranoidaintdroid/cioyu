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
}
