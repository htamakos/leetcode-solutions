#![allow(dead_code)]
#![allow(unused_variables)]
struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        'outer: for (i, &v1) in nums.iter().enumerate() {
            for (j, &v2) in nums[(i + 1)..].iter().enumerate() {
                if (v1 + v2) == target {
                    ans.push(i as i32);
                    ans.push((i + j + 1) as i32);
                    break 'outer;
                }
            }
        }

        ans
    }

    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let h: HashMap<i32, i32> = nums
            .iter()
            .enumerate()
            .map(|iv| (*iv.1, iv.0 as i32))
            .collect();

        for (i, &v) in nums.iter().enumerate() {
            match h.get(&(target - v)) {
                Some(&j) if j != (i as i32) => {
                    ans.push(i as i32);
                    ans.push(j);
                    return ans;
                }
                _ => continue,
            }
        }

        ans
    }
}

#[test]
fn test_solution1_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ans = vec![0, 1];

    assert_eq!(Solution::two_sum1(nums, target), ans);
}

#[test]
fn test_solution1_2() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ans = vec![0, 1];

    assert_eq!(Solution::two_sum2(nums, target), ans);
}

#[test]
fn test_solution2_1() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let ans = vec![1, 2];

    assert_eq!(Solution::two_sum1(nums, target), ans);
}

#[test]
fn test_solution2_2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let ans = vec![1, 2];

    assert_eq!(Solution::two_sum2(nums, target), ans);
}
