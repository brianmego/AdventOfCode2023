mod shared;
use itertools::Itertools;
use shared::{parse_set, Hand};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let parsed_inp = parse_set(inp).unwrap().1;
    let actual = total_winnings(parsed_inp);
    println!("{}", actual);
}

fn total_winnings(hands: Vec<Hand>) -> u32 {
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| hand.bet() * (i+1) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_set() {
        let inp = include_str!("../data/sample_input.txt");
        let parsed_inp = parse_set(inp).unwrap().1;
        let actual = total_winnings(parsed_inp);
        let expected = 6440;
        assert_eq!(actual, expected);
    }
}
