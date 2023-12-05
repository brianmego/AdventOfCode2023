mod shared;
use shared::{parse_card_set, Card};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let mut card_set = parse_card_set(inp).unwrap().1;
    add_all_duplicates(&mut card_set);
    println!("{}", card_set.len());
}

fn add_all_duplicates(inp: &mut Vec<Card>) {
    loop {
        let mut cards_to_add = vec![];
        let vec_copy = inp.clone();

        let need_dup_check = inp.iter().filter(|card| !card.already_calculated_duplicates()).count();
        // dbg!(need_dup_check);
        if need_dup_check == 0 {
            break;
        }
        let _: Vec<_> = inp
            .iter_mut()
            .filter(|card| !card.already_calculated_duplicates())
            .map(|card| {
                let id = card.id() as usize;
                card.set_has_calculated_duplicates();
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
        // dbg!(i, &inp);
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
