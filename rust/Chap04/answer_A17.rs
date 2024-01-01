use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let mut dp = vec![0; n];
    dp[1]=a[0];
    for i in 2..n {
        dp[i]=min(dp[i-1]+a[i-1], dp[i-2]+b[i-2]);
    }
    let mut pos = n-1;
    let mut ans = vec![];
    loop {
        ans.push(pos+1);
        if pos == 0 {
            break;
        }
        if pos>=2 && dp[pos]==dp[pos-2]+b[pos-2] {
            pos -= 2;
        } else {
            pos -= 1;
        }
    }
    ans.reverse();
    println!("{}", ans.len());
    for v in &ans {
        print!("{} ", v)
    }
}
