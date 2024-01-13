use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
use std::usize;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeSet;
const MOD: isize = 1000000007;
const INF: isize = 1 << 60;

fn main() {
    input! {
        n: usize,
    }
    let mut task = vec![];
    let mut dd = 0;
    for _ in 0..n {
        input! {
            t: usize,
            d: usize,
        }
        task.push((d, t));
        dd = max(dd, d);
    }
    task.sort();
    let mut dp = vec![vec![0; dd+1]; n+1];
    for i in 1..=n {
        let (d, t) = task[i-1];
        for j in 0..=dd {
            dp[i][j] = max(dp[i][j], dp[i-1][j]);
            if j+t <= d {
                dp[i][j+t] = max(dp[i][j+t], dp[i-1][j]+1);
            }
        }
    }
    let mut ans = 0;
    for v in &dp[n] {
        ans = max(ans, *v);
    }
    println!("{}",ans);
}