use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
use std::usize;
use std::collections::VecDeque;
const MOD: usize = 1000000007;

fn main() {
    input! {
        q: usize
    }
    let mut que = VecDeque::new();
    for i in 0..q {
        input! {
            query: usize,
        }
        if query == 1{
            input! {
                x: String,
            }
            que.push_back(x);
        } else if query == 2 {
            println!("{}", que[0]);
        } else {
            que.pop_front();
        }
    }
}
//https://qiita.com/garkimasera/items/a6df4d1cd99bc5010a5e