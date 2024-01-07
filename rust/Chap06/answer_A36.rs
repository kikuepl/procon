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
        mut n: usize,
        k: usize,
    }
    n = 2*(n-1);
    if k<n || k%2 == 1{
        println!("No");
    } else {
        println!("Yes");
    }
}