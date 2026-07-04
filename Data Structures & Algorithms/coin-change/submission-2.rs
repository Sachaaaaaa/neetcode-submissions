impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![amount as i32 + 1; amount + 1];
        dp[0] = 0;

        for a in 1..=amount {
            for &c in &coins {
                if c as usize <= a {
                    dp[a] = dp[a].min(dp[a - c as usize] + 1);
                }
            }
        }

        if dp[amount] > amount as i32 { -1 } else { dp[amount] }
    }
}