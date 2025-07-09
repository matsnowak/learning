pub fn square(s: u32) -> u64 {
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (0..=63).map(|x| 2u64.pow(x)).sum()
}
