impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_p = 0;
        let mut min_buy = prices[0];

        for &sell in &prices {
            max_p = max_p.max(sell - min_buy);
            min_buy = min_buy.min(sell);
        }
        max_p
    }
}