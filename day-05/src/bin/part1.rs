mod shared;
use shared::{Almanac, ItemType};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let almanac = Almanac::from(inp);
    let out = get_lowest_seed_location(&almanac);
    println!("{}", out);
}

fn get_lowest_seed_location(almanac: &Almanac) -> usize {
    let seeds = almanac.seeds();
    seeds
        .iter()
        .map(|seed| almanac.convert(&ItemType::Seed, &ItemType::Location, *seed))
        .min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lowest_seed_location() {
        let inp = include_str!("../data/sample_input.txt");
        let almanac = Almanac::from(inp);
        let actual = get_lowest_seed_location(&almanac);
        assert_eq!(actual, 35);
    }
}
