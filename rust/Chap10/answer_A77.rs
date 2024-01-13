use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
use std::usize;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeSet;
const MOD: isize = 1000000007;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n]
    }
    let ans = bisect_left(&a, l, k);
    println!("{}", ans);
}

fn check(a: Vec<usize>, check_num: usize, l: usize, k: usize) -> bool {
    let mut cnt = 0;
    let mut last = 0;
    let m = a.len();
    for i in 0..m {
        if check_num <= a[i]-last && check_num <= l - a[i] {
            cnt += 1;
            last = a[i];
        }
    }
    if cnt >= k {
        return true;
    } else {
        return false;
    }
}

fn bisect_left(v: &Vec<usize>, l: usize, k: usize) -> usize {
    let mut left = 0;
    let mut right = INF;
    while left < right {
        let mid = (left + right + 1) / 2;
        if check(v.to_vec(), mid, l, k) {
            left = mid ;
        } else {
            right = mid-1;
        }
    }
    left
}