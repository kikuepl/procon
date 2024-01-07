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
        c: char, 
        a: Bytes,
    }
    let mut chk = 0;
    for i in 0..n {
        if a[i] == b'W' {
            chk += 0;
        }
        else if a[i] == b'B' {
            chk += 1;
        } else {
            chk += 2;
        }
    }
    let mut b = -1;
    if c == 'W' {
        b = 0;
    }
    else if c == 'B' {
        b = 1;
    } else {
        b = 2;
    }
    if chk%3 == b {
        println!("Yes");
    } else {
        println!("No");
    }
}