use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n]
    }
    let mut dp = vec![vec![0; n+1]; n+1];
    for len in (0..n-1).rev() {
        for l in 1..=n-len {
            let r = len + l;
            let mut scr1 = 0;
            let mut scr2 = 0;
            if 1 < l {
                let (pl, al) = pa[l-2];
                if l <= pl && pl <= r {
                    scr1 = al;
                }
            };
            if r < n {
                let (pr, ar) = pa[r];
                if l <= pr && pr <= r {
                    scr2 = ar;
                }
            };
            if l == 1 {
                dp[l][r] = dp[l][r+1]+scr2;
            }
            if r == n {
                dp[l][r] = dp[l-1][r]+scr1;
            } else {
                dp[l][r] = max(dp[l-1][r]+scr1, dp[l][r+1]+scr2);
            }
        }
    }
    let mut ans = 0;
    for i in 1..=n {
        ans = max(ans, dp[i][i]);
    }
    println!("{}", ans);
}