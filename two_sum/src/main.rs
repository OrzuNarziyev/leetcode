use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (index, &element) in nums.iter().enumerate() {
            let complement = target - element;
            if let Some(&complement_index) = map.get(&complement) {
                return vec![complement_index as i32, index as i32];
            }
            map.insert(element, index);
        }
        vec![]
    }
}

fn main() {
    let answer: Vec<i32> = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", answer);
}