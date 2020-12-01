pub fn str_to_ints(s: &str) -> Vec<i32> {
    s.lines().flat_map(|l| l.parse::<i32>()).collect()
}