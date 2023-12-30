use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: i32,
        k: i32,
    }
    let mut ans = 0;
    for i in 1..=n {
      for j in 1..=n {
        if 0<k-(i+j) && k-(i+j)<=n {
          ans += 1;
        }
      }
    };
    println!("{}", ans)
}