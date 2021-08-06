pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pairs = HashMap::with_capacity(nums.len());
        
        for (i, num) in nums.iter().enumerate() {
            match pairs.get(&(target - num)) {
                None => {
                    pairs.insert(num, i);
                }
                Some(sub_i) => return vec![*sub_i as i32, i as i32],
            }
        }
        vec![]
    }
}
