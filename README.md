# LeetCode Challenges - 2026 (made with Rust)

![Rust](https://img.shields.io/badge/rust-2024.0-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Progress

Updated: 2026-01-23

### Monthly Difficulty Breakdown
| Month | Easy | Medium | Hard | Total |
|-------|------|--------|------|-------|
| January | 5 | 13 | 5 | 23 |
| **Total** | **5** | **13** | **5** | **23** |

### January
 ```
Progress: ██████████████░░░░░░ 23/31 days (74%)
Remaining: 8 problems
```

| Day | Problem | Difficulty | Status |
|-----|---------|------------|--------|
| 1 | [66. Plus One](https://leetcode.com/problems/plus-one/?envType=daily-question&envId=2026-01-01) | Easy | ✅ |
| 2 | [961. N-Repeated Element in Size 2N Array](https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/?envType=daily-question&envId=2026-01-02) | Easy | ✅ |
| 3 | [1411. Number of Ways to Paint N × 3 Grid](https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/description/?envType=daily-question&envId=2026-01-03) | Hard | ✅ |
| 4 | [1390. Four Divisors](https://leetcode.com/problems/four-divisors/description/?envType=daily-question&envId=2026-01-04) | Medium | ✅ |
| 5 | [1975. Maximum Matrix Sum](https://leetcode.com/problems/maximum-matrix-sum/?envType=daily-question&envId=2026-01-05) | Medium | ✅ |
| 6 | [1161. Maximum Level Sum of a Binary Tree](https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/description/?envType=daily-question&envId=2026-01-06) | Medium | ✅ |
| 7 | [1339. Maximum Product of Splitted Binary Tree](https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/?envType=daily-question&envId=2026-01-07) | Medium | ✅ |
| 8 | [1458. Max Dot Product of Two Subsequences](https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/?envType=daily-question&envId=2026-01-08) | Hard | ✅ |
| 9 | [865. Smallest Subtree with all the Deepest Nodes](https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/?envType=daily-question&envId=2026-01-09) | Medium | ✅ |
| 10 | [712. Minimum ASCII Delete Sum for Two Strings](https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description/?envType=daily-question&envId=2026-01-10) | Medium | ✅ |
| 11 | [85. Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle/description/?envType=daily-question&envId=2026-01-11) | Hard | ✅ |
| 12 | [1266. Minimum Time Visiting All Points](https://leetcode.com/problems/minimum-time-visiting-all-points/description/?envType=daily-question&envId=2026-01-12) | Easy | ✅ |
| 13 | [3453. Separate Squares I](https://leetcode.com/problems/separate-squares-i/description/?envType=daily-question&envId=2026-01-13) | Medium | ✅ |
| 14 | [3454. Separate Squares II](https://leetcode.com/problems/separate-squares-ii/description/?envType=daily-question&envId=2026-01-14) | Hard | ✅ |
| 15 | [2943. Maximize Area of Square Hole in Grid](https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/description/?envType=daily-question&envId=2026-01-15) | Medium | ✅ |
| 16 | [2975. Maximum Square Area by Removing Fences From a Field](https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/description/?envType=daily-question&envId=2026-01-16) | Medium | ✅ |
| 17 | [3047. Find the Largest Area of Square Inside Two Rectangles](https://leetcode.com/problems/find-the-largest-area-of-square-inside-two-rectangles/description/?envType=daily-question&envId=2026-01-17) | Medium | ✅ |
| 18 | [1895. Largest Magic Square](https://leetcode.com/problems/largest-magic-square/description/?envType=daily-question&envId=2026-01-18) | Medium | ✅ |
| 19 | [1292. Maximum Side Length of a Square with Sum Less than or Equal to Threshold](https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/description/?envType=daily-question&envId=2026-01-19) | Medium | ✅ |
| 20 | [3314. Construct the Minimum Bitwise Array I](https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/description/?envType=daily-question&envId=2026-01-20) | Easy | ✅ |
| 21 | [3314. Construct the Minimum Bitwise Array II](https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii/description/?envType=daily-question&envId=2026-01-21) | Medium | ✅ |
| 22 | [3507. Minimum Pair Removal to Sort Array I](https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i/?envType=daily-question&envId=2026-01-22) | Easy | ✅ |
| 23 | [3510. Minimum Pair Removal to Sort Array II](https://leetcode.com/problems/minimum-pair-removal-to-sort-array-ii/?envType=daily-question&envId=2026-01-23) | Hard | ✅ |
| 24 | TBD | - | ⬜ |
| 25 | TBD | - | ⬜ |
| 26 | TBD | - | ⬜ |
| 27 | TBD | - | ⬜ |
| 28 | TBD | - | ⬜ |
| 29 | TBD | - | ⬜ |
| 30 | TBD | - | ⬜ |
| 31 | TBD | - | ⬜ |


## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific month
cargo test january
```

## Structure

```
src/
├── lib.rs
└── january/
    ├── mod.rs
    ├── day_1.rs
    ├── day_2.rs
    └── ...
```

## Adding New Days

Create a new file `src/{month}/day_{day}.rs` with the following format:

```rust
// https://leetcode.com/problems/problem-name
// Problem Number. Problem Name
// Difficulty
// Description

Problem completion
```

Add module to `src/{month}/mod.rs`:

```rust
pub mod day_{day};
```

Run `./update_progress.sh` to update the README
