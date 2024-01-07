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
        m: usize,
        b: usize,
        mut a: [usize; n],
        mut c: [usize; m]
    }
    for i in 0..n {
        a[i]+=b;
    }
    let mut sum_c = 0;
    for i in 0..m {
        sum_c += c[i];
    }
    let mut ans = 0;
    for i in 0..n {
        ans += a[i]*c.len()+sum_c;
    } 
    println!("{}", ans);
}