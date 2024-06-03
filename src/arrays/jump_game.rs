impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let mut array = Vec::new();
        for i in 0..size {
            array.push(false);
        }
        array[0] = true;
        for idx in 0..size - 1 as usize {
            if !array[idx] {
                return false;
            }
            for j in 1..(nums[idx] + 1) as usize {
                if (j + idx) < size{
                    array[j + idx] = true;
                }
            }
        }
        return array[array.len() - 1] as bool;
    }
}