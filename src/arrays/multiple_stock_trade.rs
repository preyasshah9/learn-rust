//Leetcode Problem Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut cur_min = prices[0];

        for i in 1..prices.len() {
            if prices[i] > cur_min {
                sum = sum + (prices[i] - cur_min);
            }
            cur_min = prices[i];
        }
        return sum as i32;
    }
}
