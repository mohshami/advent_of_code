pub fn get_coordinates(input: &str) -> u32 {
    let tmp = input.chars().filter(|&x| x.is_numeric()).collect::<String>();
    return format!("{}{}", tmp.chars().nth(0).unwrap(), tmp.chars().last().unwrap()).parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", "12")]
    #[case("pqr3stu8vwx", "38")]
    #[case("a1b2c3d4e5f", "15")]
    #[case("treb7uchet", "77")]
    fn part1_test(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, get_coordinates(input).to_string());
    }
}
