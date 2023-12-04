mod shared;
use shared::{parse_engine_schematic, EnginePOI, EnginePart, EngineSchematic};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let schematic = parse_engine_schematic(inp).unwrap().1;
    println!("{}", sum_of_valid_parts(&schematic));
}

fn sum_of_valid_parts(schematic: &EngineSchematic) -> i32 {
    schematic
        .get_valid_parts()
        .iter()
        .map(|part| part.get_value())
        .sum()
}
impl EngineSchematic {
    pub fn get_valid_parts(&self) -> Vec<&EnginePart> {
        self.parts
            .iter()
            .filter(|part| {
                let matching_symbols = self.get_nearby_symbols(part);
                !matching_symbols.is_empty()
            })
            .map(|poi| &poi.part)
            .collect()
    }
    fn get_nearby_symbols(&self, part: &EnginePOI) -> Vec<&EnginePOI> {
        self.symbols
            .iter()
            .filter(|s| {
                s.location.row >= part.location.row - 1
                    && s.location.row <= part.location.row + 1
                    && s.location.start_col >= part.location.start_col - 1
                    && s.location.end_col <= part.location.end_col + 1
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_of_valid_parts() {
        let inp = include_str!("../data/sample_input.txt");
        let schematic = parse_engine_schematic(inp).unwrap().1;
        let actual = sum_of_valid_parts(&schematic);
        assert_eq!(actual, 4361);
    }
}
