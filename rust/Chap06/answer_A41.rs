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
        s: Bytes,
    }
    let mut ans = false;
    for i in 0..n-2 {
        if s[i]==b'R' && s[i+1]==b'R' && s[i+2]==b'R' {
            ans=true;
        }
        if s[i]==b'B' && s[i+1]==b'B' && s[i+2]==b'B' {
            ans=true;
        }
    }
    if ans==true {
        println!("Yes");
    } else  {
        println!("No");
    }
}