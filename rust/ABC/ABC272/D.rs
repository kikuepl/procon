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
        n: isize,
        m: isize,
    }
    let mut moves = vec![];
    for i in 0..=n {
        for j in 0..=n {
            if i*i + j*j == m {
                moves.push((i, j));
            }
        }
    }
    let mut g = vec![vec![INF; n as usize+1]; n as usize+1];
    g[1][1] = 0;
    let mut que = VecDeque::new();
    que.push_back((1, 1));
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for (dx, dy) in &moves {
            for (di, dj) in D {
                let nx = x + *dx as isize * di;
                let ny = y + *dy as isize * dj;
                if 0 < nx && nx <= n && 0 < ny && ny <= n && g[nx as usize][ny as usize] == INF {
                    g[nx as usize][ny as usize] = g[x as usize][y as usize]+1;
                    que.push_back((nx, ny));
                }
            }
        }
    }
    for v in 1..=n {
        for vv in 1..g[v as usize].len() {
            if g[v as usize][vv] == INF {
                print!("{} ", -1);
            }
            else {
                print!("{} ", g[v as usize][vv]);
            }
        }
        println!(" ");
    }
}
