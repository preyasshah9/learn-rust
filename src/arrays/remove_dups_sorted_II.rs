//Leetcode Problem Link: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 1;
        let mut k = 1;
        let mut cur_idx = 0;
        let mut cur_val = nums[0];
        for j in 1..nums.len() {
            if nums[j] == cur_val {
                count += 1;
            }
            else {
                count = 1;
                cur_val = nums[j];
            }
            if count <= 2 {
                nums[cur_idx + 1] = nums[j];
                cur_idx += 1;
                k += 1
            }
        }
        return k as i32;
    }
}