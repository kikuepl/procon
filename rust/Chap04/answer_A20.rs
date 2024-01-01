use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;

fn main() {
    input! {
        S: Bytes,
        T: Bytes,
    }
    let s = S.len();
    let t = T.len();
    let mut dp = vec![vec![0; s+1]; t+1];
    for i in 1..=t {
        for j in 1..=s {
            if T[i-1]==S[j-1] {
                dp[i][j]=dp[i-1][j-1]+1;
            } else {
                dp[i][j]=max(dp[i-1][j],dp[i][j-1]);
            }
        }
    }
    println!("{}", dp[t][s]);
}
