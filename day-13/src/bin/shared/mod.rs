pub mod parser;
use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileType {
    Ash,
    Rocks,
}
impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TileType::Ash => ".",
            TileType::Rocks => "#",
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Debug)]
struct Row<'a>(Vec<&'a Tile>);
impl Display for Row<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = self.0.iter().map(|t| t.to_string()).join("");
        f.write_str(&out)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Collection(Vec<Tile>);
impl Collection {
    fn push(&mut self, tile: Tile) {
        self.0.push(tile)
    }
    fn get_row(&self, row_num: usize) -> Row {
        Row(self.0.iter().filter(|t| t.loc.y == row_num).collect())
    }
    fn get_column(&self, col_num: usize) -> Row {
        Row(self.0.iter().filter(|t| t.loc.x == col_num).collect())
    }
    fn count_rows(&self) -> usize {
        self.0.iter().unique_by(|t| t.loc.y).count()
    }
    fn count_columns(&self) -> usize {
        self.0.iter().unique_by(|t| t.loc.x).count()
    }
}

pub type CollectionGroup = Vec<Collection>;

#[derive(Debug, Copy, Clone)]
struct Tile {
    tile_type: TileType,
    loc: Loc,
}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.tile_type == other.tile_type
    }
}

impl Tile {
    fn new(tile_type: TileType, loc: Loc) -> Self {
        Self { tile_type, loc }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.tile_type.to_string())
    }
}

#[derive(Debug)]
struct BadTileTypeError;

impl TryFrom<&char> for TileType {
    type Error = BadTileTypeError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rocks),
            _ => unreachable!(),
        }
    }
}

type Valley = Collection;

impl Valley {
    pub fn find_column_symmetry(&self) -> Option<usize> {
        let col_count = self.count_columns();
        let mut possible_symmetry_cols = vec![];
        for i in 0..col_count {
            match self.get_column(i) == self.get_column(i + 1) {
                true => possible_symmetry_cols.push(i),
                false => (),
            }
        };
        'a: for col in possible_symmetry_cols.iter() {
            let left_pointer = *col;
            let right_pointer = *col + 1;
            let mut step = 1;
            while (step <= left_pointer) && (right_pointer + step) < col_count {
                let left_pointer = left_pointer - step;
                let right_pointer = right_pointer + step;
                let left_column = self.get_column(left_pointer);
                let right_column = self.get_column(right_pointer);
                if left_column != right_column {
                    continue 'a;
                }
                step += 1;
            };
            return Some(col + 1);
        }
        None
    }
    pub fn find_row_symmetry(&self) -> Option<usize> {
        let row_count = self.count_rows();
        let mut possible_symmetry_rows = vec![];
        for i in 0..row_count {
            match self.get_row(i) == self.get_row(i + 1) {
                true => possible_symmetry_rows.push(i),
                false => (),
            }
        };
        'a: for row in possible_symmetry_rows.iter() {
            let top_pointer = *row;
            let bottom_pointer = *row + 1;
            let mut step = 1;
            while (step <= top_pointer) && (bottom_pointer + step) < row_count {
                let top_pointer = top_pointer - step;
                let bottom_pointer = bottom_pointer + step;
                let top_row = self.get_row(top_pointer);
                let bottom_row = self.get_row(bottom_pointer);
                if top_row != bottom_row {
                    continue 'a;
                }
                step += 1;
            };
            return Some(row + 1);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::parser::parse_collection_group;

    #[test]
    fn test_count_rows() {
        let inp = include_str!("../../data/sample_input.txt");
        let valley = &parse_collection_group(inp).unwrap().1[0];
        assert_eq!(valley.count_rows(), 7);
    }
    #[test]
    fn test_count_columns() {
        let inp = include_str!("../../data/sample_input.txt");
        let valley = &parse_collection_group(inp).unwrap().1[0];
        assert_eq!(valley.count_columns(), 9);
    }
    #[test]
    fn test_get_row() {
        let inp = include_str!("../../data/sample_input.txt");
        let valley = &parse_collection_group(inp).unwrap().1[0];
        let actual = valley.get_row(1).to_string();
        let expected = "..#.##.#.";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_column_symmetry_none() {
        let inp = include_str!("../../data/sample_input.txt");
        let valley = &parse_collection_group(inp).unwrap().1[1];
        let actual = valley.find_column_symmetry();
        assert_eq!(actual, None);
    }
    #[test]
    fn test_find_column_symmetry_some() {
        let inp = include_str!("../../data/sample_input.txt");
        let valley = &parse_collection_group(inp).unwrap().1[0];
        let actual = valley.find_column_symmetry();
        assert_eq!(actual, Some(5));
    }

    #[test]
    fn test_find_row_symmetry_none() {
        let inp = include_str!("../../data/sample_input.txt");
        let valley = &parse_collection_group(inp).unwrap().1[0];
        let actual = valley.find_row_symmetry();
        assert_eq!(actual, None);
    }
    #[test]
    fn test_find_row_symmetry_some() {
        let inp = include_str!("../../data/sample_input.txt");
        let valley = &parse_collection_group(inp).unwrap().1[1];
        let actual = valley.find_row_symmetry();
        assert_eq!(actual, Some(4));
    }
}
