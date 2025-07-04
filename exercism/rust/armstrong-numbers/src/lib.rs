pub fn is_armstrong_number(num: u32) -> bool {
    num.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x.pow(num.to_string().len() as u32))
        .sum::<u32>()
        == num
}
