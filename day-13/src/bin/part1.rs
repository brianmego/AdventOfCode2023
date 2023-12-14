mod shared;
use shared::{parser::parse_collection_group, CollectionGroup};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let collection_group = parse_collection_group(inp).unwrap().1;
    let out = summarize_notes(collection_group);
    println!("{}", out);
}

fn summarize_notes(collection_group: CollectionGroup) -> usize {
    let mut sum = 0;
    for collection in collection_group {
        let row_score = collection.find_row_symmetry().map_or(0, |r| r * 100);
        let col_score = collection.find_column_symmetry().unwrap_or(0);
        sum += row_score + col_score;
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_summarize_notes() {
        let inp = include_str!("../data/sample_input.txt");
        let collection_group = parse_collection_group(inp).unwrap().1;
        let actual = summarize_notes(collection_group);
        let expected = 405;
        assert_eq!(actual, expected);
    }
}
