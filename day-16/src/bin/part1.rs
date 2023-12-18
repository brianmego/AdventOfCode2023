mod shared;
use aoc_utils::{parse_collection, Collection, Direction, Loc};
use shared::{Energize, MirrorTile};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let collection: (&str, Collection<MirrorTile>) = parse_collection(inp).unwrap();
    let actual = collection.1.energize_tiles(Loc::new(0,0), Direction::East);
    println!("{}", actual.len());
}
