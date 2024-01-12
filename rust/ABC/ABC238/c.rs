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
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: isize,
            s: isize,
        }
        let x = s-a*2;
        if 0 <= s-2*a && (x & a==0) {
            println!("Yes");
            continue;
        }
        println!("No");
    }
}