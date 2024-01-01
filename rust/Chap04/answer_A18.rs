use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;
    for i in 1..n+1 {
        for j in 0..s+1 {
            if dp[i-1][j] == true {
                dp[i][j] = true;
            if j+a[i-1] <= s && dp[i][j] ==true {
                dp[i][j+a[i-1]] = true;
            }
            }
        }
    }
    if dp[n][s]==true {
        println!("Yes");
    } else {
        println!("No");
    }
}
