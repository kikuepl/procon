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
        b: [usize; n],
        c: [usize; n],
        d: [usize; n]
    }
    let mut p =vec![0; n*n];
    let mut q = vec![0; n*n];
    for i in 0..n {
        for j in 0..n {
            p[i*n+j] =a[i]+b[j];
        }
    }

    for i in 0..n {
        for j in 0..n {
            q[i*n+j] =c[i]+d[j];
        }
    }
    q.sort();

    for i in 0..n*n {
        let target = if k < p[i] {
            continue;
        } else {
            k - p[i]
        };
        let index = 二分探索2(&q, &target);
        if index-1< q.len() && q[index-1] == target {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
