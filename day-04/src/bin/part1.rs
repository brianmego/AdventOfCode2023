mod shared;
use shared::parse_card_set;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let score = sum_card_set(inp);
    println!("{}", score);
}

fn sum_card_set(inp: &str) -> u32 {
    parse_card_set(inp).unwrap().1.iter().map(|card| card.score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sample_set() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = sum_card_set(inp);
        let expected = 13;
        assert_eq!(actual, expected);
    }
}
