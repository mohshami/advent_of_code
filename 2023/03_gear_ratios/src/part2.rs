fn get_number(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> u16 {
    if !schematic[i][j].is_numeric() {
        return 0;
    }

    let mut result: u16 = 0;
    let row = &schematic[i];
    let mut cj = j;

    while cj > 0 && row[cj].is_numeric() {
        cj -= 1;
    }
    if !row[cj].is_numeric() {
        cj += 1;
    }

    for c in &row[cj..] {
        if !c.is_numeric() {
            break;
        }

        result *= 10;
        result += c.to_digit(10).unwrap() as u16;
    }
    result
}

fn get_ratio(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let start_row: usize = if i == 0 { 0 } else { i - 1 };
    let start_col: usize = if j == 0 { 0 } else { j - 1 };

    let end_row: usize = if i == schematic.len() { i } else { i + 1 };
    let end_col: usize = if j == schematic[i].len() { j } else { j + 1 };

    let mut result: Vec<u16> = vec![];

    let mut tmp: u16;

    for ci in start_row..=end_row {
        for cj in start_col..=end_col {
            tmp = get_number(schematic, ci, cj);
            if (cj == start_col || !schematic[ci][cj - 1].is_digit(10)) && tmp > 0 {
                result.push(tmp);
            }
        }
    }

    if result.len() != 2 {
        return 0;
    }

    result[0] as u32 * result[1] as u32
}

pub fn get_ratios(schematic: &Vec<Vec<char>>) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            if schematic[i][j] == '*' {
                result.push(get_ratio(schematic, i, j));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const MY_SCHEMATIC: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[rstest]
    #[case(0, 2, 467)]
    #[case(2, 3, 35)]
    fn test_get_number(#[case] row: usize, #[case] col: usize, #[case] result: u16) {
        let schematic = MY_SCHEMATIC
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .filter(|line| line.len() > 0)
            .collect::<Vec<Vec<char>>>();
        assert_eq!(get_number(&schematic, row, col), result);
    }

    #[rstest]
    #[case(1, 3, 16345)]
    #[case(8, 5, 451490)]
    fn test_get_ratio(#[case] row: usize, #[case] col: usize, #[case] result: u32) {
        let schematic = MY_SCHEMATIC
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .filter(|line| line.len() > 0)
            .collect::<Vec<Vec<char>>>();
        assert_eq!(get_ratio(&schematic, row, col), result);
    }

    #[rstest]
    fn test_get_ratios () {
        let schematic = MY_SCHEMATIC
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .filter(|line| line.len() > 0)
            .collect::<Vec<Vec<char>>>();
        let result = get_ratios(&schematic);

        assert_eq!(result.iter().sum::<u32>(), 467835);
    }
}
