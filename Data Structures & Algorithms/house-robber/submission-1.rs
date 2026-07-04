
impl Solution {

/*
    // f(k) = max(f(k-1), f(k-2)+ nums[k])
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let mut money = vec![0; nums.len()];;
        money[0] = nums[0];

        money[1] = money[0].max(nums[1]); 

        let mut i = 2;
        while i < nums.len(){
            money[i] = money[i-1].max(money[i-2] + nums[i]); 
            i+=1;
        }

        money[i-1]
    }
    */

    // We only need to remember the last two elements :
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev2 = 0; // Représente f(k-2)
        let mut prev1 = 0; // Représente f(k-1)

        for num in nums {
            let current = prev1.max(prev2 + num);
            prev2 = prev1;
            prev1 = current;
        }

        prev1
    }
}
