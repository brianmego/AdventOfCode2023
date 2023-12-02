mod shared;
use shared::{Game, GameConfig, parser::parse_set};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let gameset = parse_set(inp).unwrap().1;
    let powers = sum_of_powers(gameset);
    println!("{}", powers);
}

fn sum_of_powers(gameset: Vec<Game>) -> u32 {
    gameset.iter().map(|game| game.power()).sum()
}
impl Game {
    fn optimal_config(&self) -> GameConfig {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;
        let _: Vec<_> = self
            .rounds
            .iter()
            .map(|round| {
                max_blue = round.blue.max(max_blue);
                max_red = round.red.max(max_red);
                max_green = round.green.max(max_green);
            })
            .collect();
        GameConfig::new(max_blue, max_red, max_green)
    }

    fn power(&self) -> u32 {
        let opt_config = self.optimal_config();
        opt_config.max_blue * opt_config.max_red * opt_config.max_green
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_power() {
        let inp_str = include_str!("../data/sample_input.txt");
        let inp = parse_set(inp_str).unwrap().1;
        assert_eq!(inp[0].power(), 48);
        assert_eq!(inp[1].power(), 12);
        assert_eq!(inp[2].power(), 1560);
        assert_eq!(inp[3].power(), 630);
        assert_eq!(inp[4].power(), 36);
    }

    #[test]
    fn test_sum_of_powers() {
        let inp_str = include_str!("../data/sample_input.txt");
        let inp = parse_set(inp_str).unwrap().1;
        let actual = sum_of_powers(inp);
        assert_eq!(actual, 2286);
    }
}
