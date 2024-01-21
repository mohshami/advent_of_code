use crate::gamedata::*;

pub fn game_possible(input: &str, red: u8, green: u8, blue: u8) -> u16 {
    if input.len() == 0 { return 0; }

    let (game_number, draws) = game_data(input);

    for i in draws {
        if i.red > red || i.green > green || i.blue > blue { return 0 };
    }

    game_number
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 12, 13, 14, 1)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12, 13, 14, 2)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 12, 13, 14, 0)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 12, 13, 14, 0)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 12, 13, 14, 5)]
    fn test_game_data(#[case] input: &str, #[case] red: u8, #[case] green: u8, #[case] blue: u8, #[case] result: u16) {
        let game_number = game_possible(input, red, green, blue);

        assert_eq!(game_number, result);
    }
}
