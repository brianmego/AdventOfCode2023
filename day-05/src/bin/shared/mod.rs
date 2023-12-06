#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ItemType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl ItemType {
    fn next(&self) -> ItemType {
        match self {
            ItemType::Seed => ItemType::Soil,
            ItemType::Soil => ItemType::Fertilizer,
            ItemType::Fertilizer => ItemType::Water,
            ItemType::Water => ItemType::Light,
            ItemType::Light => ItemType::Temperature,
            ItemType::Temperature => ItemType::Humidity,
            ItemType::Humidity => ItemType::Location,
            ItemType::Location => todo!(),
        }
    }
}

impl From<&str> for ItemType {
    fn from(value: &str) -> Self {
        match value {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => {
                println!("Unknown value {}", value);
                panic!()
            }
        }
    }
}
type Seed = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct SeedRange {
    min_id: usize,
    range: usize,
}

impl SeedRange {
    pub fn new(min_id: usize, range: usize) -> Self { Self { min_id, range } }
}

pub struct Almanac {
    seeds: Vec<Seed>,
    converters: Vec<Converter>,
}

impl Almanac {
    fn new(seeds: Vec<Seed>, converters: Vec<Converter>) -> Self {
        Self { seeds, converters }
    }
    pub fn convert(&self, from_type: &ItemType, to_type: &ItemType, value: usize) -> usize {
        if from_type == to_type {
            return value;
        };
        let matching_converter = self
            .converters
            .iter()
            .filter(|c| &c.from_type == from_type)
            .find(|c| c.min_from_id <= value && value <= c.max_from_id);
        match matching_converter {
            Some(c) => match &c.to_type == to_type {
                true => c.min_to_id + (value - c.min_from_id),
                false => self.convert(&c.to_type, to_type, c.min_to_id + (value - c.min_from_id)),
            },
            None => self.convert(&from_type.next(), to_type, value),
        }
    }
    pub fn seeds(&self) -> Vec<Seed> {
        self.seeds.clone()
    }

    pub fn seeds_as_range(&self) -> Vec<SeedRange> {
        self
            .seeds
            .chunks_exact(2)
            .map(|chunk| {
                let start = chunk[0];
                let range = chunk[1];
                SeedRange::new(start, range)
            })
            .collect()
    }

    pub fn get_lowest_seed_location(&self, seeds: Vec<Seed>) -> usize {
        seeds
            .iter()
            .map(|seed| self.convert(&ItemType::Seed, &ItemType::Location, *seed))
            .min()
            .unwrap()
    }
    pub fn get_lowest_seed_location_by_range(&self, seeds: Vec<SeedRange>) -> usize {
        seeds
            .iter()
            .map(|seedrange| self.convert(&ItemType::Seed, &ItemType::Location, seedrange.min_id))
            .min()
            .unwrap()
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut seeds = vec![];
        let mut converters = vec![];
        let mut active_convert_from: ItemType = ItemType::Seed;
        let mut active_convert_to: ItemType = ItemType::Seed;
        for line in value.lines() {
            if line.starts_with("seeds:") {
                let seed_list: &str = line.split("seeds: ").last().unwrap();
                seeds = seed_list
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
            } else if line.ends_with("map:") {
                let convert_str = line.split_whitespace().next().unwrap();
                let mut convert_items = convert_str.split('-').take(3);
                active_convert_from = ItemType::from(convert_items.next().unwrap());
                convert_items.next();
                active_convert_to = ItemType::from(convert_items.next().unwrap());
            } else if line.is_empty() {
                continue;
            } else {
                let mut mapping = line.split_whitespace();
                let id = mapping.next().unwrap().parse::<usize>().unwrap();
                let start_matching_id = mapping.next().unwrap().parse::<usize>().unwrap();
                let range_of_matches = mapping.next().unwrap().parse::<usize>().unwrap();
                converters.push(Converter::new(
                    active_convert_from,
                    active_convert_to,
                    id,
                    start_matching_id,
                    range_of_matches,
                ));
            }
        }
        Self { seeds, converters }
    }
}
#[derive(Debug, PartialEq)]
struct Converter {
    from_type: ItemType,
    to_type: ItemType,
    min_to_id: usize,
    max_to_id: usize,
    min_from_id: usize,
    max_from_id: usize,
}

impl Converter {
    fn new(
        from_type: ItemType,
        to_type: ItemType,
        to_id: usize,
        from_id: usize,
        range_of_matches: usize,
    ) -> Self {
        let min_to_id = to_id;
        let max_to_id = to_id + range_of_matches - 1;
        let min_from_id = from_id;
        let max_from_id = from_id + range_of_matches - 1;
        Converter {
            from_type,
            to_type,
            min_to_id,
            max_to_id,
            min_from_id,
            max_from_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_converter() {
        let actual = Converter::new(ItemType::Seed, ItemType::Soil, 50, 98, 2);
        assert_eq!(actual.min_to_id, 50);
        assert_eq!(actual.max_to_id, 51);
        assert_eq!(actual.min_from_id, 98);
        assert_eq!(actual.max_from_id, 99);
        assert_eq!(actual.from_type, ItemType::Seed);
        assert_eq!(actual.to_type, ItemType::Soil);
    }

    #[test]
    fn test_parse_almanac() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual = Almanac::from(inp);
        assert_eq!(actual.seeds, vec![79, 14, 55, 13]);
        let expected_converter = Converter::new(ItemType::Seed, ItemType::Soil, 50, 98, 2);
        assert_eq!(actual.converters.len(), 18);
        assert_eq!(actual.converters[0], expected_converter);
    }

    #[test_case(ItemType::Seed, ItemType::Soil, 79, 81)]
    #[test_case(ItemType::Seed, ItemType::Soil, 14, 14)]
    #[test_case(ItemType::Seed, ItemType::Soil, 55, 57)]
    #[test_case(ItemType::Seed, ItemType::Soil, 13, 13)]
    #[test_case(ItemType::Seed, ItemType::Location, 79, 82)]
    #[test_case(ItemType::Seed, ItemType::Location, 14, 43)]
    #[test_case(ItemType::Seed, ItemType::Location, 55, 86)]
    #[test_case(ItemType::Seed, ItemType::Location, 13, 35)]
    fn test_almanac_lookup(from_type: ItemType, to_type: ItemType, from_id: usize, exp: usize) {
        let inp = include_str!("../../data/sample_input.txt");
        let actual = Almanac::from(inp);
        assert_eq!(actual.convert(&from_type, &to_type, from_id), exp);
    }

    #[test]
    fn test_get_lowest_seed_location() {
        let inp = include_str!("../../data/sample_input.txt");
        let almanac = Almanac::from(inp);
        let actual = almanac.get_lowest_seed_location(almanac.seeds());
        assert_eq!(actual, 35);
    }

    // #[test]
    // fn test_get_lowest_seed_location_seed_range() {
    //     let inp = include_str!("../../data/sample_input.txt");
    //     let almanac = Almanac::from(inp);
    //     let actual = almanac.get_lowest_seed_location_by_range(almanac.seeds_as_range());
    //     assert_eq!(actual, 46);
    // }
}
