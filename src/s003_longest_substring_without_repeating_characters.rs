pub struct Solution{}
use std::{cmp::max, collections::HashMap};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::with_capacity(s.len());
        let mut max_size = 0;
        let mut substr_start = 0;
        for (idx, c) in s.chars().enumerate() {
            if let Some(sub_idx) = map.insert(c, idx) {
                if sub_idx >= substr_start {
                    max_size = max(max_size, idx - substr_start);
                    substr_start = sub_idx + 1;
                }
            };
        }
        max(max_size, s.len() - substr_start) as i32
    }
}
