use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut j = 0;

    for i in 0..n {
        while j < n && a[j] - a[i] <= k {
            j += 1;
        }
        ans += j - i - 1; 
    }

    println!("{}", ans);
}