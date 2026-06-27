impl Solution {
    // To solve this, we use dynamic programming.
    // The best overall profit is the maximum among the best profits of each day.
    // The best profit for a given day is equal to today's price (we can only sell today) minus the smallest previous price (the best buy).
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // Safety check: if the array is empty, we can't make any profit.
        if prices.is_empty() {
            return 0;
        }

        let mut max_profit = 0; // The profit if we do nothing (or buy and sell on the same day).
        let mut min_buy = prices[0];

        // &prices allows us to use a reference to avoid destroying prices at the end of the loop.
        // &sell dereferences the &i32 given by the for loop to do math on the value instead of the reference.
        for &sell in &prices {
            max_profit = max_profit.max(sell - min_buy); // Compare the best profit of today with the stored best profit.
            min_buy = min_buy.min(sell); // See if today is a better day to buy.
        }
        
        max_profit
    }
}