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
    let mut left = 0;
    let mut right = 1000000001;
    while left < right {
        let mid = (left + right) / 2;
        if check(&a, &mid, &k) == true {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    println!("{}", left);
}

fn check(v: &Vec<usize>, x: &usize, k: &usize) -> bool{
    let mut cnt = 0;
    for i in &v {
        cnt += x/i;
    }
    cnt >= *k
}