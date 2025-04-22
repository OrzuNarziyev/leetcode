struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        if digits.len() == 0 {
            return vec![1];
        }
        for i in digits.iter_mut().rev() {
            if *i + 1 < 10 {
                *i += 1;
                return digits;
            } else {
                *i = 0
            }
        }
        digits.insert(0, 1);
        digits
    }
}

fn main() {
    // let result = Solution::plus_one(vec![7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 6]);
    let result = Solution::plus_one(vec![9, 9, 9]);

    println!("{:?}", result)
}
