use crate::shared::{Network, Row};
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};

fn parse_row(inp: &str) -> IResult<&str, Row> {
    map(many1(one_of(".|-LJ7FS")), |chars| {
        Row::try_from(chars).unwrap()
    })(inp)
}

pub fn parse_network(inp: &str) -> IResult<&str, Network> {
    map(many1(terminated(parse_row, newline)), |rows| {
        Network::new(rows)
    })(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{Tile, Pipe};
    use test_case::test_case;

    #[test_case(".....", ("", Row(vec![Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground])); "Row1")]
    #[test_case(".S-7.", ("", Row(vec![Tile::Ground, Tile::StartingPosition, Tile::Pipe(Pipe::Horizontal), Tile::Pipe(Pipe::SouthWestBend), Tile::Ground])); "Row2")]
    #[test_case(".|.|.", ("", Row(vec![Tile::Ground, Tile::Pipe(Pipe::Vertical), Tile::Ground, Tile::Pipe(Pipe::Vertical), Tile::Ground])); "Row3")]
    #[test_case(".L-J.", ("", Row(vec![Tile::Ground, Tile::Pipe(Pipe::NorthEastBend), Tile::Pipe(Pipe::Horizontal), Tile::Pipe(Pipe::NorthWestBend), Tile::Ground])); "Row4")]
    fn test_parse_row(inp: &str, exp: (&str, Row)) {
        let actual = parse_row(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_network() {
        let inp = include_str!("../../data/sample_input.txt");
        let network = parse_network(inp).unwrap().1;
        assert_eq!(network.rows.len(), 5);
    }
}
