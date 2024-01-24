fn is_symbol_adjacent(schematic: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let start_row = if row == 0 { 0 } else { row - 1 };
    let start_col = if col == 0 { 0 } else { col - 1 };

    let mut end_row = row + 1;
    if end_row >= schematic.len() {
        end_row = schematic.len() - 1;
    }
    let mut end_col = col + 1;
    if end_col >= schematic.len() {
        end_col = schematic.len() - 1;
    }

    for i in start_row..=end_row {
        for j in start_col..=end_col {
            if !(schematic[i][j].is_numeric() || schematic[i][j] == '.') {
                return true;
            };
        }
    }

    false
}

#[allow(dead_code)]
pub fn part_numbers(schematic: &Vec<Vec<char>>) -> Vec<u32> {
    let mut ret_val: Vec<u32> = vec![];

    for i in 0..schematic.len() {
        let mut tmp = "".to_string();
        let mut is_part_number = false;
        for j in 0..schematic[i].len() {
            if schematic[i][j].is_numeric() {
                tmp.push(schematic[i][j]);
                if is_symbol_adjacent(schematic, i, j) {
                    is_part_number = true;
                }
            };
            if !schematic[i][j].is_numeric() && tmp.len() > 0 {
                if is_part_number {
                    ret_val.push(tmp.to_string().parse::<u32>().unwrap());
                }
                is_part_number = false;
                tmp = "".to_string();
            }
        }
        // If we were still processing a number at the end of the line
        if tmp.len() > 0 {
            if is_part_number {
                ret_val.push(tmp.to_string().parse::<u32>().unwrap());
            }
        }
    }

    ret_val
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
    #[case(0, 0, false)]
    #[case(0, 2, true)]
    #[case(2, 6, true)]
    fn test_is_symbol_adjacent(#[case] row: usize, #[case] col: usize, #[case] result: bool) {
        let schematic = MY_SCHEMATIC
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .filter(|line| line.len() > 0)
            .collect::<Vec<Vec<char>>>();

        assert_eq!(result, is_symbol_adjacent(&schematic, row, col));
    }

    #[test]
    fn test_part_numbers() {
        let schematic = MY_SCHEMATIC
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .filter(|line| line.len() > 0)
            .collect::<Vec<Vec<char>>>();

        assert_eq!(
            vec![467, 35, 633, 617, 592, 755, 664, 598],
            part_numbers(&schematic)
        );
    }
}
