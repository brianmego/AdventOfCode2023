mod shared;
use core::cmp::Ordering;
use itertools::Itertools;
use nom::{
    character::complete::{char, digit1, newline, one_of},
    multi::{many1, many_m_n},
    sequence::{separated_pair, terminated},
    IResult,
};

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
        .map(|(i, hand)| hand.bet() * (i + 1) as u32)
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn next(&self) -> Option<Card> {
        match self {
            Card::Joker => Some(Card::Two),
            Card::Two => Some(Card::Three),
            Card::Three => Some(Card::Four),
            Card::Four => Some(Card::Five),
            Card::Five => Some(Card::Six),
            Card::Six => Some(Card::Seven),
            Card::Seven => Some(Card::Eight),
            Card::Eight => Some(Card::Nine),
            Card::Nine => Some(Card::Ten),
            Card::Ten => Some(Card::Queen),
            Card::Queen => Some(Card::King),
            Card::King => Some(Card::Ace),
            Card::Ace => None,
        }
    }
}

#[derive(Debug)]
struct NotACardError;
impl TryFrom<&char> for Card {
    type Error = NotACardError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Joker),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(NotACardError),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl From<&Vec<Card>> for HandType {
    fn from(value: &Vec<Card>) -> Self {
        match value.contains(&Card::Joker) {
            true => {
                let mut new_value = Card::Joker.next();
                let mut new_hands: Vec<HandType> = vec![];
                while let Some(new_card) = new_value {
                    let new_hand: Vec<Card> = value
                        .iter()
                        .map(|card| match card {
                            &Card::Joker => new_card,
                            r => *r,
                        })
                        .collect();
                    new_hands.push(HandType::from(&new_hand));
                    new_value = new_card.next();
                }
                *new_hands.iter().max().unwrap()
            }
            false => HandType::get_specific_hand_type(value),
        }
    }
}

impl HandType {
    fn get_specific_hand_type(value: &Vec<Card>) -> Self {
        let buckets: Vec<(usize, &Card)> = value
            .iter()
            .sorted()
            .dedup_with_count()
            .sorted_by_key(|x| x.0)
            .rev()
            .collect();
        if buckets[0].0 == 5 {
            return HandType::FiveOfAKind;
        } else if buckets[0].0 == 4 {
            return HandType::FourOfAKind;
        } else if buckets[0].0 == 3 {
            if buckets[1].0 == 2 {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        } else if buckets[0].0 == 2 {
            if buckets[1].0 == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }
        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(cards: &str, bet: u32) -> Hand {
        let cards = parse_cards(cards).unwrap().1;
        let hand_type = HandType::from(&cards);
        Hand {
            cards,
            hand_type,
            bet,
        }
    }
    fn new_from_cards(cards: Vec<Card>, bet: u32) -> Hand {
        let hand_type = HandType::from(&cards);
        Hand {
            cards,
            hand_type,
            bet,
        }
    }
    pub fn bet(&self) -> u32 {
        self.bet
    }
}

pub fn parse_set(inp: &str) -> IResult<&str, Vec<Hand>> {
    many1(terminated(parse_hand, newline))(inp)
}

fn parse_hand(inp: &str) -> IResult<&str, Hand> {
    let (inp, (cards, bet)): (&str, (Vec<Card>, &str)) =
        separated_pair(parse_cards, char(' '), digit1)(inp)?;
    let bet = bet.parse::<u32>().unwrap();
    Ok((inp, Hand::new_from_cards(cards, bet)))
}
fn parse_cards(inp: &str) -> IResult<&str, Vec<Card>> {
    let (inp, hand) = many_m_n(5, 5, one_of("AKQJT98765432"))(inp)?;
    let cards: Vec<Card> = hand
        .iter()
        .map(|card| Card::try_from(card).unwrap())
        .collect();
    Ok((inp, cards))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("32T3K 765", ("", Hand::new("32T3K", 765)); "32T3K 765")]
    fn test_parse_hand(inp: &str, exp: (&str, Hand)) {
        let actual = parse_hand(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test_case(Hand::new("32T3K", 765), HandType::OnePair; "32T3K")]
    #[test_case(Hand::new("T55J5", 684), HandType::FourOfAKind; "T55J5")]
    #[test_case(Hand::new("KK677", 28), HandType::TwoPair; "KK677")]
    #[test_case(Hand::new("KTJJT", 220), HandType::FourOfAKind; "KTJJT")]
    #[test_case(Hand::new("QQQJA", 483), HandType::FourOfAKind; "QQQJA")]
    fn test_new_hand(inp: Hand, exp: HandType) {
        let actual = inp.hand_type;
        let expected = exp;
        assert_eq!(actual, expected);
    }

    #[test_case((Hand::new("AAAA5", 765), Hand::new("AAA55", 756)), true)]
    #[test_case((Hand::new("KTJJT", 756), Hand::new("KK677", 765)), true)]
    fn test_hand_sorting(inp: (Hand, Hand), exp: bool) {
        let (hand_one, hand_two) = inp;
        let actual = hand_one > hand_two;
        assert_eq!(actual, exp);
    }

    #[test_case((Card::King, Card::King), Ordering::Equal)]
    #[test_case((Card::Ace, Card::King), Ordering::Greater)]
    #[test_case((Card::Six, Card::King), Ordering::Less)]
    fn test_card_sorting(inp: (Card, Card), exp: Ordering) {
        let (card_one, card_two) = inp;
        let actual = card_one.partial_cmp(&card_two);
        assert_eq!(actual, Some(exp));
    }

    #[test]
    fn test_parse_set() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = parse_set(inp);
        assert!(actual.is_ok());
        let hands = actual.unwrap().1;
        assert_eq!(hands.len(), 5);
    }

    #[test]
    fn test_sample_set() {
        let inp = include_str!("../data/sample_input.txt");
        let parsed_inp = parse_set(inp).unwrap().1;
        let actual = total_winnings(parsed_inp);
        let expected = 5905;
        assert_eq!(actual, expected);
    }
}
