use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        h: usize.
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut Z = vec![vec![0; w+2]; h+2];
    for abcdi in abcd {
        let (ai, bi, ci, di) = abcdi;
        Z[ai][bi]+=1;
        Z[ci][di]+=1;
        Z[ai][di+1]-=1;
        Z[ci+1][bi]-=1;
    }
    let mut S = vec![vec![0; w+2]; h+2];
    for i in 0..=h {
        for j in 0..=w {
            S[i+1][j+1]=Z[i][j]+S[i+1][j]+S[i][j+1]-S[i][j];
        }
    }
    for i in &S{
        println!("{}", &i)
    }
}