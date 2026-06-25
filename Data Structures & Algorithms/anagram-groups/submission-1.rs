impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut dict: HashMap<[u8; 26], usize> = HashMap::new();
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut index: usize = 0; 

        for word in strs {
            let mut frequency = [0_u8; 26];
            for letter in word.bytes() {
                frequency[(letter - b'a') as usize] += 1;
            }

            // TOBEDONE: 
            // Remove double hash useless
            // Comment to justify & or not & 
            if dict.contains_key(&frequency) {
                match dict.get(&frequency) {
                    Some(&ind) => result[ind].push(word), 
                    None => println!("a"),
                }
            } else {
                dict.insert(frequency, index,);
                result.push(vec![word]);
                index +=1;
            }
        }
        result
    }
}
