impl Solution {
    // Note : this program assume only english letters and crash if e.g. '-' is present
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        let mut dict = [0; 26];

        // .bytes() is faster than .char() because with .char() Rust must be aware of UTF-8 e.g. read 2 bytes for 'é'
        for (byte_s, byte_t) in s.bytes().zip(t.bytes()) {
            
            // Letters interpreted as int go from 97 to 122
            // By subtracting by b'a' we stay in the range {0, ..., 25}
            dict[(byte_s - b'a') as usize] += 1;
            dict[(byte_t - b'a') as usize] -= 1;
        }

        // Check that all letters apperead the same amount of time in each string
        // and therefor - and + compensate to give 0 everywhere
        dict.iter().all(|&x| x == 0)
    }

}
