/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut no_spaces: String = code.to_owned();
    no_spaces.retain(|c| !c.is_whitespace());

    if no_spaces.len() <= 1 {
        return false;
    }

    if no_spaces.find(|c: char| !c.is_ascii_digit()).is_some() {
        return false;
    }

    let sum: u32 = no_spaces
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, n)| {
            if i % 2 == 1 {
                if n * 2 > 9 { n * 2 - 9 } else { n * 2 }
            } else {
                n
            }
        })
        .sum();
    dbg!(sum.rem_euclid(10) == 0)
}
