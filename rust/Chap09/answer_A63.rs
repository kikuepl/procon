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
    let mut que = VecDeque::new();
    let mut dist = vec![-1; n];
    que.push_back(0);
    dist[0] = 0;
    bfs(&mut que, &mut dist, g);
    for dd in dist {
        println!("{}", dd);
    }
}

fn bfs(que: &mut VecDeque<usize>, dist: &mut Vec<isize>, graph: Vec<Vec<usize>>) {
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for &v in &graph[pos] {
            if dist[v] == -1 {
                que.push_back(v);
                dist[v] = dist[pos]+1;
            }
        }
    }
}
