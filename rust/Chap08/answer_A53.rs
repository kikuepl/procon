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
const MOD: usize = 1000000007;

fn main() {
    input! {
        q: usize
    }
    let mut heap = BinaryHeap::new();
    for i in 0..q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                x: isize,
            }
            heap.push(-x);
        } else if query == 2 {
            println!("{}", -1*heap.peek().unwrap());
        } else {
            heap.pop();
        }
    }
}