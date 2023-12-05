mod shared;
use shared::{parse_card_set, Card};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let mut card_set = parse_card_set(inp).unwrap().1;
    add_all_duplicates(&mut card_set);
    println!("{}", card_set.len());
}

fn add_all_duplicates(inp: &mut Vec<Card>) {
    let mut start_idx = 0;
    loop {
        let mut cards_to_add = vec![];
        let vec_copy = inp.clone();

        let (_, unchecked) = inp.split_at(start_idx);
        start_idx = inp.len();
        if unchecked.is_empty() {
            break;
        }
        let _: Vec<_> = unchecked
            .iter()
            .map(|card| {
                let id = card.id() as usize;
                let winning_numbers_count = card.winning_numbers_count();
                for i in id..=id + winning_numbers_count - 1 {
                    let card_to_copy = vec_copy.get(i).unwrap();
                    cards_to_add.push(card_to_copy.clone());
                }
            })
            .collect();
        let _: Vec<_> = cards_to_add
            .iter()
            .map(|card| inp.push(card.clone()))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_set() {
        let inp = include_str!("../data/sample_input.txt");
        let mut inp = parse_card_set(inp).unwrap().1;
        add_all_duplicates(&mut inp);
        assert_eq!(inp.len(), 30);
    }
}
