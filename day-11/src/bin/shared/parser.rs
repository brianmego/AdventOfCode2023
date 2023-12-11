use crate::shared::{Universe, Row};
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};

fn parse_row(inp: &str) -> IResult<&str, Row> {
    map(many1(one_of(".#")), |chars| {
        Row::try_from(chars).unwrap()
    })(inp)
}

pub fn parse_galaxy(inp: &str) -> IResult<&str, Universe> {
    map(many1(terminated(parse_row, newline)), |rows| {
        Universe::new(rows)
    })(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{Tile, Loc};
    use test_case::test_case;

    #[test_case(".#.", ("", Row(vec![Tile::Space, Tile::Galaxy, Tile::Space])); "Row1")]
    #[test_case("#.#", ("", Row(vec![Tile::Galaxy, Tile::Space, Tile::Galaxy])); "Row2")]
    fn test_parse_row(inp: &str, exp: (&str, Row)) {
        let actual = parse_row(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_universe() {
        let inp = include_str!("../../data/sample_input.txt");
        let universe = parse_galaxy(inp).unwrap().1;
        assert_eq!(universe.rows.len(), 10);
        assert_eq!(universe.get_tile(Loc::new(0, 2)).unwrap().tile, Tile::Galaxy);
        assert_eq!(universe.get_tile(Loc::new(0, 0)).unwrap().tile, Tile::Space);
    }
}

