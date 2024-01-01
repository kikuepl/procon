fn count(s: String, target: String) -> usize {
    let cnt = s.chars().filter(|&c| target.contains(c)).count();
    cnt/target.len()
}