use proconio::input;
use proconio::marker::Bytes;
use proconio::marker::Chars;
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
const UNF: usize = 1 << 60;
const INF: isize = 1 << 60;

const D: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

fn division(n: usize) -> Vec<usize> {
    let mut divs = vec![];
    let mut large_divs = vec![];
    
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                large_divs.push(n / i);
            }
        }
        i += 1;
    }

    divs.extend(large_divs.into_iter().rev());
    divs
}

fn num(n: usize) -> usize {
    let mut tmp = 10_000_000_000;
    let mut cnt = 0;
    while n < tmp {
        cnt += 1;
        tmp /= 10;
    }

    return 10-cnt+1;
}

fn main() {
    input! {
        n: usize
    }
    let ans = division(n);
    let mut answer = UNF;
    for i in 0..ans.len() {
        answer =min(answer, max(num(ans[i]), num(ans[ans.len()-i-1])));
    }
    println!("{answer}");
}

