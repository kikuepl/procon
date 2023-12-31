use proconio::input;
use std::f64::consts::PI;
use std::process::exit;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        d: usize,
        queries: [(usize, usize); d],
    }
    let mut 前 = a.clone();
    let mut 後 = a.clone();
    後.reverse();
    for i in 1..n {
        前[i]=max(前[i],前[i-1]);
        後[i]=max(後[i], 後[i-1]);
    }
    後.reverse();
    for i in &queries {
        let (l,r) = i;
        let ans = if *l as i32 == 1{
            後[*r]
        } else if *r as i32 == n as i32 {
            前[l-2]
        } else {
            max(前[l-2], 後[*r])
        };
        println!("{}", ans);
    }

}