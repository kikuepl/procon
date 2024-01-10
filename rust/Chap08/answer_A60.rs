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
const MOD: usize = 100_000_0007;

fn main() {
    input! {
        n: usize, 
        a: [usize; n]
    }
    let mut s = Vec::new();
    for i in 0..n {
        if !s.is_empty() {
            let (x, y) = s.last().unwrap();
            println!("{:?}", y+1);
        } else {
            println!("{}", -1);
        }
        s.push((a[i], i));
        while i < n-1 && !s.is_empty() && s.last().unwrap().0 < a[i+1] {
            s.pop();
        }
    }
}