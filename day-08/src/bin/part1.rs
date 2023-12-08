mod shared;
use shared::parse_map;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let map = parse_map(inp).unwrap().1;
    let out = map.traverse_steps("ZZZ");
    println!("{}", out);
}
