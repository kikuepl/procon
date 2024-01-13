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
const INF: usize = 1 << 60;

const D: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n]
    }
    a.sort();
    b.sort();
    b.reverse();
    let mut ans = 0;
    for i in 0..n {
        ans += a[i]*b[i];
    }
    println!("{}", ans);
}