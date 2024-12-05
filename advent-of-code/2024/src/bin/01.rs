use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut values = line.split_whitespace();
        let left_value: i32 = values
            .next()
            .expect("No left value")
            .parse()
            .expect("Should be number");
        let right_value: i32 = values
            .next()
            .expect("No left value")
            .parse()
            .expect("Should be number");

        left.push(left_value);
        right.push(right_value);
    }
    assert!(!left.is_empty());
    assert!(!right.is_empty());
    assert_eq!(left.len(), right.len());

    left.sort();
    right.sort();

    let result: u32 = left
        .iter()
        .zip(right)
        .map(|(left, right)| (left - right).unsigned_abs())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = Vec::new();
    // let mut right: Vec<i32> = Vec::new();
    let mut right_freq: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut values = line.split_whitespace();
        let left_value: i32 = values
            .next()
            .expect("No left value")
            .parse()
            .expect("Should be number");
        left.push(left_value);
        let right_value: i32 = values
            .next()
            .expect("No left value")
            .parse()
            .expect("Should be number");

        right_freq
            .entry(right_value)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    assert!(!left.is_empty());
    assert!(!right_freq.is_empty());

    let result: i32 = left
        .into_iter()
        .map(|left_val| left_val * right_freq.get(&left_val).unwrap_or(&0))
        .sum();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
