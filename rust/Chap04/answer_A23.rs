use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;

fn main() {
    let inf = 1000000007;
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let mut dp = vec![vec![inf; 1 << n]; m+1];
    dp[0][0]=0;
    for i in 1..m+1 {
        for j in 0..1<<n {
            let mut already = vec![0; n];
            for k in 0..n {
                if (j & (1 << k)) == 0 {
                    already[k]=0;
                } else {
                    already[k]=1;
                };
            }
            let mut v = 0;
            for k in 0..n {
                if already[k]==1 || a[i-1][k] == 1 {
                    v+=1 << k
                }
            }
            dp[i][j]=min(dp[i-1][j], dp[i][j]);
            dp[i][v]=min(dp[i][v], dp[i-1][j]+1);
        }
    }
    let ans = if dp[m][(1 << n) - 1] == inf {
        -1
    } else {
        dp[m][(1 << n) - 1]
    };
    println!("{}", ans);
}