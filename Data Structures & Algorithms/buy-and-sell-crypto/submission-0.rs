impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut curr_difference: i32 = -999;

        let mut buy_index = 0;
        let mut sell_index = 0;
        
        while buy_index < prices.len() {
            sell_index = buy_index;
            while sell_index < prices.len() {
                if prices[sell_index] - prices[buy_index] > curr_difference {
                    curr_difference = prices[sell_index] - prices[buy_index];
                }

                sell_index +=1;
            }
            buy_index += 1;
        }

        curr_difference
    }
}
