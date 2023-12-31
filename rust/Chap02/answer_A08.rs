use proconio::input;
use std::f64::consts::PI;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[i32; w]; h],
        q: usize,
        queries: [(usize, usize, usize, usize); q]
    }
    let mut S = vec![vec![0; w+1]; h+1];
    for i in 0..h {
        for j in 0..w {
            S[i+1][j+1] = x[i][j] + S[i+1][j] + S[i][j+1] - S[i][j];
        }
    }
    for query in queries {
        let (ai, bi, ci, di) = query;
        let ans = S[ci][di] + S[ai-1][bi-1] - S[ai-1][di] - S[ci][bi-1];
        println!("{}", ans);
    }
}