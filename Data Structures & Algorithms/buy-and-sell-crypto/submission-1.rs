impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut curr_difference: i32 = -999;

        let mut buy_index = 0;
        let mut sell_index = 0;
        
        let mut i = 0;

        while i < prices.len() {
            let curr_price = prices[i];

            if curr_price < prices[buy_index] {
                buy_index = i;
                sell_index = i;
            } else if curr_price > prices[sell_index] {
                sell_index = i;
            }

            if prices[sell_index] - prices[buy_index] > curr_difference {
                curr_difference = prices[sell_index] - prices[buy_index];
            }

            i += 1;
        }

        curr_difference
    }
}
