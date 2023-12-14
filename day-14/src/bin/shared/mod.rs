use aoc_utils::{ParseableCharacters, BadTileTypeError};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ParabolicFieldTile {
    Round,
    Cube,
    Empty,
}
impl TryFrom<char> for ParabolicFieldTile {
    type Error = BadTileTypeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Round),
            '#' => Ok(Self::Cube),
            '.' => Ok(Self::Empty),
            _ => unreachable!(),
        }
    }
}
impl ParseableCharacters for ParabolicFieldTile {
    fn valid_chars() -> Vec<char> {
        vec!['O', '#', '.']
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{parse_tile_type, parse_collection, Collection};
    use nom::IResult;
    use test_case::test_case;

    #[test_case("O", Ok(("", ParabolicFieldTile::Round)); "Round")]
    #[test_case("#", Ok(("", ParabolicFieldTile::Cube)); "Cube")]
    #[test_case(".", Ok(("", ParabolicFieldTile::Empty)); "Empty")]
    fn test_tile_type(inp: &str, exp: IResult<&str, ParabolicFieldTile>) {
        let actual = parse_tile_type(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_collection() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual: Collection<ParabolicFieldTile> = parse_collection(inp).unwrap().1;
        assert_eq!(actual.len(), 100);
    }
}
