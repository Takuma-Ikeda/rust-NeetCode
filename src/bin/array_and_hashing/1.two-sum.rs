/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

#[allow(unused)]
struct Solution;

// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        for (i, n) in nums.into_iter().enumerate(){
            let diff = target - n;

            if let Some(&j) = map.get(&diff){
                return vec![i as i32, j as i32];
            }else{
                map.insert(n, i);
            }
        }

        unreachable!()
    }
}

// @lc code=end

fn main() {
    println!("Hello, world!");
}
