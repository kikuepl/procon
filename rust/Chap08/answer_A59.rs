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

struct SegTree {
    size: usize,
    ha: Vec<isize>,
}

impl SegTree {
    fn init(n: usize) -> SegTree {
        let mut size = 1;
        while size <n {
            size *= 2;
        }
        SegTree {
            size, 
            ha: vec![0; size*2]
        }
    }

    fn update(&mut self, mut pos: usize, x:isize) {
        pos += self.size;
        self.ha[pos]=x;
        while pos > 1{
            pos/=2;
            self.ha[pos]=self.ha[pos*2] + self.ha[pos*2+1]
        }
    }

    fn query(&self, l: usize, r: usize, a: usize, b: usize, pos: usize) -> isize {
        if r<=a || b<=l {
            return 0;
        } 
        if l<=a && b<=r {
            return self.ha[pos];
        }
        let mid = (a+b)/2;
        let ansl = self.query(l, r, a, mid, pos*2);
        let ansr = self.query(l, r, mid, b, pos*2+1);
        return ansl+ansr;
    }
}

fn main() {
    input! {
        n: usize, 
        q: usize,
    }
    let mut ki = SegTree::init(n);
    for _ in 0..q {
        input! {
            taipu: usize,
            tmp1: usize,
            tmp2: usize,
        }
        if taipu == 1{
            ki.update(tmp1-1, tmp2 as isize);
        } else {
            let ans = ki.query(tmp1-1, tmp2-1, 0, ki.size, 1);
            println!("{}", ans);
        }
    }
}