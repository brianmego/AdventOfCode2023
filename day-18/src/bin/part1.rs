use crate::shared::parse_instruction_set;

mod shared;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let instruction_set = parse_instruction_set(inp).unwrap().1;
    let outline = instruction_set.draw_polygon();
    // Shoelace formula for area, then add the perimiter back in
    let actual = outline.get_area() as usize + (outline.len() / 2) + 1;
    println!("{}", actual);
}
