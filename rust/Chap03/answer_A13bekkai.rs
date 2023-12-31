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

    for ai in 0..n {
        let target = a[ai]+k;
        let x = 二分探索2(&a, &target);
        ans += x-ai-1;
    }

    println!("{}", ans);
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
//binary_search.rs