mod shared;
use std::thread;

use aoc_utils::{parse_collection, Collection, Direction, Loc};
use shared::{Energize, MirrorTile};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let collection: Collection<MirrorTile> = parse_collection(inp).unwrap().1;
    let actual = test_all_entrypoints(collection);
    println!("{}", actual);
}

fn test_all_entrypoints(collection: Collection<MirrorTile>) -> usize {
    let mut runs: Vec<usize> = vec![];
    let max_row = collection.count_rows() - 1;
    let max_column = collection.count_columns() - 1;
    let mut threads = vec![];
    for row_num in 0..=max_row {
        let collection = collection.clone();
        threads.push(thread::spawn(move || {
            run_single_entrypoint(collection, 0, row_num, Direction::East)
        }))
    }
    for row_num in 0..=max_row {
        let collection = collection.clone();
        threads.push(thread::spawn(move || {
            run_single_entrypoint(collection, max_column, row_num, Direction::West)
        }))
    }
    for col_num in 0..=max_column {
        let collection = collection.clone();
        threads.push(thread::spawn(move || {
            run_single_entrypoint(collection, col_num, 0, Direction::South)
        }))
    }
    for col_num in 0..=max_column {
        let collection = collection.clone();
        threads.push(thread::spawn(move || {
            run_single_entrypoint(collection, col_num, max_row, Direction::North)
        }))
    }
    println!("Threads: {}", threads.len());
    for thread in threads {
        runs.push(thread.join().unwrap());
    }
    runs.sort();
    *runs.last().unwrap()
}

fn run_single_entrypoint(
    collection: Collection<MirrorTile>,
    x: usize,
    y: usize,
    direction: Direction,
) -> usize {
    collection.energize_tiles(Loc::new(x, y), direction).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_entrypoints_sample() {
        let inp = include_str!("../data/sample_input.txt");
        let collection: Collection<MirrorTile> = parse_collection(inp).unwrap().1;
        let actual = test_all_entrypoints(collection);
        assert_eq!(actual, 51);
    }
}
