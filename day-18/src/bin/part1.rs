use crate::shared::parse_instruction_set;

mod shared;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let instruction_set = parse_instruction_set(inp).unwrap().1;
    let outline = instruction_set.draw_polygon();
    let actual = instruction_set.fill_polygon(outline);
    println!("{}", actual.len());
}
