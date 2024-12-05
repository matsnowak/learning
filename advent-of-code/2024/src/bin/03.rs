use std::i32;

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").expect("Should compile");
    let muls = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [left, right]) = caps.extract();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .map(|(left, right)| left * right)
        .sum();
    Some(muls)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<mul_left>\d{1,3})\,(?<mul_right>\d{1,3})\)|do\(\)|don't\(\)")
        .expect("Should compile");
    let instructions: Vec<PartTwoInstructions> = re
        .captures_iter(input)
        .map(|caps| {
            let map = caps.get(0).map(|m| m.as_str());
            println!("mapped group {:?}", map);
            match map {
                Some("do()") => PartTwoInstructions::Do,
                Some("don't()") => PartTwoInstructions::Dont,
                Some(_mul) => {
                    let mul_left = caps
                        .name("mul_left")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .expect("should be number");
                    let mul_right = caps
                        .name("mul_right")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .expect("should be number");
                    PartTwoInstructions::Mul(mul_left, mul_right)
                }
                _ => PartTwoInstructions::Empty,
            }
        })
        .inspect(|f| println!("{:?}", f))
        .collect();

    let mut state = ExecutionState {
        multiplication_enabled: true,
    };
    let sum: u32 = instructions
        .into_iter()
        .map(|instruction| instruction.execute(&mut state))
        .sum();

    Some(sum)
}

#[derive(Debug)]
enum PartTwoInstructions {
    Do,
    Dont,
    Mul(u32, u32),
    Empty,
}

impl PartTwoInstructions {
    fn execute(&self, state: &mut ExecutionState) -> u32 {
        match &self {
            PartTwoInstructions::Do => {
                state.multiplication_enabled = true;
                0
            }
            PartTwoInstructions::Dont => {
                state.multiplication_enabled = false;
                0
            }
            PartTwoInstructions::Mul(left, right) => {
                if state.multiplication_enabled {
                    left * right
                } else {
                    0
                }
            }
            PartTwoInstructions::Empty => 0,
        }
    }
}

struct ExecutionState {
    multiplication_enabled: bool,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
