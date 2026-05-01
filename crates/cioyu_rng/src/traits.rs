pub trait Rng {
    fn next_u64(&mut self) -> u64;

    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }

    /// Generate uniform values in [0,1)
    #[inline]
    fn next_f64(&mut self) -> f64 {
        const DOUBLE_UNIT: f64 = 1.0 / (1u64 << 53) as f64;
        let v = self.next_u64() >> 11;
        (v as f64) * DOUBLE_UNIT
    }

    fn gen_range(&mut self, upper: u64) -> u64 {
        // Lemire method
        let mut x = self.next_u64();
        let mut m = (x as u128) * (upper as u128);
        let mut l = m as u64;

        if l < upper {
            let t = upper.wrapping_neg() % upper;
            while l < t {
                x = self.next_u64();
                m = (x as u128) * (upper as u128);
                l = m as u64;
            }
        }

        (m >> 64) as u64
    }

    fn fill_bytes(&mut self, buf: &mut [u8]) {
        let mut i = 0;

        while i + 8 <= buf.len() {
            let v = self.next_u64().to_le_bytes();
            buf[i..i + 8].copy_from_slice(&v);
            i += 8;
        }

        if i < buf.len() {
            let v = self.next_u64().to_le_bytes();
            let remaining = buf.len() - i;
            buf[i..].copy_from_slice(&v[..remaining]);
        }
    }
}

pub trait SeedableRng: Sized {
    type Seed;

    fn from_seed(seed: Self::Seed) -> Self;

    fn seed_from_u64(seed: u64) -> Self;
}
