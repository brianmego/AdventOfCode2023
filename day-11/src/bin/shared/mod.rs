pub mod parser;

#[derive(Debug)]
struct BadTileError;

#[derive(Debug)]
struct BadRowError;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Space,
    Galaxy
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
    x: i32,
    y: i32,
}

impl Loc {
    fn new(x: i32, y: i32) -> Self {
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
}

struct NetworkedRow(Vec<NetworkedTile>);

impl NetworkedRow {
    fn new(row_num: u8, naive_row: Row) -> Self {
        let networked_tiles: Vec<NetworkedTile> = naive_row
            .0
            .iter()
            .enumerate()
            .map(|(x, tile)| NetworkedTile::new(*tile, Loc::new(x as i32, row_num as i32)))
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

    fn expand(&mut self) {

    }
}
