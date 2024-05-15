/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */


struct Solution;

// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut hash_map = HashMap::new();

        for char in s.chars() {
            if hash_map.contains_key(&char) {
                if let Some(count) = hash_map.get_mut(&char) {
                    *count += 1;
                }
            } else {
                hash_map.insert(char, 1);
            }
        }

        for char in t.chars() {
            if hash_map.contains_key(&char) {
                let mut result = None;

                if let Some(count) = hash_map.get_mut(&char) {
                    *count -= 1;
                    result = Some(*count);
                }

                if result.unwrap_or(0) < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        let filtered: Vec<_> = hash_map.iter().filter(|&(_, &v)| v > 0).collect();

        if filtered.len() > 0 {
            return false;
        }

        true
    }
}
// @lc code=end

fn main() {
    println!("Hello, world!");
}
