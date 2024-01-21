use crate::gamedata::*;

pub fn game_power(input: &str) -> u16 {
    if input.len() == 0 { return 0; }

    let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

    let (_game_number, draws) = game_data(input);

    for i in draws {
        if i.red > min_red { min_red = i.red };
        if i.green > min_green { min_green = i.green };
        if i.blue > min_blue { min_blue = i.blue };
    }

    min_red as u16 * min_green as u16 * min_blue as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_game_power(#[case] input: &str, #[case] result: u16) {
        let game_number = game_power(input);

        assert_eq!(game_number, result);
    }
}

