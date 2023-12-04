mod shared;
use shared::{parse_engine_schematic, EngineSchematic, EnginePart, EnginePOI};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let schematic = parse_engine_schematic(inp).unwrap().1;
    println!("{}", sum_of_gear_ratios(&schematic));
}

fn sum_of_gear_ratios(schematic: &EngineSchematic) -> i32 {
    schematic
        .get_gears()
        .iter()
        .map(|gear| gear.get_gear_ratio())
        .sum()
}

impl Gear {
}

#[derive(Debug, PartialEq)]
pub struct Gear {
    part_one: EnginePart,
    part_two: EnginePart,
}

impl Gear {
    fn new(part_one: EnginePart, part_two: EnginePart) -> Self {
        Self { part_one, part_two }
    }

    pub fn get_gear_ratio(&self) -> i32 {
        self.part_one.get_value() * self.part_two.get_value()
    }
}

impl EngineSchematic {

    fn get_nearby_parts(&self, symbol: &EnginePOI) -> Vec<&EnginePOI> {
        self.parts
            .iter()
            .filter(|part| {
                part.location.row >= symbol.location.row - 1
                    && part.location.row <= symbol.location.row + 1
                    && part.location.start_col <= symbol.location.end_col
                    && part.location.end_col >= symbol.location.start_col
            })
            .collect()
    }

    pub fn get_gears(&self) -> Vec<Gear> {
        self.symbols
            .iter()
            .filter(|poi| match poi.part {
                EnginePart::Symbol(s) => s == '*',
                _ => false,
            })
            .filter(|poi| {
                let matching_parts = self.get_nearby_parts(poi);
                matching_parts.len() == 2
            })
            .map(|poi| {
                let nearby_parts = self.get_nearby_parts(poi);
                Gear::new(nearby_parts[0].part.clone(), nearby_parts[1].part.clone())
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
        let actual = sum_of_gear_ratios(&schematic);
        assert_eq!(actual, 467835);
    }

    #[test]
    fn test_get_gears() {
        let inp = include_str!("../data/sample_input.txt");
        let schematic = parse_engine_schematic(inp).unwrap().1;
        let actual = schematic.get_gears();
        dbg!(&actual);
        assert_eq!(actual.len(), 2);
    }
}
