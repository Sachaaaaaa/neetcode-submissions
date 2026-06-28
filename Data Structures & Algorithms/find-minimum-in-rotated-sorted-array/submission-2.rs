impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut right_index = nums.len()-1;

        while left_index < right_index{
            let middle = left_index + (right_index - left_index) / 2;

            if nums[middle] < nums[right_index] {
                right_index = middle ;
            } else {
                left_index = middle + 1; 
            }
        }

        nums[right_index]
    }
}
