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
        a: [usize; n-1],
        b: [usize; n-1]
    }
    let mut dp = vec![-1000000007; n+1];
    dp[1]=0;
    for i in 1..=n-1 {
        let ai = a[i-1];
        let bi = b[i-1];
        dp[ai]=max(dp[i]+100, dp[ai]);
        dp[bi]=max(dp[i]+150, dp[bi]);
    }
    println!("{}", dp[n]);
}