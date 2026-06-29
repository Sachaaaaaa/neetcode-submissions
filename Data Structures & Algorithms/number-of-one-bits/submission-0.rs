impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut count = 0;
        let mut i = 0;
        while i < 32 {
            if (n >> i) & 0x1 == 1 {
                count += 1;
            }
            i += 1;
        }

        count
    }
}
