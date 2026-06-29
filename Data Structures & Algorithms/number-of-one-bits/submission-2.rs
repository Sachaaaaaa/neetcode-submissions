impl Solution {

    // Note: there is a built in solution : n.count_ones() as i32
    // Brian Kernighan Algorithm : 
    pub fn hamming_weight(n: u32) -> i32 {
        let mut count = 0;
        let mut n_mut = n;
        
        while n_mut != 0 {
            n_mut &= n_mut - 1; // Shut down the rightmost bit that is 1

            // e.g with n = 0011 1000
            // n - 1 = 0011 0111
            // so n &= n - 1 is 0011 0000

            count += 1;
        }

        count
    }
}
