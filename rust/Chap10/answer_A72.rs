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
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h]
    }
    let mut ans = 0;
    for t in 0..1<<h {
        let mut d = c.clone();
        let mut can_cnt : isize= k as isize;
        for i in 1..=h {
            let wari = 1 << (i-1);
            if (t/wari)%2 == 0 {
                continue;
            }
            can_cnt -= 1;
            for j in 0..w {
                d[i-1][j] = '#';
            }
        }
        if can_cnt >= 0 {
            let mut ans1 = 0;
            let mut column = vec![];
            for jj in 0..w {
                let mut cnt = 0;
                for ii in 0..h {
                    if c[ii][jj] == '.' {
                        cnt += 1;
                    }
                }
                column.push((cnt, jj));
            }
            column.sort();
            column.reverse();
            for jjj in 0..can_cnt {
                let (_, idx) = column[jjj as usize];
                for kk in 0..h {
                    d[kk][idx] = '#';
                }
            }
            for v in 0..h {
                for vv in 0..w {
                    if d[v][vv] == '#' {
                        ans1 += 1;
                    }
                }
            }
            ans = max(ans, ans1);
        }
    }
    println!("{}", ans);
}