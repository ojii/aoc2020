use std::str::FromStr;

pub fn str_to_ints<F: FromStr>(s: &str) -> Vec<F> {
    s.lines().flat_map(|l| l.parse::<F>()).collect()
}
