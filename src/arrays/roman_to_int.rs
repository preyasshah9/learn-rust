//Leetcode Problem Link: https://leetcode.com/problems/roman-to-integer

use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut char_map = HashMap::new();
        char_map.insert('I', 1);
        char_map.insert('V', 5);
        char_map.insert('X', 10);
        char_map.insert('L', 50);
        char_map.insert('C', 100);
        char_map.insert('D', 500);
        char_map.insert('M', 1000);
        let mut idx = s.len();
        let mut output = 0;
        let mut chars: Vec<char> = s.chars().collect();
        while idx > 0 {
            let mut cur_ch = chars[idx - 1];
            let mut expr = 0;
            if idx > 1 {
                let prev_ch = chars[idx - 2];
                if char_map[&prev_ch] < char_map[&cur_ch] {
                    expr = char_map[&cur_ch] - char_map[&prev_ch];
                    idx = idx - 1;
                }
                else {
                    expr = char_map[&cur_ch]; 
                }
            }
            else {
                expr = char_map[&cur_ch];
            }
            output = output + expr;
            idx = idx - 1;
        }
        return output as i32;
    }
}
