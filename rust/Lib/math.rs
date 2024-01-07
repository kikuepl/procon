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