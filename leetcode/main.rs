struct Solution;
use std::cmp::{max, min};
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = k - 1;
        nums.sort();
        let mut min_diff = i32::MAX;
        for i in 0..nums.len() - k as usize {
            min_diff = min(min_diff, nums[i + k as usize] - nums[i]);
        }
        min_diff
    }
}

fn main() {
    let nums = vec![9, 4, 1, 7];
    let k = 2;
    let result = Solution::minimum_difference(nums, k);
    println!("result: {}", result);
}
