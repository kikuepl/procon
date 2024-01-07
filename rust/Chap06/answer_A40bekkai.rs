use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
const MOD: usize = 1000000007;
const ans: usize = 0;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut cnt = vec![0; 101];
    for ai in &a {
        cnt[*ai]+=1;
    }
    let mut ans: usize = 0;
    for i in &cnt {
        ans += (i*(i-1)*(i-2))/6;
    }
    println!("{}", ans);
}