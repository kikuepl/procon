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
const MOD: isize = 1000000007;
const INF: isize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: isize,
        }
        g[a-1].push((b-1, c));
        g[b-1].push((a-1, c));
    }
    let cur = dijkstra(n, &g);
    for v in &cur {
        if v == &INF  {
            println!("{}", -1);
        } else {
            println!("{}", v);
        }
    }
}
fn dijkstra(n: usize, g: &Vec<Vec<(usize, isize)>>) -> Vec<isize> {
    let mut cur = vec![INF; n];
    cur[0] = 0;
    let mut kakutei = vec![false; n];
    let mut que = BinaryHeap::new();
    que.push((0, 0));
    while !que.is_empty() {
        let (_, pos) = que.pop().unwrap();
        if kakutei[pos] == true {
            continue;
        }
        kakutei[pos] = true;
        for v in 0..g[pos].len() {
            let (nex, scr) = g[pos][v];
            if cur[pos]+scr < cur[nex] {
                cur[nex] = cur[pos]+scr;
                que.push((-cur[nex], nex));
            }
        }
    }
    cur
}