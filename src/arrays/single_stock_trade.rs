https://leetcode.com/problems/best-time-to-buy-and-sell-stock

use std::cmp::{max, min};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }
        let mut result = 0;
        let mut start = prices[0];
        for price in prices {
            result = max(price - start, result);
            start = min(price, start);
        }
        return result as i32;
    }
}
