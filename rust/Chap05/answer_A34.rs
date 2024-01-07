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
        x: usize,
        y: usize, 
        a: [usize; n]
    }
    let mut grundy = vec![0; 100000+1];
    for i in 0..=100000 {
        let mut trans = vec![false; 3];
        if x<=i {
            trans[grundy[i-x]] = true;
        }
        if y<=i  {
            trans[grundy[i-y]] = true;
        }
        if trans[0] == false {
            grundy[i] = 0
        } else if trans[1] == false {
            grundy[i] = 1
        } else {
            grundy[i] = 2
        };
    }
    let mut Gmi = 0;
    for i in 0..n {
        Gmi ^= grundy[a[i]];
    }
    if Gmi == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
