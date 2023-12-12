mod shared;
use shared::parser::parse_universe;
use shared::Universe;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let mut universe = parse_universe(inp).unwrap().1;
    universe.expand(1000000);
    let actual = get_all_galaxy_distances(universe);
    let sum: usize = actual.iter().sum();
    println!("{}", sum);
}

fn get_all_galaxy_distances(universe: Universe) -> Vec<usize> {
    let galaxies = universe.get_all_galaxies();
    let mut galaxies_copy = galaxies.clone();
    let mut distances: Vec<usize> = vec![];
    for galaxy_1 in &galaxies {
        for galaxy_2 in &mut galaxies_copy {
            match galaxy_1 == galaxy_2 {
                true => (),
                false => distances.push(galaxy_1.calculate_distance(galaxy_2)),
            }
        }
        galaxies_copy.remove(0);
    }
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_galaxy_distances() {
        let inp = include_str!("../data/sample_input.txt");
        let mut universe = parse_universe(inp).unwrap().1;
        universe.expand(10);
        let actual = get_all_galaxy_distances(universe);
        let sum: usize = actual.iter().sum();
        assert_eq!(actual.len(), 36);
        assert_eq!(sum, 1030);
    }
    #[test]
    fn test_get_all_galaxy_distances_100() {
        let inp = include_str!("../data/sample_input.txt");
        let mut universe = parse_universe(inp).unwrap().1;
        universe.expand(100);
        let actual = get_all_galaxy_distances(universe);
        let sum: usize = actual.iter().sum();
        assert_eq!(actual.len(), 36);
        assert_eq!(sum, 8410);
    }

}
