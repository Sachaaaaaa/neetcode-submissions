impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut right_index = nums.len() - 1;

        while left_index < right_index {
            // Rust's shadowing allows us to ovewrite a variable without specifying mut 
            let middle = left_index + (right_index - left_index) / 2; 

            // IMPORTANT: We must compare 'middle' with 'right', not 'left'.
            // Comparing with 'left' fails if the array is perfectly sorted (0 rotations).
            if nums[middle] < nums[right_index] {
                // The right half is strictly sorted. 
                // The minimum is either 'middle' itself or somewhere to its left.
                right_index = middle;
            } else {
                // nums[middle] >= nums[right_index]
                // The shifting point (minimum) is strictly to the right side.
                left_index = middle + 1; 
            }
        }

        // When left_index == right_index, we have found the minimum.
        nums[right_index]
    }
}