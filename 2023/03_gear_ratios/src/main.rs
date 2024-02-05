use std::fs;

pub mod part1;
pub mod part2;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let schematic = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<Vec<char>>>();

    let part_numbers = part1::part_numbers(&schematic);
    let ratios = part2::get_ratios(&schematic);

    println!("{}", part_numbers.iter().sum::<u32>());
    println!("{}", ratios.iter().sum::<u32>());
}
