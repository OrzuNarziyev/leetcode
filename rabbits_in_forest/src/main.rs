use std::collections::HashMap;
use std::hash::Hash;

pub struct Solution;


impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        answers.iter().fold(
            HashMap::<&i32, i32>::new(), |mut m, i| {
                *m.entry(&i).or_insert(0) += 1;
                m
            }).into_iter().map(|(k, v)| ((k + v) / (k + 1)) * (k + 1)).sum()
    }
}
fn main() {
    let _result = Solution::num_rabbits(vec![1, 1, 2]);
}