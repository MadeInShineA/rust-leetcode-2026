// https://leetcode.com/problems/separate-squares-ii/description/?envType=daily-question&envId=2026-01-14
// 3454. Separate Squares II
// Hard
// You are given a 2D integer array squares. Each squares[i] = [xi, yi, li]
// represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
// Find the minimum y-coordinate value of a horizontal line such that
// the total area of the squares above the line equals the total area of the squares below the line.
// Answers within 10-5 of the actual answer will be accepted.
// Note: Squares may overlap. Overlapping areas should be counted only once in this version.

pub struct Solution;

pub const EPSILON: f64 = 1e-5;

impl Solution {
    pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() <= epsilon
    }

    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let n = squares.len();

        let mut xs: Vec<i64> = Vec::with_capacity(2 * n);
        #[derive(Clone)]
        struct Event {
            y: i64,
            x1: i64,
            x2: i64,
            t: i64,
        }
        let mut events: Vec<Event> = Vec::with_capacity(2 * n);

        for s in squares {
            let x = s[0] as i64;
            let y = s[1] as i64;
            let l = s[2] as i64;
            xs.push(x);
            xs.push(x + l);
            events.push(Event {
                y,
                x1: x,
                x2: x + l,
                t: 1,
            });
            events.push(Event {
                y: y + l,
                x1: x,
                x2: x + l,
                t: -1,
            });
        }

        xs.sort_unstable();
        xs.dedup();
        let m = xs.len();

        use std::collections::HashMap;
        let mut idx = HashMap::with_capacity(m);
        for (i, &v) in xs.iter().enumerate() {
            idx.insert(v, i);
        }

        let mut cnt = vec![0i64; 4 * m];
        let mut seglen = vec![0i64; 4 * m];

        fn upd(
            i: usize,
            l: usize,
            r: usize,
            ql: usize,
            qr: usize,
            v: i64,
            xs: &Vec<i64>,
            cnt: &mut Vec<i64>,
            seglen: &mut Vec<i64>,
        ) {
            if qr <= l || r <= ql {
                return;
            }
            if ql <= l && r <= qr {
                cnt[i] += v;
            } else {
                let mid = (l + r) >> 1;
                upd(i << 1, l, mid, ql, qr, v, xs, cnt, seglen);
                upd(i << 1 | 1, mid, r, ql, qr, v, xs, cnt, seglen);
            }
            if cnt[i] > 0 {
                seglen[i] = xs[r] - xs[l];
            } else if l + 1 == r {
                seglen[i] = 0;
            } else {
                seglen[i] = seglen[i << 1] + seglen[i << 1 | 1];
            }
        }

        events.sort_by_key(|e| e.y);

        let mut prev = events[0].y;
        let mut total = 0f64;
        let mut i = 0usize;

        while i < events.len() {
            let y = events[i].y;
            let dy = y - prev;
            if dy > 0 && seglen[1] > 0 {
                total += seglen[1] as f64 * dy as f64;
            }
            while i < events.len() && events[i].y == y {
                let e = &events[i];
                upd(
                    1,
                    0,
                    m,
                    idx[&e.x1],
                    idx[&e.x2],
                    e.t,
                    &xs,
                    &mut cnt,
                    &mut seglen,
                );
                i += 1;
            }
            prev = y;
        }

        let half = total / 2.0;
        cnt.fill(0);
        seglen.fill(0);

        prev = events[0].y;
        let mut cur = 0f64;
        i = 0;

        while i < events.len() {
            let y = events[i].y;
            let dy = y - prev;
            if dy > 0 && seglen[1] > 0 {
                let area = seglen[1] as f64 * dy as f64;
                if cur + area >= half {
                    return prev as f64 + (half - cur) / seglen[1] as f64;
                }
                cur += area;
            }
            while i < events.len() && events[i].y == y {
                let e = &events[i];
                upd(
                    1,
                    0,
                    m,
                    idx[&e.x1],
                    idx[&e.x2],
                    e.t,
                    &xs,
                    &mut cnt,
                    &mut seglen,
                );
                i += 1;
            }
            prev = y;
        }

        prev as f64
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_separate_squares() {
        assert!(Solution::approx_equal(
            Solution::separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]),
            1.00000,
            EPSILON,
        ));

        assert!(Solution::approx_equal(
            Solution::separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]),
            1.00000,
            EPSILON
        ));
    }
}
