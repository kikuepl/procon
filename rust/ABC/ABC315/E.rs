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
const INF: usize = 1 << 60;

const D: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
fn main() {
    input! {
        n: usize,
    }
    let mut g = vec![vec![]; n];
    for i in 0..n {
        input! {
            c: usize,
        }
        for _ in 0..c {
            input! {
                p: usize
            }
            g[i].push(p-1);
        }
    }
    let mut visited = vec![false; n];
    let mut ans = vec![];
    dfs(0, &g, &mut visited, &mut ans);
    for v in ans {
        print!("{} ", v);
    }
}

fn dfs(position: usize, graph: &[Vec<usize>], visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    visited[position] = true;
    for &vv in &graph[position] {
        if visited[vv]==false {
            dfs(vv, graph, visited, ans);
        }
    }
    if position != 0 {
        ans.push(position+1);

    }
}