fn 二分探索(v: &Vec<usize>, x: &usize) -> Option<usize> {
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