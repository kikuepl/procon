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
const MOD: usize = 1000000007;

fn main() {
    input! {
        query: usize
    }
    let mut set = BTreeSet::new();
    for i in 0..query {
        input! {
            q: usize, 
            x: usize,
        }
        if q == 1{
            set.insert(x);
        } else if q == 2 {
            set.remove(&x);
        } else {
            let ans = if set.range(x..).next() != None {
                println!("{}",set.range(x..).next().unwrap());
            } else {
                println!("{}", -1);
            };
        }
    }
}