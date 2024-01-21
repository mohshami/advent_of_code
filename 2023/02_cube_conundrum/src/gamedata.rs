pub struct Draw {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

const DEFAULT_DRAW: Draw = Draw {
    red: 0,
    green: 0,
    blue: 0,
};

fn get_draws(input: &str) -> Vec<Draw> {
    let mut result: Vec<Draw> = vec![];

    for i in input.split(";") {
        let mut current_draw = Draw { ..DEFAULT_DRAW };
        for j in i.split(",") {
            let parts = j.trim().split(" ").collect::<Vec<&str>>();
            match parts[1] {
                "red" => {
                    current_draw.red = parts[0].parse::<u8>().unwrap();
                }
                "green" => {
                    current_draw.green = parts[0].parse::<u8>().unwrap();
                }
                "blue" => {
                    current_draw.blue = parts[0].parse::<u8>().unwrap();
                }
                _ => {}
            }
        }
        result.push(current_draw);
    }

    result
}

pub fn game_data(input: &str) -> (u16, Vec<Draw>) {
    let parts = input.split(":").collect::<Vec<&str>>();

    let game_number = parts[0].split(" ").last().unwrap().parse::<u16>().unwrap();
    let draws = get_draws(parts[1].trim());

    (game_number, draws)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", (1, vec![Draw {red: 4, green: 0, blue: 3}, Draw {red: 1, green: 2, blue: 6}, Draw {red: 0, green: 2, blue: 0}]))]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", (2, vec![Draw {red: 0, green: 2, blue: 1}, Draw {red: 1, green: 3, blue: 4}, Draw {red: 0, green: 1, blue: 1}]))]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", (3, vec![Draw {red: 20, green: 8, blue: 6}, Draw {red: 4, green: 13, blue: 5}, Draw {red: 1, green: 5, blue: 0}]))]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", (4, vec![Draw {red: 3, green: 1, blue: 6}, Draw {red: 6, green: 3, blue: 0}, Draw {red: 14, green: 3, blue: 15}]))]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", (5, vec![Draw {red: 6, green: 3, blue: 1}, Draw {red: 1, green: 2, blue: 2}]))]
    fn test_game_data(#[case] input: &str, #[case] result: (u16, Vec<Draw>)) {
        let (game_number, draws) = game_data(input);

        assert_eq!(game_number, result.0);

        for (result_i, draw) in result.1.iter().zip(draws.iter()) {
            assert_eq!(draw.red, result_i.red);
            assert_eq!(draw.green, result_i.green);
            assert_eq!(draw.blue, result_i.blue);
        }
    }
}
