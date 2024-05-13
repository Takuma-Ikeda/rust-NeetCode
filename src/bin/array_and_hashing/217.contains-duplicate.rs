/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

#[allow(unused)]
struct Solution;

// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut hash_map = HashMap::new();

        for num in nums {
            if hash_map.contains_key(&num) {
                return true;
            } else {
                hash_map.insert(num, 1);
            }
        }

        false
    }
}

// @lc code=end

fn main() {
    println!("Hello, world!");
}
