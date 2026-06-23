impl Solution {
    fn hash_letter(letter: char) -> Option<usize> {
    match letter {
        'a' => Some(0),
        'b' => Some(1),
        'c' => Some(2),
        'd' => Some(3),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(6),
        'h' => Some(7),
        'i' => Some(8),
        'j' => Some(9),
        'k' => Some(10),
        'l' => Some(11),
        'm' => Some(12),
        'n' => Some(13),
        'o' => Some(14),
        'p' => Some(15),
        'q' => Some(16),
        'r' => Some(17),
        's' => Some(18),
        't' => Some(19),
        'u' => Some(20),
        'v' => Some(21),
        'w' => Some(22),
        'x' => Some(23),
        'y' => Some(24),
        'z' => Some(25),
        '-' => Some(25),
        _ => None, 
    }
    }


    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
            return false;
        }

        let mut dict: [i32; 27] = [0; 27];

        for (char_s, char_t) in s.chars().zip(t.chars()){
            let index_s = Solution::hash_letter(char_s).unwrap();
            dict[index_s] += 1;
            let index_t = Solution::hash_letter(char_t).unwrap();
            dict[index_t] -= 1;
        }

        dict.iter().all(|&x| x == 0)
    }
}
