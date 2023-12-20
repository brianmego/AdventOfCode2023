mod shared;
use shared::{parse_workflows_and_ratings, Operation, Workflow, Rating};

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let (_, (workflows, ratings)) = parse_workflows_and_ratings(inp).unwrap();
    let sum = rate_all_workflows(workflows, ratings);
    println!("{}", sum);
}

fn rate_all_workflows(workflows: Vec<Workflow>, ratings: Vec<Rating>) -> usize{
    let mut sum = 0;
    for rating in ratings {
        if rating.validate_rating(&workflows, "in") == Operation::Accepted {
            sum += rating.sum()
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_all_workflows() {
        let inp = include_str!("../data/sample_input.txt");
        let (_, (workflows, ratings)) = parse_workflows_and_ratings(inp).unwrap();
        let sum = rate_all_workflows(workflows, ratings);
        assert_eq!(sum, 19114);
    }
}
