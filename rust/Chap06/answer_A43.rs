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
        l: usize,
        ab: [(usize, char); n]
    }
    let mut ans = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        if b == 'E' {
            ans=max(l-a,ans);
        }
        if b == 'W' {
            ans = max(a, ans);
        }
    }
    println!("{}", ans );
}