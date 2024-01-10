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
        lr: [(usize, usize); n]
    }
    let mut rl = vec![];
    for i in 0..n {
        let (l,r) = lr[i];
        rl.push((r, l));
    }
    rl.sort();
    let mut ans = 0;
    let mut pos = 0;
    for i in 0..n {
        let (r,l) = rl[i];
        if pos<=l {
            ans += 1;
            pos = r
        }
    }
    println!("{}", ans);
}

