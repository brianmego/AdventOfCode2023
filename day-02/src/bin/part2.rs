use std::str::FromStr;

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::opt,
    error::Error,
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let gameset = parse_set(inp).unwrap().1;
    let powers = sum_of_powers(gameset);
    println!("{}", powers);
}

struct GameConfig {
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

impl GameConfig {
    fn new(max_blue: u32, max_red: u32, max_green: u32) -> Self {
        Self {
            max_blue,
            max_red,
            max_green,
        }
    }
}
#[derive(Debug, PartialEq, Default)]
struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug, PartialEq, Default)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(id: u32, rounds: Vec<Round>) -> Self {
        Self { id, rounds }
    }

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

impl Round {
    fn new(blue: u32, red: u32, green: u32) -> Self {
        Self { blue, red, green }
    }
}
#[derive(Debug, PartialEq)]
enum BeadColor {
    Blue,
    Red,
    Green,
}
#[derive(Debug)]
struct BadBeadColor;
impl FromStr for BeadColor {
    type Err = BadBeadColor;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            _ => Err(BadBeadColor),
        }
    }
}

fn discover_color(inp: &str) -> IResult<&str, (u32, BeadColor)> {
    let (inp, (count, _, color)) = match tuple((
        digit1::<&str, Error<&str>>,
        tag(" "),
        alt((tag("blue"), tag("green"), tag("red"))),
    ))(inp)
    {
        Ok((remaining, (count, _, color))) => {
            (remaining, (count, "", BeadColor::from_str(color).unwrap()))
        }
        Err(e) => Err(e)?,
    };
    Ok((inp, (count.parse::<u32>().unwrap(), color)))
}
/// parses '3 blue, 4 red, 1 green;'
fn parse_round(inp: &str) -> IResult<&str, Round> {
    let (inp, many_match): (&str, Vec<(u32, BeadColor)>) = terminated(
        many0(terminated(discover_color, opt(tag(", ")))),
        opt(tag(";")),
    )(inp)?;
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;
    many_match.iter().for_each(|res| match res.1 {
        BeadColor::Blue => blue = res.0,
        BeadColor::Red => red = res.0,
        BeadColor::Green => green = res.0,
    });
    Ok((inp, Round::new(blue, red, green)))
}

/// parses 'Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red'
fn parse_game(inp: &str) -> IResult<&str, Game> {
    let (inp, (game_id, rounds)) = tuple((
        delimited(tag("Game "), digit1, tag(":")),
        many1(preceded(tag(" "), parse_round)),
    ))(inp)?;
    Ok((inp, Game::new(game_id.parse::<u32>().unwrap(), rounds)))
}

/// parses multiple Game lines into a vector
fn parse_set(inp: &str) -> IResult<&str, Vec<Game>> {
    many1(terminated(parse_game, newline))(inp)
}

fn sum_of_powers(gameset: Vec<Game>) -> u32 {
    gameset.iter().map(|game| game.power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("3 green", ("", (3, BeadColor::Green)))]
    #[test_case("3 blue", ("", (3, BeadColor::Blue)))]
    #[test_case("22 red", ("", (22, BeadColor::Red)))]
    fn test_discover_color(inp: &str, exp: (&str, (u32, BeadColor))) {
        let actual = discover_color(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_discover_color_err() {
        let actual = discover_color("21 orange");
        assert!(actual.is_err());
    }

    #[test_case("3 blue, 4 red;", ("", Round::new(3, 4, 0)))]
    #[test_case("1 red, 2 green, 6 blue;", ("", Round::new(6, 1, 2)))]
    #[test_case("2 green", ("", Round::new(0, 0, 2)))]
    #[test_case("20 red, 13 green", ("", Round::new(0, 20, 13)))]
    fn test_parse_round(inp: &str, exp: (&str, Round)) {
        let actual = parse_round(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test_case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        (
            "",
            Game::new(
                3,
                vec![Round::new(6, 20, 8),
                    Round::new(5, 4, 13),
                    Round::new(0, 1, 5)],
            )
        )
    )]
    fn test_parse_game(inp: &str, exp: (&str, Game)) {
        let actual = parse_game(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_set() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = parse_set(inp);
        assert!(actual.is_ok());
        let games = actual.unwrap().1;
        assert_eq!(games.len(), 5);
        assert_eq!(games[4].id, 5);
    }

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
