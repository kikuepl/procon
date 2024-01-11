//同じやつを探す
fn 二分探索1(v: &Vec<usize>, x: &usize) -> Option<usize> {
    let mut left = 0;
    let mut right = v.len()-1;
    while left <= right {
        let mid = (left + right) / 2;
        if *x < v[mid] {
            right = mid - 1;
        }
        if *x == v[mid] {
            return mid;
        }
        if *x > v[mid] {
            left = mid + 1;
        }
    }
    return -1;
}
//インデックスを特定する
//こっちの方が使いやすいかも
fn bisect_right(v: &Vec<usize>, target: &usize) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = (left + right ) / 2;
        if v[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn bisect_left(v: &Vec<usize>, target: &usize) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = (left + right) / 2;
        if v[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}
