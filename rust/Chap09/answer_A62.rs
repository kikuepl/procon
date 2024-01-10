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
        n: usize,
        m: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        g[a-1].push(b-1);
        g[b-1].push(a-1);
    }
    let mut visited = vec![false; n];
    dfs(0, &g, &mut visited);
    for v in visited {
        if v == false {
            println!("The graph is not connected.");
            exit(0);
        }
    }
    println!("The graph is connected.");
}
fn dfs(position: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[position] = true;
    for &vv in &graph[position] {
        if visited[vv]==false {
            dfs(vv, graph, visited);
        }
    }
}
