use proconio::input;
use std::collections::VecDeque;

const MOD: isize = 1000000007;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut g = vec![vec![]; n+1];
    let mut dist = vec![0; n+1];
    let mut parent = vec![0; n+1];

    for _ in 0..n-1 {
        input! {
            a: usize,
            b: usize,
        }
        g[a].push(b);
        g[b].push(a);
    }

    // 親を設定するためのBFS
    let mut que = VecDeque::new();
    que.push_back(1);
    parent[1] = 1; // 根の親は自身とする
    while let Some(v) = que.pop_front() {
        for &u in &g[v] {
            if u != parent[v] {
                parent[u] = v;
                que.push_back(u);
            }
        }
    }

    // 各クエリの処理
    for _ in 0..q {
        input! {
            p: usize,
            x: usize,
        }
        dist[p] += x;
    }

    // 部分木のカウンターの値を累積
    let mut que = VecDeque::new();
    que.push_back(1);
    while let Some(v) = que.pop_front() {
        for &u in &g[v] {
            if u != parent[v] {
                dist[u] += dist[v];
                que.push_back(u);
            }
        }
    }

    for v in 1..=n {
        print!("{} ", dist[v]);
    }
}
