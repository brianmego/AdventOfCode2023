mod shared;
use crate::shared::parse_true_instruction_set;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let instruction_set = parse_true_instruction_set(inp).unwrap().1;
    let outline = instruction_set.draw_polygon();
    let perimiter= instruction_set.get_perimiter();
    // Shoelace formula for area, then add the perimiter back in
    let actual = outline.get_area() + (perimiter / 2) + 1;
    println!("{}", actual);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area() {
        let inp = include_str!("../data/sample_input.txt");
        let instruction_set = parse_true_instruction_set(inp).unwrap().1;
        let outline = instruction_set.draw_polygon();
        let perimiter = instruction_set.get_perimiter();
        let actual = outline.get_area() + (perimiter / 2) + 1;
        assert_eq!(actual, 952408144115);
    }
}
