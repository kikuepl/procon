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
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n]
    }
    let mut ans = vec![24; d+1];
    for i in 1..=n {
        let (l,r,h) = lrh[i-1];
        for j in l..=r {
            ans[j]=min(ans[j],h);
        }
    }
    let mut answer = 0;
    for i in 1..=d {
        answer += ans[i];
    }
    println!("{}", answer);
}
