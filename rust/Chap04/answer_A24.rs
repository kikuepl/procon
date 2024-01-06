use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;

fn main() {
    let inf = 1000000007;
    input! {
        n: usize,
        a:[usize; n]
    }
    let mut ll = vec![];
    let mut dp = vec![0; n];
    let mut len = 0;
    for i in 0..n {
      let pos = 二分探索2(&ll, &a[i]);
      dp[i]=pos;
      if pos >= len {
        ll.push(a[i]);
        len+=1;
      } else {
        ll[pos] = a[i];
      }
    }
    println!("{}", len);
}

fn bisect_left(v: &Vec<usize>, target: &usize) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = left + (right - left ) / 2;
        if v[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}