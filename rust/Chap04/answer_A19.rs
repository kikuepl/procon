use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;

fn main() {
    input! {
        n: usize,
        w: usize,
        v: [(usize, usize); n],
    }

    let mut dp = vec![vec![-1; w+1]; n+1];
    dp[0][0]=0;
    for i in 1..n+1 {
        for j in 0..w+1 {
            if dp[i-1][j] != -1 {
                dp[i][j]=max(dp[i][j], dp[i-1][j]);
                let (wei, value) = v[i-1];
                if wei+j<=w {
                    dp[i][wei+j]=max(dp[i][wei+j], dp[i-1][j]+value as i64);
                }
            }
        }
    }
    let mut ans = 0;
    for &i in &dp[n] {
        ans = max(ans, i);
    }
    println!("{}", ans);
}
