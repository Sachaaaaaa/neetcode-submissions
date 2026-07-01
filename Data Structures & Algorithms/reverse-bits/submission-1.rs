/*
impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        let mut res: u32 = 0;
        for i in 0..32 {
            let bit = (n >> i) & 1;
            res += bit << (31 - i);
        }
        res
    }
}*/

// Optimal version : bitwise divide and conquer
// Bitwise reverse using hierarchical swaps (bit-twiddling):
// progressively swap blocks of size 16, 8, 4, 2, then 1 bit using masks and shifts.
// Each step locally reverses groups, and together they produce the full bit reversal.
// No loops over bits: constant number of mask/shift operations.
impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        let mut ret = n;
        ret = (ret >> 16) | (ret << 16);
        ret = ((ret & 0xff00ff00) >> 8) | ((ret & 0x00ff00ff) << 8);
        ret = ((ret & 0xf0f0f0f0) >> 4) | ((ret & 0x0f0f0f0f) << 4);
        ret = ((ret & 0xcccccccc) >> 2) | ((ret & 0x33333333) << 2);
        ret = ((ret & 0xaaaaaaaa) >> 1) | ((ret & 0x55555555) << 1);
        ret
    }
}