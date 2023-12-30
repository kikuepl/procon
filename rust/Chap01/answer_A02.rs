use proconio::input;
use std::f64::consts::PI;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }
    let ans = if a.contains(&x) {
      "Yes"
    } else {
      "No"
    };
    println!("{}", ans);
}