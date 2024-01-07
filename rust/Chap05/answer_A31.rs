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
    }
    let n3 = n/3;
    let n5 = n/5;
    let n15 = n/15;
    println!("{}", n3 + n5 -n15);
}
