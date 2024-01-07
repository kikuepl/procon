use proconio::input;
use proconio::marker::Bytes;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;
use std::sync::PoisonError;
const inf: usize = 1000000007;

fn main() {
    input! {
        q: usize,
        x: [usize; q]
    }
    for i in &x {
        let mut prime_check = true;
        let n = (*i as f64).sqrt() as usize;
        for j in 2..=n {
            if i%j==0 {
                prime_check = false;
            }
        } 
        if prime_check == true {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}