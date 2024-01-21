use std::fs;

pub mod gamedata;
pub mod part1;
pub mod part2;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let part1_output = contents
        .lines()
        .map(|line| part1::game_possible(line, 12, 13, 14))
        .sum::<u16>();

    let part2_output = contents
        .lines()
        .map(|line| part2::game_power(line) as u32)
        .sum::<u32>();

    println!("{part1_output}");
    println!("{part2_output}");
}
