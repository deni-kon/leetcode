impl Solution {
  pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut sum: i64 = 0;
    let mut res: i64 = 0;
    for i in 0..nums.len() {
      sum += nums[i] as i64;
      res = std::cmp::max(res, (sum + i as i64) / (i as i64+ 1) )
    }
    res as i32
}}
