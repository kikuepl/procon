use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[n-1][i]=a[i];
    }
    for i in (0..n-1).rev() {
        for j in 0..=i {
            if (i+1)%2 == 0 {
                dp[i][j] = min(dp[i+1][j], dp[i+1][j+1])
            } else {
                dp[i][j] = max(dp[i+1][j], dp[i+1][j+1])
            }
        }
    }
    println!("{}", dp[0][0]);
}