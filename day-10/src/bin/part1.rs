mod shared;
use shared::{parser::parse_network, Path};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let network = parse_network(inp).unwrap().1;
    let starting_tile = network.get_starting_tile().unwrap();
    let connected_path = network.get_connected_paths(starting_tile, starting_tile, None, Path::default());
    println!("{}", get_furthest_point(&connected_path));
}

fn get_furthest_point(path: &Path) -> usize {
    path.len() / 2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_furthest_from_starting() {
        let inp = include_str!("../data/sample_input.txt");
        let network = parse_network(inp).unwrap().1;
        let starting_tile = network.get_starting_tile().unwrap();
        let connected_path = network.get_connected_paths(starting_tile, starting_tile, None, Path::default());
        assert_eq!(get_furthest_point(&connected_path), 4);
    }

    #[test]
    fn get_furthest_from_starting_sample_2() {
        let inp = include_str!("../data/sample_input2.txt");
        let network = parse_network(inp).unwrap().1;
        let starting_tile = network.get_starting_tile().unwrap();
        let connected_path = network.get_connected_paths(starting_tile, starting_tile, None, Path::default());
        assert_eq!(get_furthest_point(&connected_path), 8);
    }

}
