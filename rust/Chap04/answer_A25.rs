use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
const inf: usize = 1000000007;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Bytes; h]
    }
    let mut dp = vec![vec![0 as usize; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == b'#' || dp[i][j] == 1 {
                continue;
            }
            let hidari = if 0 < j {
                dp[i][j-1]
            } else {
                0
            };
            let ue = if 0 < i {
                dp[i-1][j]
            } else {
                0
            };
            dp[i][j] = hidari + ue;
        }
    }
    let ans = dp[h-1][w-1];
    println!("{}", ans);
}