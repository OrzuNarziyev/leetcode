pub struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut index = 0;
        for (i, num) in nums.iter().enumerate() {
            if target == *num {
                index = i;
                break;
            } else if target > *num {
                index = i + 1
            }
        }
        return index as i32;
    }
    pub fn search_insert_two(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        return nums.binary_search(&target).unwrap_or_else(|i| i) as i32;
    }
}
fn main() {
    let result = Solution::search_insert_two(vec![1,3,5,6], 2);
    println!("{:?}", result)
}
