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
        a: [usize; n]
    }
    let mut cnt = [0; 101];
    for ai in &a {
        cnt[*ai]+=1;
    }
    let mut ans = 0;
    for cc in &cnt {
        if cc < &3 {
            continue;
        } else {
            ans += combination(&cc, &3, &MOD);
        }
    }
    println!("{}", ans);
}

//繰り返し二乗法
fn pow(a: &usize, b: &usize, m: &usize) -> usize {
    let mut p: usize = *a;
    let mut ans: usize = 1;
    for i  in 0..30 {
        let chk:usize = 1 << i;
        let check = b/chk;
        if check%2 == 1 {
            ans *= p;
            ans %= m;
        }
        p *= p;
        p %= m;
    }
    return ans;
}

//nCrを求める
fn combination(n: &usize, r: &usize, m: &usize) -> usize {
    let mut a: usize = 1;
    let mut b: usize = 1;
    for i in 1..=*n {
        a *= i;
        a %= m;
    }
    for j in 1..=*r {
        b *= j;
        b %= m;
    }
    for k in 1..=*n-r {
        b *= k;
        b %= m;
    }
    let m2:usize = m-2;
    return (a * pow(&b, &m2, &m))%m;
}