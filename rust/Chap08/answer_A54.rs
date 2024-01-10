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
const MOD: usize = 1000000007;

fn main() {
    input! {
        q: usize
    }
    let mut map = HashMap::new();
    for i in 0..q {
        input! {
            query: usize,
            name: String,
        }
        if query == 1 {
            input! {
                score: usize,
            }
            map.insert(name, score);
        } else {
            println!("{}", map[&name])
        }
}
}