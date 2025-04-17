pub struct Solution;

impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j]
            }
        }
        nums.truncate(i + 1);
        (i + 1) as i32
    }
}

fn main() {
    let result = Solution::remove_duplicates(&mut vec![1, 1, 2]);
    println!("{:?}", result)
    // put you code here to launch it
}
