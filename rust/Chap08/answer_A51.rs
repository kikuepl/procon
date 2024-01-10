use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
use std::usize;
const MOD: usize = 1000000007;

fn main() {
    input! {
        q: usize
    }
    let mut stack = vec![];
    for i in 0..q {
        input! {
            query: usize
        }
        if query==1 {
            input! {
                name: String,
            }
            stack.push(name);
        } else if query== 2{
            let top = stack.last().unwrap();
            println!("{}", top);
        } else {
            stack.pop();
        }
    }
}
//https://qiita.com/garkimasera/items/a6df4d1cd99bc5010a5e