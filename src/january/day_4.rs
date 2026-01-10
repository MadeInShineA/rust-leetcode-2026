// https://leetcode.com/problems/four-divisors/description/?envType=daily-question&envId=2026-01-04
// 1390. Four Divisors
// Medium
// Given an integer array nums, return the sum of divisors of the integers in that array that have exactly four divisors.
// If there is no such integer in the array, return 0.
pub struct Solution;

impl Solution {
    fn sum_of_four_divisors(num: i32) -> Option<i32> {
        let mut divisors_sum: i32 = 0;
        let mut count: i32 = 0;
        let sqrt_num = (num as f32).sqrt() as i32;

        for i in 1..=sqrt_num {
            if (num % i) == 0 {
                if i != num / i {
                    divisors_sum += i + num / i;
                    count += 2;
                } else {
                    divisors_sum += i;
                    count += 1;
                }
                if count > 4 {
                    return None;
                }
            }
        }

        if count == 4 { Some(divisors_sum) } else { None }
    }
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter_map(|&num| Solution::sum_of_four_divisors(num))
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sum_four_divisors() {
        assert_eq!(Solution::sum_four_divisors(vec![21, 4, 7]), 32);
        assert_eq!(Solution::sum_four_divisors(vec![21, 21]), 64);
        assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
    }
}
