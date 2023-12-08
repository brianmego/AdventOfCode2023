use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate<'a> {
    loc: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Coordinate<'a> {
    fn new(loc: &'a str, left: &'a str, right: &'a str) -> Self {
        Self { loc, left, right }
    }
}

/// parses "AAA = (BBB, BBB)"
fn parse_coordinate(inp: &str) -> IResult<&str, Coordinate> {
    let (inp, coordinate): (&str, Coordinate) = map(
        separated_pair(
            alpha1,
            tag(" = ("),
            terminated(tuple((alpha1, tag(", "), alpha1)), tag(")")),
        ),
        |(loc, (left, _, right))| Coordinate::new(loc, left, right),
    )(inp)?;
    Ok((inp, coordinate))
}

/// parses LRLRLRLLLRRR
fn parse_directions(inp: &str) -> IResult<&str, Directions> {
    many1(one_of("LR"))(inp)
}

pub struct Map<'a> {
    directions: Directions,
    database: HashMap<&'a str, Coordinate<'a>>
}


impl<'a> Map<'a> {
    fn new(directions: Directions, coordinates: Vec<Coordinate<'a>>) -> Self {
        let mut database = HashMap::new();
        coordinates.iter().for_each(|c| {database.insert(c.loc, c.clone());});
        Self {
            directions,
            database,
        }
    }

    fn search(&self, point: &str) -> &Coordinate {
        self.database.get(point).unwrap()
    }

    pub fn traverse_steps(&self, desired: &str) -> usize {
        let mut current_coordinate = self.search("AAA");
        let mut steps = 0;
        while current_coordinate.loc != desired {
            self.directions.iter().for_each(|c| {match c {
                'L' => {current_coordinate = self.search(current_coordinate.left); steps += 1},
                'R' => {current_coordinate = self.search(current_coordinate.right); steps += 1},
                _ => unreachable!()
            }});
        }
        steps
    }
}

type Directions = Vec<char>;
pub fn parse_map(inp: &str) -> IResult<&str, Map> {
    map(
        separated_pair(
            parse_directions,
            many1(newline),
            many1(terminated(parse_coordinate, newline)),
        ),
        |(directions, coordinates)| Map::new(directions, coordinates),
    )(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("AAA = (BBB, BBB)", ("", Coordinate::new("AAA", "BBB", "BBB")); "AAA")]
    #[test_case("BBB = (AAA, ZZZ)", ("", Coordinate::new("BBB", "AAA", "ZZZ")); "BBB")]
    #[test_case("ZZZ = (ZZZ, ZZZ)", ("", Coordinate::new("ZZZ", "ZZZ", "ZZZ")); "ZZZ")]
    fn test_parse_coordinate(inp: &str, exp: (&str, Coordinate)) {
        let actual = parse_coordinate(inp);
        assert_eq!(actual, Ok(exp));
    }

    fn test_parse_directions() {
        let actual = parse_directions("LRLRLRLLLRRR");
        let expected = Ok((
            "",
            vec!['L', 'R', 'L', 'R', 'L', 'R', 'L', 'L', 'L', 'R', 'R', 'R'],
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_map() {
        let inp = include_str!("../../data/sample_input.txt");
        let map = parse_map(inp).unwrap().1;
        assert_eq!(map.search("BBB"), &Coordinate::new("BBB", "AAA", "ZZZ"));
        assert_eq!(map.traverse_steps("ZZZ"), 6)
    }
}
