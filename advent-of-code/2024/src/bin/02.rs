use core::num;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let line_vec = line
                .split_whitespace()
                .map(|number| number.parse::<i32>().expect("Should be a number"))
                .collect::<Vec<i32>>();
            line_vec
        })
        .filter(|vector| vector.len() > 2)
        .map(|numbers_vec| {
            let result = if is_report_safe(&numbers_vec) { 1 } else { 0 };
            dbg!(result);
            result
        })
        .sum();
    Some(result)
}

fn is_report_safe(numbers_vec: &[i32]) -> bool {
    // dbg!(&numbers_vec);
    assert!(numbers_vec.len() > 2, " Should be at least three numbers");
    let mut delta = numbers_vec[1] - numbers_vec[0];
    let direction = delta.signum();
    if !(1..=3).contains(&delta.abs()) {
        return false;
    }
    let windows = numbers_vec[1..].windows(2);
    for x in windows {
        if let [l, r] = x {
            delta = r - l;
            // dbg!(l, r, delta);
            if !(1..=3).contains(&delta.abs()) {
                // dbg!("delta too big", delta);
                return false;
            }
            let new_direction = delta.signum();
            if new_direction != direction {
                // dbg!("Direction changed");
                return false;
            }
        } else {
            panic!("Half window");
        }
    }
    true
}
fn is_report_safe_after_removing(numbers_vec: Vec<i32>) -> bool {
    println!("{:?}", &numbers_vec);
    assert!(numbers_vec.len() > 2, " Should be at least three numbers");
    let mut delta = numbers_vec[1] - numbers_vec[0];
    let direction = delta.signum();
    if !(1..=3).contains(&delta.abs()) {
        let mut remove_first = numbers_vec.clone();
        remove_first.remove(0);
        if is_report_safe(&remove_first) {
            println!("Safe by removing first");
            return true;
        }
        return false;
    }

    let windows = numbers_vec[1..].windows(2);
    for (pos, x) in windows.enumerate() {
        if let [l, r] = x {
            delta = r - l;
            let new_direction = delta.signum();
            if (!(1..=3).contains(&delta.abs())) || (new_direction != direction) {
                let mut remove_previous = numbers_vec.clone();
                remove_previous.remove(pos);
                if is_report_safe(&remove_previous) {
                    println!("Safe by removing the level, {}, {}", pos, numbers_vec[pos]);
                    return true;
                }

                let mut remove_current = numbers_vec.clone();
                remove_current.remove(pos + 1);
                if is_report_safe(&remove_current) {
                    println!(
                        "Safe by removing the level, {}, {}",
                        pos + 1,
                        numbers_vec[pos + 1]
                    );
                    return true;
                }

                let mut remove_next = numbers_vec.clone();
                remove_next.remove(pos + 2);
                if is_report_safe(&remove_next) {
                    println!(
                        "Safe by removing the level, {}, {}",
                        pos + 2,
                        numbers_vec[pos + 2]
                    );
                    return true;
                }

                println!("Unsafe regardles of wich level is removed");
                return false;
            }
        } else {
            panic!("Half window");
        }
    }
    println!("Safe without removing any level");
    true
}
pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let line_vec = line
                .split_whitespace()
                .map(|number| number.parse::<i32>().expect("Should be a number"))
                .collect::<Vec<i32>>();
            line_vec
        })
        .filter(|vector| vector.len() > 2)
        .map(|numbers_vec| {
            let result = if is_report_safe_after_removing(numbers_vec) {
                1
            } else {
                0
            };
            result
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }
}
