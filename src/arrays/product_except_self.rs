//Leetcode Problem Link: https://leetcode.com/problems/product-of-array-except-self

use std::collections::VecDeque;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![0; nums.len()];
        let mut suffix = vec![0; nums.len()];
        let mut result = vec![0; nums.len()];
        
        suffix[nums.len() - 1] = 1;
        for i in (1..nums.len()).rev() {
            suffix[i - 1] = suffix[i] * nums[i];
        }
        prefix[0] = 1;
        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in 0..nums.len() {
            result[i] = prefix[i] * suffix[i];
        }
        return result;
    }
}