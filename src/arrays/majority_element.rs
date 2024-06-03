//Leetcode Problem Link: https://leetcode.com/problems/majority-element

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut elements = HashMap::new();
        let mut output = 0;
        let mut max = 0;
        for num in nums {
            if let Some(x) = elements.get_mut(&num) {
                *x += 1;
            }
            else {
                elements.insert(num, 1);
            }
        }
        for (key, val) in &elements {
            if *val > max {
                max = *val;
                output = *key;
            }
        }
        output as i32
    }
}