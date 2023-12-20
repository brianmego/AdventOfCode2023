mod shared;
use shared::{parse_workflows_and_ratings, Operation, Workflow, Rating};

fn main() {
    let inp = include_str!("../data/sample_input.txt");
    let (_, (workflows, ratings)) = parse_workflows_and_ratings(inp).unwrap();
    let sum = rate_all_workflows(workflows);
    println!("{}", sum);

}

fn rate_all_workflows(workflows: Vec<Workflow>) -> usize{
    let mut sum = 0;
    for x in 0..4000 {
        for m in 0..4000 {
            for a in 0..4000 {
                for s in 0..4000 {
                    let rating = Rating::new(x, m, a, s);
                    if rating.validate_rating(&workflows, "in") == Operation::Accepted {
                        sum += rating.sum()
                    }
        // dbg!(s);
                }
        // dbg!(a);
            }
        dbg!(m);
        }
        dbg!(x);
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
        let sum = rate_all_workflows(workflows);
        assert_eq!(sum, 167409079868000);
    }
}
