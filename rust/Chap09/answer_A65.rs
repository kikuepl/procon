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
        a: [usize; n-1],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n-1 {
        g[a[i]-1].push(i+1);
    }
    let mut dp = vec![0; n];
    for i in (0..n).rev() {
        for v in 0..g[i].len() {
            dp[i] += dp[g[i][v]]+1;
        }
    }
    for v in &dp {
        print!("{} ", v);
    }
}