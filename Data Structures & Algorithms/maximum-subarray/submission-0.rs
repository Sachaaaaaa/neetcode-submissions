impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut global_max = i32::MIN;
        let mut local_max = i32::MIN;

        for elem in nums{
            if local_max < 0 {
                local_max = elem;
            } else {
                local_max += elem;
            }
            if local_max > global_max {
                global_max = local_max;
            }
        }

        global_max
    }
}
