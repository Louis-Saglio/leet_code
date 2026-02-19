/**
Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:

    0 <= a, b, c, d < n
    a, b, c, and d are distinct.
    nums[a] + nums[b] + nums[c] + nums[d] == target

You may return the answer in any order.



Example 1:

Input: nums = [1,0,-1,0,-2,2], target = 0
Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]

Example 2:

Input: nums = [2,2,2,2,2], target = 8
Output: [[2,2,2,2]]



Constraints:

    1 <= nums.length <= 200
    -109 <= nums[i] <= 109
    -109 <= target <= 109
**/

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        let mut result: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum =
                        nums[left] as i64 + nums[right] as i64 + nums[i] as i64 + nums[j] as i64;
                    if sum == target {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        left += 1;
                        right -= 1;
                    } else if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }
        result
    }
}

#[test]
fn test_1() {
    let solution = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
    println!("{:?}", solution);
    assert_eq!(solution.len(), 3);
    assert!(solution.contains(&vec![-2, -1, 1, 2]));
    assert!(solution.contains(&vec![-2, 0, 0, 2]));
    assert!(solution.contains(&vec![-1, 0, 0, 1]));
}

#[test]
fn test_2() {
    let solution = Solution::four_sum(vec![2, 2, 2, 2, 2], 8);
    println!("{:?}", solution);
    assert_eq!(solution, vec![vec![2, 2, 2, 2]])
}

#[test]
fn test_3() {
    let solution = Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0);
    println!("{:?}", solution);
    assert_eq!(solution.len(), 2);
    assert!(solution.contains(&vec![-1, -1, 1, 1]));
    assert!(solution.contains(&vec![-2, -1, 1, 2]));
}

#[test]
fn test_4() {
    let solution = Solution::four_sum(
        vec![1000000000, 1000000000, 1000000000, 1000000000],
        -294967296,
    );
    println!("{:?}", solution);
    assert_eq!(solution, Vec::<Vec<i32>>::new());
}
