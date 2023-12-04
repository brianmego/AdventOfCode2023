mod shared;
use shared::{parse_card_set, Card};

fn main() {}

fn add_all_duplicates(inp: Vec<Card>) -> Vec<Card> {
    let new_inp: Vec<Card> = inp
        .into_iter()
        .filter(|card| !card.already_calculated_duplicates())
        .collect();
    let new_inp = calculate_duplicates(new_inp, &inp);
    // match new_inp.is_empty() {
    //     true => new_inp,
    //     false => {

    //         add_all_duplicates(new_inp);
    // },
    // }
}

fn calculate_duplicates(card_to_calc: Vec<Card>, full_cards: &Vec<Card>) -> Vec<Card> {
    let new_cards = vec![];
    card_to_calc.iter().map(|card| {
        let id = card.id() as usize;
        for i in id..card.winning_numbers_count() {
            let card_to_copy = full_cards.get(i - 1).unwrap();
        }
    });
    new_cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_set() {
        let inp = include_str!("../data/sample_input.txt");
        let inp = parse_card_set(inp).unwrap().1;
        let actual = add_all_duplicates(inp);
        assert_eq!(actual.len(), 30);
    }
}
