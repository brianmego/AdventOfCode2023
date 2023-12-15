use aoc_utils::{parse_collection, Collection};

use crate::shared::{ParabolicFieldCollection, ParabolicFieldTile};
mod shared;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let col: Collection<ParabolicFieldTile> = parse_collection(inp).unwrap().1;
    let col = ParabolicFieldCollection(col);
    let north_tilted = col.tilt_north();
    let actual = calcuate_north_load(north_tilted);
    println!("{}", actual);
}

fn calcuate_north_load(parabolic_field_collection: Vec<Vec<ParabolicFieldTile>>) -> usize {
    let mut sum = 0;
    for col in parabolic_field_collection {
        for (row_num, tile) in col.iter().rev().enumerate() {
            if tile == &ParabolicFieldTile::Round {
                sum += row_num + 1;
            }
        }
    }
    sum
}

#[test]
fn test_parse_collection() {
    let inp = include_str!("../data/sample_input.txt");
    let col: Collection<ParabolicFieldTile> = parse_collection(inp).unwrap().1;
    let col = ParabolicFieldCollection(col);
    let north_tilted = col.tilt_north();
    let actual = calcuate_north_load(north_tilted);

    assert_eq!(actual, 136);
}
