pub struct Solution;
impl Solution {
    pub fn remove_element_one(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        nums.retain(|num| *num == val);
        return nums.len() as i32;
    }

    pub fn remove_element_two(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut count = 0;
        for (i, num) in nums.to_owned().iter().enumerate() {
            if num == &val {
                nums.remove(i - count);
                count += 1;
            }
        }
        count as i32
    }
}

fn main() {
    let result = Solution::remove_element_two(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2);
    println!("{:?}", result)
}