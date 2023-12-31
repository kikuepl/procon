use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let ans = 二分探索(&a, &x).unwrap();
    println!("{}", ans+1);

}

fn 二分探索(v: &Vec<usize>, x: &usize) -> Option<usize> {
    let mut left = 0;
    let mut right = v.len()-1;
    while left <= right {
        let mid = (left + right) / 2;
        if *x < v[mid] {
            right = mid - 1;
        }
        if *x == v[mid] {
            return Some(mid);
        }
        if *x > v[mid] {
            left = mid + 1;
        };
    }
    return None;
}

//binary_search.rs