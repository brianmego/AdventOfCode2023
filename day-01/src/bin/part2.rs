fn main() {
    let inp = include_str!("../data/part1.txt");
    println!("{}", get_calibration_sum(inp));
}

fn get_two_digit_from_line(calibration_val: &str) -> u8 {
    let calibration_val: String = calibration_val.chars().collect::<Vec<char>>()
        .windows(5.min(calibration_val.len()))
        .map(|w| rewrite_words_to_nums(w.iter().collect::<String>()))
        .collect::<Vec<String>>()
        .concat();

    let digits: Vec<u32> = calibration_val
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    (first * 10 + last) as u8
}

fn rewrite_words_to_nums(inp: String) -> String {
    inp.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

fn get_calibration_sum(calibration_script: &str) -> usize {
    calibration_script
        .lines()
        .map(|l| get_two_digit_from_line(l) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine", 29)]
    #[test_case("eightwothree", 83)]
    #[test_case("abcone2threexyz", 13)]
    #[test_case("xtwone3four", 24)]
    #[test_case("4nineeightseven2", 42)]
    #[test_case("zoneight234", 14)]
    #[test_case("7pqrstsixteen", 76)]
    #[test_case("9vft", 99)]
    #[test_case("79", 79)]
    fn test_sample_inputs(inp: &str, exp: u8) {
        let actual = get_two_digit_from_line(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn get_sum_of_input() {
        let inp = include_str!("../data/sample_input2.txt");
        let actual = get_calibration_sum(inp);
        assert_eq!(actual, 281);
    }
}
