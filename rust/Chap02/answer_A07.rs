use proconio::input;
use std::f64::consts::PI;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        d: usize,
        n: usize,
    }
    let mut ans = vec![0; d+1];
    for _ in 0..n {
        input! {
            l: usize,
            r: usize,
        }
        let l = l-1;
        let r = r-1;
        ans[r+1]-=1;
        ans[l]+=1;
    }
    for i in 1..=d {
        ans[i]+=ans[i-1];
    }
    for i in 0..d {
        println!("{}", ans[i]);
    }
}