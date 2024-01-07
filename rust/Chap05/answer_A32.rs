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
        a: [usize; n]
    }
    let mut nim = a[0];
    for i in 1..n {
        nim ^= a[i];
    }
    if nim == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
