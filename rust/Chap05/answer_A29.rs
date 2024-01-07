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
        a: usize,
        b: usize,
    }
    let answer = pow(&a, &b, &MOD);
    println!("{}", answer);
}

fn pow(a: &usize, b: &usize, m: &usize) -> usize {
    let mut p: usize = *a;
    let mut ans: usize = 1;
    for i  in 0..30 {
        let chk:usize = 1 << i;
        let check = b/chk;
        if check%2 == 1 {
            ans *= p;
            ans %= m;
        }
        p *= p;
        p %= m;
    }
    return ans;
}