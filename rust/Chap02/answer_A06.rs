use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i32; n],
    }
    
    for i in 1..n {
      a[i]+=a[i-1];
    }
    
    for i in 0..q {
      input! {
        x: usize,
        y: usize,
      }
      let x = x-1;
      let y = y-1;
      if x==0 {
        println!("{}", a[y]);
      } else {
        println!("{}", a[y]-a[x-1]);
      }
    }
}