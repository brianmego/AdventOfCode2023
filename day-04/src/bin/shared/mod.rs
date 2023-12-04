use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::{many0, many1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    id: u32,
    winning_set: Vec<u32>,
    your_set: Vec<u32>,
    has_calculated_duplicates: bool,
}

impl Card {
    fn new(id: u32, winning_set: Vec<u32>, your_set: Vec<u32>) -> Self {
        Self {
            id,
            winning_set,
            your_set,
            has_calculated_duplicates: false,
        }
    }
    pub fn score(&self) -> u32 {
        let winning_numbers = self
            .your_set
            .iter()
            .filter(|n| self.winning_set.contains(n))
            .count();
        2_u32.pow(winning_numbers as u32) / 2
    }

    pub fn winning_numbers_count(&self) -> usize {
        self.your_set
            .iter()
            .filter(|n| self.winning_set.contains(n))
            .count()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn already_calculated_duplicates(&self) -> bool {
        self.has_calculated_duplicates
    }

    pub fn set_has_calculated_duplicates(&mut self) {
        self.has_calculated_duplicates = true;
    }
}

pub fn parse_card_set(inp: &str) -> IResult<&str, Vec<Card>> {
    many1(terminated(parse_card, newline))(inp)
}

/// parses 'Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53'
fn parse_card(inp: &str) -> IResult<&str, Card> {
    let (inp, (card_id, (winning_set, your_set))) = tuple((
        delimited(terminated(tag("Card"), many1(tag(" "))), digit1, tag(":")),
        separated_pair(parse_number_set, tag("|"), parse_number_set),
    ))(inp)?;
    let card_id = card_id.parse::<u32>().unwrap();
    Ok((inp, Card::new(card_id, winning_set, your_set)))
}

/// parses '41 48 83 86 17 | 83 86  6 31 17  9 48 53'
fn parse_number_set(inp: &str) -> IResult<&str, Vec<u32>> {
    let (inp, number_set): (&str, Vec<&str>) =
        many1(delimited(many0(tag(" ")), digit1, many0(tag(" "))))(inp)?;
    let number_set = number_set
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    Ok((inp, number_set))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        (
            "",
            Card::new(
                1,
                vec![41, 48, 83, 86, 17],
                vec![83, 86, 6, 31, 17, 9, 48, 53]
            )
        );
        "Card 1"
    )]
    #[test_case(
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        (
            "",
            Card::new(
                3,
                vec![1, 21, 53, 59, 44],
                vec![69, 82, 63, 72, 16, 21, 14, 1]
            )
        );
        "Card 3"
    )]
    fn test_parse_card(inp: &str, exp: (&str, Card)) {
        let actual = parse_card(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test_case("41 48  6  1 23", ("", vec![41, 48, 6, 1, 23]); "set1")]
    fn test_number_set(inp: &str, exp: (&str, Vec<u32>)) {
        let actual = parse_number_set(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test_case(Card::new(1, vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]), 2; "card2")]
    #[test_case(Card::new(4, vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]), 1; "card4")]
    #[test_case(Card::new(5, vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]), 0; "card5")]
    fn get_score(inp: Card, exp: u32) {
        let actual = inp.score();
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_set() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual = parse_card_set(inp);
        assert!(actual.is_ok());
        let cards = actual.unwrap().1;
        assert_eq!(cards.len(), 6)
    }
}
