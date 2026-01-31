// https://leetcode.com/problems/find-smallest-letter-greater-than-target/description/?envType=daily-question&envId=2026-01-31
// 744. Find Smallest Letter Greater Than Target
// Easy
// You are given an array of characters letters that is sorted in non-decreasing order
// and a character target
// There are at least two different characters in letters.
// Return the smallest character in letters that is lexicographically greater than target.
// If such a character does not exist, return the first character in letters.

pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut res: Option<char> = None;

        for &letter in &letters {
            if letter > target {
                if let Some(current_solution) = res {
                    res = Some(current_solution.min(letter))
                } else {
                    res = Some(letter)
                }
            }
        }

        if let Some(res) = res { res } else { letters[0] }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_greater_letter() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
            'c'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'),
            'x'
        );
    }
}
