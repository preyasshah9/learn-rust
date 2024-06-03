//Leetcode Problem Link: https://leetcode.com/problems/gas-station

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let gas_total: i32 = gas.iter().sum();
        let cost_total: i32 = cost.iter().sum();
        if gas_total < cost_total {
            return -1;
        }
        let mut tank = 0;
        let mut idx = 0;
        for i in 0..gas.len() {
            tank = tank + (gas[i] - cost[i]);
            if tank < 0 {
                tank = 0;
                idx = i + 1;
            }
        }
        idx as i32
    }
}