use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize, 
        ab: [(usize, usize); n]
    }
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for i in 0..n {
        a[i] = ab[i].0;
        b[i] = ab[i].1;
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            let ai = a[i];
            let bi = b[j];
            let mut cnt = 0;
            for h in 0..n {
                if ai<=a[h]&&a[h]<=ai+k && bi<=b[h]&&b[h]<=bi+k {
                    cnt+=1;
                }
            }
            ans = max(ans, cnt);
        }
    }
    println!("{}", ans);
}