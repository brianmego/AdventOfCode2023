mod shared;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let out = margin_of_error(parse_inp(inp));
    println!("{}", out);
}

struct Race {
    total_time: u32,
    record_distance: u32,
}

impl Race {
    fn new(total_time: u32, record_distance: u32) -> Self {
        Self {
            total_time,
            record_distance,
        }
    }
}

fn parse_inp(inp: &str) -> Vec<Race> {
    let mut times: Vec<u32> = vec![];
    let mut record_distances: Vec<u32> = vec![];
    for line in inp.lines() {
        if line.starts_with("Time:") {
            let mut iter = line.split_whitespace();
            iter.next();
            iter.for_each(|t| times.push(t.parse::<u32>().unwrap()));
        }
        if line.starts_with("Distance:") {
            let mut iter = line.split_whitespace();
            iter.next();
            iter.for_each(|d| record_distances.push(d.parse::<u32>().unwrap()));
        }
    }
    let zipper = times.iter().zip(record_distances);
    zipper
        .map(|(time, distance)| Race::new(*time, distance))
        .collect()
}
fn margin_of_error(races: Vec<Race>) -> u32 {
    races
        .iter()
        .map(|race| winning_strategies(race.total_time, race.record_distance))
        .product()
}
fn winning_strategies(time: u32, record_distance: u32) -> u32 {
    calculate_distance(1, time);
    calculate_distance(2, time);
    calculate_distance(3, time);
    (1..time)
        .filter(|speed| calculate_distance(*speed, time) > record_distance)
        .count() as u32
}
fn calculate_distance(speed: u32, total_time: u32) -> u32 {
    speed * (total_time - speed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0, 7, 0)]
    #[test_case(1, 7, 6)]
    #[test_case(2, 7, 10)]
    #[test_case(3, 7, 12)]
    #[test_case(4, 7, 12)]
    #[test_case(5, 7, 10)]
    #[test_case(6, 7, 6)]
    #[test_case(7, 7, 0)]
    fn test_calculate_distance(speed: u32, total_time: u32, exp: u32) {
        let actual = calculate_distance(speed, total_time);
        assert_eq!(actual, exp);
    }

    #[test_case(7, 9, 4)]
    #[test_case(15, 40, 8)]
    #[test_case(30, 200, 9)]
    fn test_winning_strategies(time: u32, record_distance: u32, exp: u32) {
        let actual = winning_strategies(time, record_distance);
        assert_eq!(actual, exp);
    }

    #[test_case(vec![
        Race::new(7, 9),
        Race::new(15, 40),
        Race::new(30, 200),
    ])]
    fn test_calc_margin_of_error(races: Vec<Race>) {
        let actual = margin_of_error(races);
        assert_eq!(actual, 288);
    }

    #[test]
    fn test_sample_input() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = margin_of_error(parse_inp(inp));
        assert_eq!(actual, 288);
    }
}
