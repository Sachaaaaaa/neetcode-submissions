impl Solution {

    /* 
    Not optimal because if contains + insert compute two times the same hash 
    if an element is not in the HashSet
    
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut dict = HashSet::new();

        for elem in nums {
            if dict.contains(&elem) {
                return true;
            } else {
                dict.insert(elem);
            }
        }

        false
    } */

    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut dict = HashSet::new();

        for elem in nums {
            if !dict.insert(elem) {
                return true;
            } 
        }

        false
    }
}
