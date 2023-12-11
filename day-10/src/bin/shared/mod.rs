pub mod parser;

#[derive(Debug)]
struct BadTileError;

#[derive(Debug)]
struct BadRowError;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Ground,
    Pipe(Pipe),
    StartingPosition,
}

impl Tile {
    ///Things that can be connected to the north side of self
    fn valid_north_connection_points(&self) -> Vec<Tile> {
        match self {
            Tile::Pipe(Pipe::Vertical | Pipe::NorthEastBend | Pipe::NorthWestBend) => vec![
                Tile::Pipe(Pipe::Vertical),
                Tile::Pipe(Pipe::SouthEastBend),
                Tile::Pipe(Pipe::SouthWestBend),
                Tile::StartingPosition,
            ],
            Tile::StartingPosition => vec![
                Tile::Pipe(Pipe::Vertical),
                Tile::Pipe(Pipe::SouthEastBend),
                Tile::Pipe(Pipe::SouthWestBend),
            ],
            Tile::Ground
            | Tile::Pipe(Pipe::Horizontal | Pipe::SouthEastBend | Pipe::SouthWestBend) => vec![],
        }
    }

    ///Things that can be connected to the south side of self
    fn valid_south_connection_points(&self) -> Vec<Tile> {
        match self {
            Tile::Pipe(Pipe::Vertical | Pipe::SouthEastBend | Pipe::SouthWestBend) => vec![
                Tile::Pipe(Pipe::Vertical),
                Tile::Pipe(Pipe::NorthEastBend),
                Tile::Pipe(Pipe::NorthWestBend),
            ],
            Tile::StartingPosition => vec![
                Tile::Pipe(Pipe::Vertical),
                Tile::Pipe(Pipe::NorthEastBend),
                Tile::Pipe(Pipe::NorthWestBend),
            ],
            Tile::Ground
            | Tile::Pipe(Pipe::Horizontal | Pipe::NorthEastBend | Pipe::NorthWestBend) => vec![],
        }
    }
    ///Things that can be connected to the east side of self
    fn valid_east_connection_points(&self) -> Vec<Tile> {
        match self {
            Tile::Pipe(Pipe::Horizontal | Pipe::NorthEastBend | Pipe::SouthEastBend) => {
                vec![
                    Tile::Pipe(Pipe::Horizontal),
                    Tile::Pipe(Pipe::NorthWestBend),
                    Tile::Pipe(Pipe::SouthWestBend),
                    Tile::StartingPosition,
                ]
            }
            Tile::StartingPosition => vec![
                Tile::Pipe(Pipe::Horizontal),
                Tile::Pipe(Pipe::NorthWestBend),
                Tile::Pipe(Pipe::SouthWestBend),
            ],
            Tile::Ground
            | Tile::Pipe(Pipe::Vertical | Pipe::NorthWestBend | Pipe::SouthWestBend) => vec![],
        }
    }
    ///Things that can be connected to the west side of self
    fn valid_west_connection_points(&self) -> Vec<Tile> {
        match self {
            Tile::Pipe(Pipe::Horizontal | Pipe::NorthWestBend | Pipe::SouthWestBend) => vec![
                Tile::Pipe(Pipe::Horizontal),
                Tile::Pipe(Pipe::NorthEastBend),
                Tile::Pipe(Pipe::SouthEastBend),
                Tile::StartingPosition,
            ],
            Tile::StartingPosition => vec![
                Tile::Pipe(Pipe::Horizontal),
                Tile::Pipe(Pipe::NorthEastBend),
                Tile::Pipe(Pipe::SouthEastBend),
            ],
            Tile::Ground
            | Tile::Pipe(Pipe::Vertical | Pipe::NorthEastBend | Pipe::SouthEastBend) => vec![],
        }
    }
}

impl TryFrom<&char> for Tile {
    type Error = BadTileError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '|' => Ok(Self::Pipe(Pipe::Vertical)),
            '-' => Ok(Self::Pipe(Pipe::Horizontal)),
            'L' => Ok(Self::Pipe(Pipe::NorthEastBend)),
            'J' => Ok(Self::Pipe(Pipe::NorthWestBend)),
            '7' => Ok(Self::Pipe(Pipe::SouthWestBend)),
            'F' => Ok(Self::Pipe(Pipe::SouthEastBend)),
            'S' => Ok(Self::StartingPosition),
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Loc {
    x: i32,
    y: i32,
}

impl Loc {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn west(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }
    fn east(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }
    fn north(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }
    fn south(&self) -> Self {
        Self::new(self.x, self.y + 1)
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

#[derive(Default)]
pub struct Path {
    networked_tiles: Vec<NetworkedTile>,
}

impl Path {
    fn push(&mut self, next_tile: NetworkedTile) {
        self.networked_tiles.push(next_tile)
    }
    pub fn len(&self) -> usize {
        self.networked_tiles.len()
    }
}

pub struct Network {
    rows: Vec<NetworkedRow>,
}

impl Network {
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

    pub fn get_starting_tile(&self) -> Option<&NetworkedTile> {
        for row in &self.rows {
            for tile in &row.0 {
                if tile.tile == Tile::StartingPosition {
                    return Some(tile);
                }
            }
        }
        None
    }

    pub fn get_connected_paths(
        &self,
        this_tile: &NetworkedTile,
        start_tile: &NetworkedTile,
        prev_tile: Option<&NetworkedTile>,
        mut existing_path: Path,
    ) -> Path {
        let connections = self.get_tile_connections(this_tile);
        debug_assert!(connections.len() == 2);

        let next_tile = match prev_tile {
            Some(t) => match connections[0] == t {
                true => connections[1],
                false => connections[0],
            },
            None => {
                existing_path.push(*this_tile);
                connections[0]
            }
        };
        match next_tile == start_tile {
            true => existing_path,
            false => {
                existing_path.push(*next_tile);
                self.get_connected_paths(next_tile, start_tile, Some(this_tile), existing_path)
            }
        }
    }

    fn get_tile_connections(&self, networked_tile: &NetworkedTile) -> Vec<&NetworkedTile> {
        let mut connections = vec![];
        if let Some(connected) = self.get_tile(networked_tile.location.west()) {
            if networked_tile
                .tile
                .valid_west_connection_points()
                .contains(&connected.tile)
            {
                connections.push(connected);
            }
        };
        if let Some(connected) = self.get_tile(networked_tile.location.north()) {
            if networked_tile
                .tile
                .valid_north_connection_points()
                .contains(&connected.tile)
            {
                connections.push(connected);
            }
        };
        if let Some(connected) = self.get_tile(networked_tile.location.east()) {
            if networked_tile
                .tile
                .valid_east_connection_points()
                .contains(&connected.tile)
            {
                connections.push(connected);
            }
        };
        if let Some(connected) = self.get_tile(networked_tile.location.south()) {
            if networked_tile
                .tile
                .valid_south_connection_points()
                .contains(&connected.tile)
            {
                connections.push(connected);
            }
        };
        connections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::parse_network;

    #[test]
    fn test_get_starting_tile() {
        let inp = include_str!("../../data/sample_input.txt");
        let network = parse_network(inp).unwrap().1;
        let actual = network.get_starting_tile().unwrap();
        assert_eq!(actual.location, Loc::new(1, 1));
    }

    #[test]
    fn test_get_connections() {
        let inp = include_str!("../../data/sample_input.txt");
        let network = parse_network(inp).unwrap().1;
        let starting_tile = network.get_tile(Loc::new(1, 1)).unwrap();
        let actual = network.get_tile_connections(starting_tile);
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn get_connected_paths() {
        let inp = include_str!("../../data/sample_input.txt");
        let network = parse_network(inp).unwrap().1;
        let starting_tile = network.get_tile(Loc::new(1, 1)).unwrap();
        let actual =
            network.get_connected_paths(starting_tile, starting_tile, None, Path::default());
        assert_eq!(actual.len(), 8);
    }

    #[test]
    fn get_furthest_from_starting() {
        let inp = include_str!("../../data/sample_input.txt");
        let network = parse_network(inp).unwrap().1;
        let starting_tile = network.get_tile(Loc::new(1, 1)).unwrap();
        let connected_path =
            network.get_connected_paths(starting_tile, starting_tile, None, Path::default());
        assert_eq!(connected_path.len() / 2, 4);
    }

    #[test]
    fn get_furthest_from_starting_sample_2() {
        let inp = include_str!("../../data/sample_input2.txt");
        let network = parse_network(inp).unwrap().1;
        let starting_tile = network.get_tile(Loc::new(1, 1)).unwrap();
        let connected_path =
            network.get_connected_paths(starting_tile, starting_tile, None, Path::default());
        assert_eq!(connected_path.len() / 2, 8);
    }
}
