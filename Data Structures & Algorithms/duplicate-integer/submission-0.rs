impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut dict = HashSet::new();

        for elem in &nums {
            if dict.contains(elem) {
                return true;
            } else {
                dict.insert(elem);
            }
        }

        false
    }
}
