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
        q: usize,
    }
    let mut a = vec![0; n];
    for i in 0..n {
        a[i]=i+1;
    }
    let mut re = false;
    for i in 0..q {
        input! {
            t: usize
        }
        if t==1 {
            input! {
                x: usize,
                y: usize,
            }
            if re == true {
                a[n-x] = y;
            } else {
                a[x-1] = y;
            }
        }
        if t==2 {
            re = !re;
        }
        if t==3 {
            input! {
                x: usize,
            }
            if re == true {
                println!("{}", a[n-x])
            } else {
                println!("{}", a[x-1])
            }
        }
    }
}