pub mod parser;

#[derive(Debug)]
struct BadTileError;

#[derive(Debug)]
struct BadRowError;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Space,
    Galaxy,
}

impl TryFrom<&char> for Tile {
    type Error = BadTileError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '#' => Ok(Self::Galaxy),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Row(Vec<Tile>);
impl TryFrom<Vec<char>> for Row {
    type Error = BadRowError;

    fn try_from(value: Vec<char>) -> Result<Self, Self::Error> {
        let tiles = value.iter().map(|c| Tile::try_from(c).unwrap()).collect();
        Ok(Self(tiles))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NetworkedTile {
    tile: Tile,
    location: Loc,
}

impl NetworkedTile {
    fn new(tile: Tile, location: Loc) -> Self {
        Self { tile, location }
    }
    pub fn calculate_distance(&self, other: &NetworkedTile) -> usize {
        let x_diff = (self.location.x as isize - other.location.x as isize).unsigned_abs();
        let y_diff = (self.location.y as isize - other.location.y as isize).unsigned_abs();
        x_diff + y_diff
    }
}

#[derive(Clone)]
struct NetworkedRow(Vec<NetworkedTile>);

impl NetworkedRow {
    fn new(row_num: u8, naive_row: Row) -> Self {
        let networked_tiles: Vec<NetworkedTile> = naive_row
            .0
            .iter()
            .enumerate()
            .map(|(x, tile)| NetworkedTile::new(*tile, Loc::new(x as usize, row_num as usize)))
            .collect();
        Self(networked_tiles)
    }
}

pub struct Universe {
    rows: Vec<NetworkedRow>,
}

impl Universe {
    fn new(rows: Vec<Row>) -> Self {
        let networked_rows: Vec<_> = rows
            .iter()
            .enumerate()
            .map(|(row_num, naive_row)| (NetworkedRow::new(row_num as u8, naive_row.clone())))
            .collect();
        Self {
            rows: networked_rows,
        }
    }

    fn get_tile(&self, loc: Loc) -> Option<&NetworkedTile> {
        self.rows.get(loc.y as usize)?.0.get(loc.x as usize)
    }

    pub fn get_all_galaxies(&self) -> Vec<&NetworkedTile> {
        self.rows.iter().flat_map(|row| {
            row.0.iter().filter(|nt| nt.tile == Tile::Galaxy).collect::<Vec<_>>()
        }).collect()
    }

    pub fn expand(&mut self, multiple: usize) {
        let cols_to_expand = self.get_columns_to_expand();
        let rows_to_expand = self.get_rows_to_expand();
        println!("Expanding all rows");
        rows_to_expand.iter().rev().for_each(|i| {
            for _ in 0..multiple - 1 {
                self.rows.insert(*i, self.rows[*i].clone())
            }
        });
        println!("Transposing Rows");
        self.rows = self.transposed_rows();
        println!("Expanding all columns");
        cols_to_expand.iter().rev().for_each(|i| {
            for _ in 0..multiple - 1 {
                self.rows.insert(*i, self.rows[*i].clone())
            }
        });

        println!("Transposing Rows");
        self.rows = self.transposed_rows();
        println!("Recalculating");
        self.recalculate_loc();
    }

    fn get_columns_to_expand(&self) -> Vec<usize> {
        self.determine_expand_points(&self.transposed_rows())
    }

    fn get_rows_to_expand(&self) -> Vec<usize> {
        self.determine_expand_points(&self.rows)
    }
    fn determine_expand_points(&self, rows: &Vec<NetworkedRow>) -> Vec<usize> {
        rows.iter()
            .enumerate()
            .filter(|(_, row)| row.0.iter().all(|nt| nt.tile == Tile::Space))
            .map(|(i, _)| i)
            .collect()
    }
    fn recalculate_loc(&mut self) {
        for (row_num, row) in self.rows.iter_mut().enumerate() {
            for (col_num, tile) in row.0.iter_mut().enumerate() {
                tile.location.x = col_num;
                tile.location.y = row_num;
            }
        }
    }

    fn transposed_rows(&self) -> Vec<NetworkedRow> {
        (0..self.rows[0].0.len())
            .map(|i| {
                self.rows
                    .iter()
                    .map(|inner| inner.0[i])
                    .collect::<Vec<NetworkedTile>>()
            })
            .map(NetworkedRow)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::parser::parse_universe;
    use test_case::test_case;

    #[test]
    fn test_get_rows_to_expand() {
        let row_1 = Row(vec![Tile::Space, Tile::Space, Tile::Space]);
        let row_2 = Row(vec![Tile::Space, Tile::Space, Tile::Galaxy]);
        let row_3 = Row(vec![Tile::Space, Tile::Space, Tile::Galaxy]);
        let universe = Universe::new(vec![row_1, row_2, row_3]);
        assert_eq!(universe.get_columns_to_expand(), vec![0, 1]);
    }

    #[test]
    fn test_get_cols_to_expand() {
        let row_1 = Row(vec![Tile::Space, Tile::Space, Tile::Space]);
        let row_2 = Row(vec![Tile::Space, Tile::Space, Tile::Galaxy]);
        let row_3 = Row(vec![Tile::Space, Tile::Space, Tile::Galaxy]);
        let universe = Universe::new(vec![row_1, row_2, row_3]);
        assert_eq!(universe.get_columns_to_expand(), vec![0, 1]);
    }
    #[test]
    fn test_expand() {
        let inp = include_str!("../../data/sample_input.txt");
        let mut universe = parse_universe(inp).unwrap().1;
        assert_eq!(universe.rows.len(), 10);
        assert_eq!(universe.rows[0].0.len(), 10);
        assert_eq!(
            universe.get_tile(Loc::new(3, 0)).unwrap().tile,
            Tile::Galaxy
        );
        assert_eq!(
            universe.get_tile(Loc::new(3, 0)).unwrap().location,
            Loc::new(3, 0)
        );
        universe.expand(2);
        assert_eq!(universe.rows.len(), 12);
        assert_eq!(universe.rows[0].0.len(), 13);
        assert_eq!(
            universe.get_tile(Loc::new(4, 0)).unwrap().tile,
            Tile::Galaxy
        );
        assert_eq!(
            universe.get_tile(Loc::new(4, 0)).unwrap().location,
            Loc::new(4, 0)
        );
    }

    #[test_case((Loc::new(1, 6), Loc::new(5, 11)), 9)]
    #[test_case((Loc::new(4, 0), Loc::new(9, 10)), 15)]
    #[test_case((Loc::new(0, 2), Loc::new(12, 7)), 17)]
    #[test_case((Loc::new(12, 7), Loc::new(9, 10)), 6)]
    #[test_case((Loc::new(0, 11), Loc::new(5, 11)), 5)]
    fn test_calculate_distance((loc1, loc2): (Loc, Loc), exp: usize) {
        let inp = include_str!("../../data/sample_input.txt");
        let mut universe = parse_universe(inp).unwrap().1;
        universe.expand(2);
        let galaxy_1 = universe.get_tile(loc1).unwrap();
        let galaxy_2 = universe.get_tile(loc2).unwrap();
        assert_eq!(galaxy_1.tile, Tile::Galaxy);
        assert_eq!(galaxy_2.tile, Tile::Galaxy);
        let actual = galaxy_1.calculate_distance(galaxy_2);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_get_all_galaxies() {
        let inp = include_str!("../../data/sample_input.txt");
        let universe = parse_universe(inp).unwrap().1;
        let actual = universe.get_all_galaxies();
        assert_eq!(actual.len(), 9);
    }
}
