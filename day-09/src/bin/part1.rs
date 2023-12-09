mod shared;
use shared::{get_next_in_pattern, parse_line_as_vec};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let sum_of_next_lines: i32 = inp
        .lines()
        .map(|line| get_next_in_pattern(parse_line_as_vec(line)))
        .sum();
    println!("{}", sum_of_next_lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let inp = include_str!("../data/sample_input.txt");
        let sum_of_next_lines: i32 = inp
            .lines()
            .map(|line| get_next_in_pattern(parse_line_as_vec(line)))
            .sum();
        assert_eq!(sum_of_next_lines, 114);
    }
}
