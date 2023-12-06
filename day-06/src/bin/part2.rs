mod shared;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let out = margin_of_error(parse_inp(inp));
    println!("{}", out);
}

struct Race {
    total_time: usize,
    record_distance: usize,
}

impl Race {
    fn new(total_time: usize, record_distance: usize) -> Self {
        Self {
            total_time,
            record_distance,
        }
    }
}

fn parse_inp(inp: &str) -> Vec<Race> {
    let mut times: Vec<&str> = vec![];
    let mut record_distances: Vec<&str> = vec![];
    for line in inp.lines() {
        if line.starts_with("Time:") {
            let mut iter = line.split_whitespace();
            iter.next();
            iter.for_each(|t| times.push(t));
        }
        if line.starts_with("Distance:") {
            let mut iter = line.split_whitespace();
            iter.next();
            iter.for_each(|d| record_distances.push(d));
        }
    }
    let time = times.join("").parse::<usize>().unwrap();
    let distance = record_distances.join("").parse::<usize>().unwrap();
    vec![Race::new(time, distance)]
}
fn margin_of_error(races: Vec<Race>) -> usize {
    races
        .iter()
        .map(|race| winning_strategies(race.total_time, race.record_distance))
        .product()
}
fn winning_strategies(time: usize, record_distance: usize) -> usize {
    (1..time)
        .filter(|speed| calculate_distance(*speed, time) > record_distance)
        .count()
}
fn calculate_distance(speed: usize, total_time: usize) -> usize {
    speed * (total_time - speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = margin_of_error(parse_inp(inp));
        assert_eq!(actual, 71503);
    }
}
