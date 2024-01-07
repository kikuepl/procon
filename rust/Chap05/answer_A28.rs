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
        n: usize,
    }
    let mut ans: isize = 0;
    for i in 0..n {
        input! {
            t: char,
            a: isize,
        }
        if t == '+' {
            ans += a;
        } else if t == '-' {
            ans -= a;
        } else if t == '*' {
            ans *= a;
        }
        ans %= 10000;
        if ans < 0 {
            println!("{}", ans+10000);
        } else {
            println!("{}", ans);
        }
    }
}