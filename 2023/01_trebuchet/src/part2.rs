pub fn get_coordinates(input: &str) -> u32 {
    let mut processed_input = "".to_string();

    for i in 0..input.len() {
        if input[i..].starts_with("one") {
            processed_input.push('1');
        } else if input[i..].starts_with("two") {
            processed_input.push('2');
        } else if input[i..].starts_with("three") {
            processed_input.push('3');
        } else if input[i..].starts_with("four") {
            processed_input.push('4');
        } else if input[i..].starts_with("five") {
            processed_input.push('5');
        } else if input[i..].starts_with("six") {
            processed_input.push('6');
        } else if input[i..].starts_with("seven") {
            processed_input.push('7');
        } else if input[i..].starts_with("eight") {
            processed_input.push('8');
        } else if input[i..].starts_with("nine") {
            processed_input.push('9');
        }
        
        processed_input.push(input.chars().nth(i).unwrap());
    }

    let tmp = processed_input.chars().filter(|&x| x.is_numeric()).collect::<String>();
    return format!("{}{}", tmp.chars().nth(0).unwrap(), tmp.chars().last().unwrap()).parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", "29")]
    #[case("4nineeightseven2", "42")]
    #[case("xtwone3four", "24")]
    #[case("eightwothree", "83")]
    #[case("abcone2threexyz", "13")]
    #[case("zoneight234", "14")]
    #[case("7pqrstsixteen", "76")]
    fn part2_test(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, get_coordinates(&input).to_string());
    }
}
