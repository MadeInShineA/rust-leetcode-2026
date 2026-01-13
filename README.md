# LeetCode Challenges - 2026 (made with Rust)

![Rust](https://img.shields.io/badge/rust-2024.0-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Progress

Updated: 2026-01-13

### Monthly Difficulty Breakdown
| Month | Easy | Medium | Hard | Total |
|-------|------|--------|------|-------|
| January | 3 | 7 | 3 | 13 |
| **Total** | **3** | **7** | **3** | **13** |

### January
 ```
Progress: ████████░░░░░░░░░░░░ 13/31 days (41%)
Remaining: 18 problems
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
| 14 | TBD | - | ⬜ |
| 15 | TBD | - | ⬜ |
| 16 | TBD | - | ⬜ |
| 17 | TBD | - | ⬜ |
| 18 | TBD | - | ⬜ |
| 19 | TBD | - | ⬜ |
| 20 | TBD | - | ⬜ |
| 21 | TBD | - | ⬜ |
| 22 | TBD | - | ⬜ |
| 23 | TBD | - | ⬜ |
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
