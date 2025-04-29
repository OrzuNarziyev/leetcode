use std::cmp::max;

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let n = nums.len();
        let mut w = nums[..k].iter().sum::<i32>();
        let mut m = w;
        for i in k..n {
            w += nums[i] - nums[i - k];
            m = m.max(w);
        }
        m as f64 / k as f64
    }
}

fn main() {
    let result = Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4);
    println!("{}", result)
}