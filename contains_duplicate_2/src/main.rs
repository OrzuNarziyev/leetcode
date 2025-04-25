use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let j = map.entry(num).or_insert(i);
            if i != *j && i - *j <= k as usize {
                return true;
            } else {
                *j = i
            }
        }
        false
    }
}

fn main() {
    let nums = vec![1, 0, 1, 1];
    let result = Solution::contains_nearby_duplicate(nums, 1);
    println!("{}", result);
}
