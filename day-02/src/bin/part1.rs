mod shared;
use shared::{ Game, GameConfig, parser::parse_set };

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let gameset = parse_set(inp).unwrap().1;
    let config = GameConfig::new(14, 12, 13);
    let sum_of_possible_ids = sum_of_possible_ids(gameset, config);
    println!("{}", sum_of_possible_ids);
}

impl Game {
    fn is_possible(&self, max_blue: u32, max_red: u32, max_green: u32) -> bool {
        self.rounds
            .iter()
            .filter(|round| {
                round.blue <= max_blue && round.red <= max_red && round.green <= max_green
            })
            .count()
            == self.rounds.len()
    }
}

pub fn sum_of_possible_ids(gameset: Vec<Game>, config: GameConfig) -> u32 {
    gameset
        .iter()
        .filter(|game| game.is_possible(config.max_blue, config.max_red, config.max_green))
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_possible_ids() {
        let inp_str = include_str!("../data/sample_input.txt");
        let inp = parse_set(inp_str).unwrap().1;
        let actual = sum_of_possible_ids(inp, GameConfig::new(14, 12, 13));
        assert_eq!(actual, 8);
    }
}
