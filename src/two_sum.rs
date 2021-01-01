#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vec = Vec::with_capacity(2);
        'outer: for (i, v1) in nums.iter().enumerate() {
            for (j, v2) in nums.iter().enumerate() {
                if i != j && v1 + v2 == target {
                    vec.push(i as i32);
                    vec.push(j as i32);
                    break 'outer;
                }
            }
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(vec![0, 1], Solution::two_sum(nums, 9));
    }
}
