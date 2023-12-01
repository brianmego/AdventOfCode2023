fn main() {
    let inp = include_str!("../data/part1.txt");
    println!("{}", get_calibration_sum(inp));
}

fn get_two_digit_from_line(calibration_val: &str) -> u8 {
    let digits: Vec<u32> = calibration_val
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(&0);
    (first * 10 + last) as u8
}
fn get_calibration_sum(calibration_script: &str) -> usize {
    calibration_script.lines().map(|l| get_two_digit_from_line(l) as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn test_sample_inputs(inp: &str, exp: u8) {
        let actual = get_two_digit_from_line(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn get_sum_of_input() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = get_calibration_sum(inp);
        assert_eq!(actual, 142);
    }
}
