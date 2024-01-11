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

fn main() {
    input! {
        n: usize,
        c: isize,
        abc: [(isize, isize, isize); n]
    }
    let mut eve = vec![];
    for (a, b, ci) in abc {
        eve.push((a, ci));
        eve.push((b+1, -ci));
    }
    eve.sort();
    let mut fee: isize = 0;
    let mut ans = 0;
    let mut pre = 0;
    for (x, y) in eve {
        ans += min(c,fee)*(x-pre);
        pre = x;
        fee += y;
    }
    println!("{}", ans);
}
