impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo = HashMap::new();

        fn dfs(coins: &[i32], amount: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if amount == 0 { return 0; }
            if let Some(&val) = memo.get(&amount) {
                return val;
            }

            let mut res = i32::MAX;
            for &coin in coins {
                if amount - coin >= 0 {
                    let result = dfs(coins, amount - coin, memo);
                    if result != i32::MAX {
                        res = res.min(1 + result);
                    }
                }
            }

            memo.insert(amount, res);
            res
        }

        let min_coins = dfs(&coins, amount, &mut memo);
        if min_coins == i32::MAX { -1 } else { min_coins }
    }
}