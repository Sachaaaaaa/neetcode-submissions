impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        let mut res = 0;
        for i in 0.. 32{
            // i -> 31 - i
            let bit = n >> i & 0x1; 
            res |= (bit<<(31-i));
        }

        res
    }
}
