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

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>
}

impl UnionFind {
    fn init(n: usize) -> UnionFind {
        let mut par = vec![INF; n];
        let mut siz = vec![1; n];
        UnionFind {
            par,
            siz,
        }
    }

    fn root(&mut self, mut x: usize) -> usize {
        if self.par[x] == INF {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    fn unite(&mut self, u: usize, v: usize) {
        let rootU = self.root(u);
        let rootV = self.root(v);
        if rootU != rootV {
            if self.siz[rootU] < self.siz[rootV] {
                self.par[rootU] = rootV;
                self.siz[rootV] += self.siz[rootU]
            } else {
                self.par[rootV] = rootU;
                self.siz[rootU] += self.siz[rootV];
            }
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
    if self.root(u) == self.root(v) {
            return true;
        } else {
            return false;
        };
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut uf = UnionFind::init(n);
    let mut a = vec![0; m];
    let mut b = vec![0; m];
    let mut c = vec![];
    for i in 0..m {
        input! {
            x: usize,
            y: usize,
            z: usize,
        }
        a[i] = x-1;
        b[i] = y-1;
        c.push((z, i));
    }
    c.sort();
    let mut ans = 0;
    for (scr, idx) in c {
        if !(uf.same(a[idx], b[idx])) {
            uf.unite(a[idx], b[idx]);
            ans += scr;
        }
    }
    println!("{}", ans);
}