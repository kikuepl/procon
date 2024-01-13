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
const INF: isize = 1 << 60;

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![0; n];
    let mut b = vec!{0; n};
    for i in 0..n {
        for j in 0..n {
            input! {
                p: usize,
            }
            if p != 0 {
                a[i] = p;
                b[j] = p;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if a[i] > a[j] {
                ans += 1;
            }
            if b[i] > b[j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}