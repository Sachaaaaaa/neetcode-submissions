impl Solution {

    /* 
    Not optimal because contains + insert compute two times the same hash if eleme is not in the HashSet
    
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut dict = HashSet::new(); // Pas de with capacity car retour anticipé

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
        // Omit with_capacity() to optimize for early exits, at the cost of slower worst-case performance (no duplicates) due to dynamic reallocations.
        let mut dict = HashSet::new();

        for elem in nums {
            if !dict.insert(elem) { // insert returns true if insert (i.e elem wasn't there), false if not
                return true;
            }
        }

        false
    }
}
