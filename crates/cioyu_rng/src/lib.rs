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

pub struct Xoshiro256pp {
    state: [u64; 4],
}

impl Xoshiro256pp {
    pub fn new(seed :u64) -> Self{
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
        let result = (self.state[0]
            .wrapping_add(self.state[3]))
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        let t =self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
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

    #[test]
    fn xoshiro_256pp_test(){
        let seed : u64 = 2;
        let mut rng_1 = Xoshiro256pp::new(seed);
        let mut rng_2 = Xoshiro256pp::new(seed);

        for _ in 0..3{
            assert_eq!(rng_1.next_u64(), rng_2.next_u64());
        }
    }

    #[test]
    fn f64_range_test(){
        let seed : u64 = 2;
        let mut rng_sm64 = SplitMix64::new(seed);
        let mut rng_xsr256pp = Xoshiro256pp::new(seed);
        
        let rng_sm64 = rng_sm64.next_f64();
        let rng_xsr256pp =rng_xsr256pp.next_f64();
        
        assert!(rng_sm64 >= 0.0 && rng_sm64 < 1.0);
        assert!(rng_xsr256pp >= 0.0 && rng_xsr256pp < 1.0);
    }
}