pub trait Rng {
    fn next_u64(&mut self) -> u64;
    fn next_f64(&mut self) -> f64 {
        let value = self.next_u64();
        value as f64 / u64::MAX as f64
    }
}

pub struct SplitMix64{
    state:u64,
}

impl SplitMix64 {
    pub fn new(seed:u64) -> Self{
        Self { state: seed }
    }
}

impl Rng for SplitMix64{
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);

        let mut z = self.state;

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);

        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn split_mix64_test(){
        let seed : u64 = 2;
        let mut rng_1 = SplitMix64::new(seed);
        let mut rng_2 = SplitMix64::new(seed);

        for _ in 0..3{
            assert_eq!(rng_1.next_u64(), rng_2.next_u64());
        }
    }
}