use aoc_utils::{BadTileTypeError, Collection, Column, ParseableCharacters, Row, Tile};
use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ParabolicFieldTile {
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

enum Direction {
    Positive,
    Negative,
}

pub struct ParabolicFieldCollection(pub Collection<ParabolicFieldTile>);
impl ParabolicFieldCollection {
    pub fn tilt_north(&self) -> Vec<Vec<ParabolicFieldTile>> {
        let mut tilted = vec![];
        for i in 0..self.0.count_columns() {
            tilted.push(ParabolicFieldCollection::tilt(
                self.0.get_column(i),
                Direction::Negative,
            ));
        }
        tilted
    }
    pub fn tilt_south(&self) -> Vec<Vec<ParabolicFieldTile>> {
        let mut tilted = vec![];
        for i in 0..self.0.count_columns() {
            tilted.push(ParabolicFieldCollection::tilt(
                self.0.get_column(i),
                Direction::Positive,
            ));
        }
        tilted
    }
    pub fn tilt_west(&self) -> Vec<Vec<ParabolicFieldTile>> {
        let mut tilted = vec![];
        for i in 0..self.0.count_rows() {
            tilted.push(ParabolicFieldCollection::tilt(
                self.0.get_row(i),
                Direction::Negative,
            ));
        }
        tilted
    }
    pub fn tilt_east(&self) -> Vec<Vec<ParabolicFieldTile>> {
        let mut tilted = vec![];
        for i in 0..self.0.count_rows() {
            tilted.push(ParabolicFieldCollection::tilt(
                self.0.get_row(i),
                Direction::Positive,
            ));
        }
        tilted
    }

    fn tilt(
        row_or_col: Column<ParabolicFieldTile>,
        direction: Direction,
    ) -> Vec<ParabolicFieldTile> {
        let mut data_grouped: Vec<ParabolicFieldTile> = vec![];
        let mut empties = 0;
        match direction {
            Direction::Positive => {
                for tile in row_or_col.into_iter().rev() {
                    match tile.tile_type() {
                        ParabolicFieldTile::Round => data_grouped.push(ParabolicFieldTile::Round),
                        ParabolicFieldTile::Cube => {
                            data_grouped.extend(vec![ParabolicFieldTile::Empty; empties]);
                            data_grouped.push(ParabolicFieldTile::Cube);
                            empties = 0;
                        }
                        ParabolicFieldTile::Empty => empties += 1,
                    }
                }
                data_grouped.extend(vec![ParabolicFieldTile::Empty; empties]);
                data_grouped.reverse();
            },
            Direction::Negative => {
                for tile in row_or_col {
                    match tile.tile_type() {
                        ParabolicFieldTile::Round => data_grouped.push(ParabolicFieldTile::Round),
                        ParabolicFieldTile::Cube => {
                            data_grouped.extend(vec![ParabolicFieldTile::Empty; empties]);
                            data_grouped.push(ParabolicFieldTile::Cube);
                            empties = 0;
                        }
                        ParabolicFieldTile::Empty => empties += 1,
                    }
                }
                data_grouped.extend(vec![ParabolicFieldTile::Empty; empties]);
            }
        };
        data_grouped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{parse_collection, parse_tile_type};
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

    #[test]
    fn test_tilt_row_negative() {
        let inp = include_str!("../../data/sample_input.txt");
        let collection: Collection<ParabolicFieldTile> = parse_collection(inp).unwrap().1;
        let actual = ParabolicFieldCollection::tilt(collection.get_column(2), Direction::Negative);
        let expected = vec![
            ParabolicFieldTile::Round,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Cube,
            ParabolicFieldTile::Round,
            ParabolicFieldTile::Round,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_tilt_row_positive() {
        let inp = include_str!("../../data/sample_input.txt");
        let collection: Collection<ParabolicFieldTile> = parse_collection(inp).unwrap().1;
        let actual = ParabolicFieldCollection::tilt(collection.get_column(2), Direction::Positive);
        let expected = vec![
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Round,
            ParabolicFieldTile::Cube,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Empty,
            ParabolicFieldTile::Round,
            ParabolicFieldTile::Round,
        ];
        assert_eq!(actual, expected);
    }
}
