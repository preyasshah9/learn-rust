//Leetcode Problem Link: https://leetcode.com/problems/merge-sorted-array/

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        //Keep Track of index at the current element.
        let (mut m, mut n) = (m as usize, n as usize);

        //For Merging Sorted Arrays, we start from the end, comparing the element, and put the higher value at the end.
        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}
