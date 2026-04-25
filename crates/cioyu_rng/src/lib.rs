pub trait Rng {
    fn next_u64(&mut self) -> u64;
    fn next_f64(&mut self) -> f64 {
        let value = self.next_u64();
        value as f64 / u64::MAX as f64
    }
}

pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Rng for SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);

        let mut z = self.state;

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);

        z ^ (z >> 31)
    }
}

impl Iterator for SplitMix64 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}

pub struct Xoshiro256pp {
    state: [u64; 4],
}

impl Xoshiro256pp {
    pub fn new(seed: u64) -> Self {
        let mut rng = SplitMix64::new(seed);
        Self {
            state: [
                rng.next_u64(),
                rng.next_u64(),
                rng.next_u64(),
                rng.next_u64(),
            ],
        }
    }
}

impl Rng for Xoshiro256pp {
    fn next_u64(&mut self) -> u64 {
        let result = (self.state[0].wrapping_add(self.state[3]))
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }
}

impl Iterator for Xoshiro256pp {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}

pub struct Pcg {
    state: u64,
    increment: u64,
}

impl Pcg {
    pub fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            increment: (stream << 1) | 1,
        };
        rng.next_u64();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u64();
        rng
    }
}

impl Pcg {
    fn next_u32(&mut self) -> u32 {
        let old = self.state;

        self.state = old
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.increment);

        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;

        xorshifted.rotate_right(rot)
    }
}

impl Rng for Pcg {
    fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
    }
}

impl Iterator for Pcg {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 42;

    #[test]
    fn splitmix64_determinism() {
        let mut rng1 = SplitMix64::new(SEED);
        let mut rng2 = SplitMix64::new(SEED);

        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn xoshiro256pp_determinism() {
        let mut rng1 = Xoshiro256pp::new(SEED);
        let mut rng2 = Xoshiro256pp::new(SEED);

        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn pcg_determinism() {
        let mut rng1 = Pcg::new(SEED, 7);
        let mut rng2 = Pcg::new(SEED, 7);

        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn f64_range_test() {
        let mut sm = SplitMix64::new(SEED);
        let mut xs = Xoshiro256pp::new(SEED);
        let mut pcg = Pcg::new(SEED, 5);

        for _ in 0..10000 {
            let v1 = sm.next_f64();
            let v2 = xs.next_f64();
            let v3 = pcg.next_f64();

            assert!(v1 >= 0.0 && v1 < 1.0);
            assert!(v2 >= 0.0 && v2 < 1.0);
            assert!(v3 >= 0.0 && v3 < 1.0);
        }
    }

    #[test]
    fn iterator_test() {
        let sm_values: Vec<u64> = SplitMix64::new(SEED).take(5).collect();
        let xs_values: Vec<u64> = Xoshiro256pp::new(SEED).take(5).collect();
        let pcg_values: Vec<u64> = Pcg::new(SEED, 3).take(5).collect();

        assert_eq!(sm_values.len(), 5);
        assert_eq!(xs_values.len(), 5);
        assert_eq!(pcg_values.len(), 5);
    }

    #[test]
    fn mean_uniformity_test() {
        let mut sm = SplitMix64::new(SEED);
        let mut xs = Xoshiro256pp::new(SEED);
        let mut pcg = Pcg::new(SEED, 9);

        let mut sum_sm = 0.0;
        let mut sum_xs = 0.0;
        let mut sum_pcg = 0.0;

        let n = 100000;

        for _ in 0..n {
            sum_sm += sm.next_f64();
            sum_xs += xs.next_f64();
            sum_pcg += pcg.next_f64();
        }

        let mean_sm = sum_sm / n as f64;
        let mean_xs = sum_xs / n as f64;
        let mean_pcg = sum_pcg / n as f64;

        let tolerance = 0.005;

        println!("mean sm64  = {}", mean_sm);
        println!("mean xoshi = {}", mean_xs);
        println!("mean pcg   = {}", mean_pcg);

        assert!((mean_sm - 0.5).abs() < tolerance);
        assert!((mean_xs - 0.5).abs() < tolerance);
        assert!((mean_pcg - 0.5).abs() < tolerance);
    }

    #[test]
    fn histogram_uniformity_test() {
        let mut rng = SplitMix64::new(SEED);

        let mut buckets = [0u32; 10];
        let samples = 100000;

        for _ in 0..samples {
            let v = rng.next_f64();
            let idx = (v * 10.0) as usize;
            buckets[idx] += 1;
        }

        let expected = samples / 10;

        for b in buckets {
            let diff = (b as i32 - expected as i32).abs();
            assert!(diff < (samples as i32 / 20));
        }
    }
}
