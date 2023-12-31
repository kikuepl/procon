use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut b = a.clone();
    b.sort();
    b.dedup();
    let mut ans = vec![0; n];
    for i in 0..n {
        let tmp = 二分探索2(&b, &a[i]);
        ans[i] = tmp;
    }
    for i in &ans {
        print!("{} ", i);
    }
}

fn 二分探索2(v: &Vec<usize>, target: &usize) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = left + (right - left ) / 2;
        if v[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}
