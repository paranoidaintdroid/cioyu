use crate::splitmix64::SplitMix64;
use crate::traits::Rng;

#[allow(dead_code)]
#[inline]
pub fn expand_2(seed: u64) -> [u64; 2] {
    let mut sm = SplitMix64::new(seed);

    [sm.next_u64(), sm.next_u64()]
}

#[inline]
pub fn expand_4(seed: u64) -> [u64; 4] {
    let mut sm = SplitMix64::new(seed);

    [sm.next_u64(), sm.next_u64(), sm.next_u64(), sm.next_u64()]
}

#[allow(dead_code)]
#[inline]
pub fn expand_8(seed: u64) -> [u64; 8] {
    let mut sm = SplitMix64::new(seed);

    [
        sm.next_u64(),
        sm.next_u64(),
        sm.next_u64(),
        sm.next_u64(),
        sm.next_u64(),
        sm.next_u64(),
        sm.next_u64(),
        sm.next_u64(),
    ]
}
