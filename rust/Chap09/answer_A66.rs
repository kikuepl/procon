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
const INF: usize = 1 << 60;

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
        q: usize,
        query: [(usize, usize, usize); q]
    }
    let mut UF = UnionFind::init(n);
    for (t, u, v) in query {
        if t == 1 {
            UF.unite(u-1, v-1);
        } else if t == 2 {
            if UF.same(u-1, v-1) == true {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}   