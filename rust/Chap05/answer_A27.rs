use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
const inf: usize = 1000000007;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }
    while a != 0 && b!=0 {
        let c = max(a,b);
        let d = min(a,b);
        a = d;
        b = c-(c/d*d);
    }
    println!("{}",a);
}