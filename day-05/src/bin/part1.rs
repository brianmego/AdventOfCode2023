mod shared;
use shared::Almanac;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let almanac = Almanac::from(inp);
    let out = almanac.get_lowest_seed_location(almanac.seeds());
    println!("{}", out);
}
