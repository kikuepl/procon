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
        n: usize,
    }
    let mut mae_B = 0;
    let mut usiro_A = 0;
    let mut mae_and_usiro = 0;
    let mut count_ab = 0;
    for _ in 0..n {
        input! {
            s: Chars,
        }
        if s[0] == 'B' && s[s.len()-1] == 'A' {
            mae_and_usiro += 1;
        } else if s[0]=='B' {
            mae_B += 1;
        } else if s[s.len()-1] == 'A' {
            usiro_A += 1;
        }
        count_ab += count(s.iter(), 'A', 'B');
    }
    count_ab += min(usiro_A, mae_B);
    if mae_and_usiro > 0 {
        count_ab += mae_and_usiro -1;
        if mae_B > 0 || usiro_A > 0 {
            count_ab += 1;
        }
    }
    println!("{}", count_ab);
}

fn count(s: std::slice::Iter<char>, a: char, b: char) -> usize {
    let mut cnt = 0;
    let mut prev = ' ';
    for &c in s {
        if prev == a && c == b {
            cnt += 1;
        }
        prev = c;
    }
    cnt
}