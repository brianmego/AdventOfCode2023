use aoc_utils::{BadTileTypeError, Collection, Direction, Loc, ParseableCharacters};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
enum LightBeamMovement {
    Forward,
    Bend,
    Split(Loc, Direction),
    Complete,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct LightBeam<'a> {
    location: Loc,
    collection: &'a Collection<MirrorTile>,
    direction: Direction,
    enabled: bool,
    id: Uuid,
}

#[derive(Clone, Debug, PartialEq)]
struct HistoryEntry((Loc, Direction));

impl<'a> LightBeam<'a> {
    fn new(location: Loc, collection: &'a Collection<MirrorTile>, direction: Direction) -> Self {
        let id = Uuid::new_v4();
        // println!("Spawned beam {} ", id);
        Self {
            location,
            collection,
            direction,
            enabled: true,
            id,
        }
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    #[allow(dead_code)]
    fn enable(&mut self) {
        self.enabled = true;
    }

    fn move_next(&mut self) -> LightBeamMovement {
        let next_loc = self.location.get_neighbor(self.direction);
        // if self.enabled {
        //     dbg!(self.id, self.location, next_loc, self.direction);
        //     println!();
        // }
        match next_loc {
            Some(new_loc) => {
                let movement_type = match self.collection.get_tile(new_loc) {
                    Some(new_tile) => match new_tile.tile_type() {
                        MirrorTile::Empty => LightBeamMovement::Forward,
                        MirrorTile::Mirror(slope) => {
                            match slope {
                                Slope::Positive => match self.direction {
                                    Direction::North | Direction::South => {
                                        self.direction = self.direction.rotate_clockwise()
                                    }
                                    Direction::East | Direction::West => {
                                        self.direction = self.direction.rotate_counterclockwise()
                                    }
                                },
                                Slope::Negative => match self.direction {
                                    Direction::North | Direction::South => {
                                        self.direction = self.direction.rotate_counterclockwise()
                                    }
                                    Direction::East | Direction::West => {
                                        self.direction = self.direction.rotate_clockwise()
                                    }
                                },
                            };
                            LightBeamMovement::Bend
                        }
                        MirrorTile::Splitter(split_type) => match split_type {
                            SplitterType::Horizontal => match self.direction {
                                Direction::North | Direction::South => {
                                    self.direction = Direction::West;
                                    LightBeamMovement::Split(new_loc, Direction::East)
                                }
                                Direction::East | Direction::West => LightBeamMovement::Forward,
                            },
                            SplitterType::Vertical => match self.direction {
                                Direction::East | Direction::West => {
                                    self.direction = Direction::North;
                                    LightBeamMovement::Split(new_loc, Direction::South)
                                }
                                Direction::North | Direction::South => LightBeamMovement::Forward,
                            },
                        },
                    },
                    None => LightBeamMovement::Complete,
                };
                if movement_type != LightBeamMovement::Complete {
                    self.location = new_loc;
                } else {
                    // println!("id: {} COMPLETE", self.id)
                }
                movement_type
            }
            None => {
                // println!("id: {} COMPLETE", self.id);
                LightBeamMovement::Complete
            }
        }
    }
}
#[derive(Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
pub enum Slope {
    Positive, // /
    Negative, // \
}

#[derive(Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
pub enum SplitterType {
    Horizontal, // -
    Vertical,   // |
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Ord, Eq)]
pub enum MirrorTile {
    Mirror(Slope),
    Splitter(SplitterType),
    Empty,
}
impl TryFrom<char> for MirrorTile {
    type Error = BadTileTypeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Self::Splitter(SplitterType::Horizontal)),
            '|' => Ok(Self::Splitter(SplitterType::Vertical)),
            '/' => Ok(Self::Mirror(Slope::Positive)),
            '\\' => Ok(Self::Mirror(Slope::Negative)),
            '.' => Ok(Self::Empty),
            _ => unreachable!(),
        }
    }
}
impl ParseableCharacters for MirrorTile {
    fn valid_chars() -> Vec<char> {
        vec!['.', '|', '-', '\\', '/']
    }
}
pub trait Energize {
    fn energize_tiles(&self, starting_loc: Loc, starting_direction: Direction) -> Vec<Loc>;
}

impl Energize for Collection<MirrorTile> {
    fn energize_tiles(&self, starting_loc: Loc, starting_direction: Direction) -> Vec<Loc> {
        let mut light_beams: Vec<LightBeam> = vec![];
        let starting_direction = match self.get_tile(starting_loc).unwrap().tile_type() {
            MirrorTile::Mirror(_) => Direction::South,
            MirrorTile::Splitter(_) => Direction::South,
            MirrorTile::Empty => starting_direction,
        };
        let mut location_history: Vec<HistoryEntry> = vec![];
        let initial_light_beam = LightBeam::new(starting_loc, self, starting_direction);
        let mut beams_to_add: Vec<LightBeam> = vec![];
        light_beams.push(initial_light_beam);
        loop {
            let enabled_beams = light_beams.iter_mut().filter(|b| b.enabled);
            for beam in enabled_beams {
                location_history.push(HistoryEntry((beam.location, beam.direction)));
                let next = beam.move_next();
                match next {
                    LightBeamMovement::Forward | LightBeamMovement::Bend => {}
                    LightBeamMovement::Split(new_loc, new_direction) => {
                        beams_to_add.push(LightBeam::new(new_loc, self, new_direction));
                    }
                    LightBeamMovement::Complete => beam.disable(),
                }
                if location_history.contains(&HistoryEntry((beam.location, beam.direction))) {
                    beam.disable();
                    continue;
                }
            }
            light_beams.extend(beams_to_add.clone());
            beams_to_add.clear();
            if light_beams.iter().filter(|b| b.enabled).count() == 0 {
                break;
            }
        }
        let mut energized_list: Vec<Loc> = location_history
            .iter()
            .map(|loc_hist| loc_hist.0 .0)
            .collect();
        energized_list.sort();
        energized_list.dedup();
        energized_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::parse_collection;

    #[test]
    fn test_parse_sample() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual: (&str, Collection<MirrorTile>) = parse_collection(inp).unwrap();
        assert_eq!(actual.0.len(), 0);
        assert_eq!(actual.1.len(), 100);
    }

    #[test]
    fn test_energize_sample() {
        let inp = include_str!("../../data/sample_input.txt");
        let collection: (&str, Collection<MirrorTile>) = parse_collection(inp).unwrap();
        let actual = collection.1.energize_tiles(Loc::new(0,0), Direction::East);
        assert_eq!(actual.len(), 46);
    }
}
