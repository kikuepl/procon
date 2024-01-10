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
        m : usize,
        ab: [(usize, usize); m]
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (mut a, mut b) = ab[i];
        a -= 1;
        b -= 1;
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..n {
        print!("{}: {{", i+1);
        for (j, v) in g[i].iter().enumerate() {
            if j > 0 {
                print!{", "};
            }
            print!("{}", v+1);
        }
        println!("}}");
    }
}