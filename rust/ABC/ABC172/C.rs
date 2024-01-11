use proconio::input;
use proconio::marker::{Bytes, Chars};
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
const INF: isize = 1 << 60;

const D: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    let mut ans = 0;
    for i in 1..n {
        a[i]+=a[i-1];
    }
    for j in 1..m {
        b[j]+=b[j-1];
    }
    a.insert(0, 0);
    b.insert(0,0);
    for i in 0..=n {
        if a[i] > k {
            break;
        } 
        let j = bisect_right(&b,&(k-a[i]))-1;
        ans= max(ans, i+j);
    }
    println!("{}", ans);
}

fn bisect_right(v: &Vec<usize>, target: &usize) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = (left + right ) / 2;
        if v[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}