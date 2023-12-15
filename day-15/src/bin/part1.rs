mod shared;
use shared::hash;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let out = sum_hash(inp);
    println!("{}", out);
}

fn sum_hash(inp: &str) -> usize {
    inp.split(',').map(hash).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = sum_hash(inp);
        assert_eq!(actual, 1320);
    }
}
