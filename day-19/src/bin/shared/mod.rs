use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, char as nomchar, digit1, newline, one_of},
    combinator::map,
    multi::{many0, many1, many_m_n},
    sequence::{delimited, preceded, terminated, tuple, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Operation<'a> {
    Accepted,
    Rejected,
    Run(&'a str),
}
impl<'a> From<&'a str> for Operation<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            dest_workflow => Self::Run(dest_workflow),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    pub fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self { x, m, a, s }
    }

    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    pub fn validate_rating(&self, workflows: &Vec<Workflow>, active_workflow: &str) -> Operation {
        let active_workflow = workflows
            .iter()
            .find(|w| w.name == active_workflow)
            .unwrap();
        for rule in &active_workflow.rules {
            let rule_matches = self.rule_matches(rule);
            match rule_matches {
                true => match rule.operation {
                    Operation::Accepted => {
                        return Operation::Accepted;
                    }
                    Operation::Rejected => {
                        return Operation::Rejected;
                    }
                    Operation::Run(workflow_name) => {
                        return self.validate_rating(workflows, workflow_name);
                    }
                },
                false => continue,
            }
        }
        match active_workflow.fallthrough_op {
            Operation::Accepted => Operation::Accepted,
            Operation::Rejected => Operation::Rejected,
            Operation::Run(workflow_name) => self.validate_rating(workflows, workflow_name),
        }
    }

    fn rule_matches(&self, rule: &Rule<'_>) -> bool {
        match rule.comparison {
            Comparison::GreaterThan => match rule.category {
                Category::X => self.x > rule.value,
                Category::M => self.m > rule.value,
                Category::A => self.a > rule.value,
                Category::S => self.s > rule.value,
            },
            Comparison::LessThan => match rule.category {
                Category::X => self.x < rule.value,
                Category::M => self.m < rule.value,
                Category::A => self.a < rule.value,
                Category::S => self.s < rule.value,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}
impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, PartialEq)]
enum Comparison {
    GreaterThan,
    LessThan,
}

impl From<char> for Comparison {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, PartialEq)]
struct Rule<'a> {
    category: Category,
    comparison: Comparison,
    value: usize,
    operation: Operation<'a>,
}

impl<'a> Rule<'a> {
    fn new(
        category: Category,
        comparison: Comparison,
        value: usize,
        operation: Operation<'a>,
    ) -> Self {
        Self {
            category,
            comparison,
            value,
            operation,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallthrough_op: Operation<'a>,
}

impl<'a> Workflow<'a> {
    fn new(name: &'a str, rules: Vec<Rule<'a>>, fallthrough_op: Operation<'a>) -> Self {
        Self {
            name,
            rules,
            fallthrough_op,
        }
    }
}

fn parse_workflow(inp: &str) -> IResult<&str, Workflow> {
    map(
        tuple((
            alpha1,
            delimited(
                nomchar('{'),
                tuple((
                    many1(map(
                        terminated(
                            tuple((
                                one_of("amsx"),
                                one_of("<>"),
                                digit1,
                                preceded(nomchar(':'), alpha1),
                            )),
                            nomchar(','),
                        ),
                        |(category, comparison, value, operation): (char, char, &str, &str)| {
                            let category = Category::from(category);
                            let comparison = Comparison::from(comparison);
                            let value = value.parse::<usize>().unwrap();
                            let operation = Operation::from(operation);
                            Rule::new(category, comparison, value, operation)
                        },
                    )),
                    alpha1,
                )),
                nomchar('}'),
            ),
        )),
        |(name, (rules, fallthrough_op))| {
            Workflow::new(name, rules, Operation::from(fallthrough_op))
        },
    )(inp)
}

fn parse_workflow_set(inp: &str) -> IResult<&str, Vec<Workflow>> {
    many1(terminated(parse_workflow, many0(newline)))(inp)
}

fn parse_rating(inp: &str) -> IResult<&str, Rating> {
    map(
        delimited(
            nomchar('{'),
            tuple((
                delimited(tag("x="), digit1, tag(",")),
                delimited(tag("m="), digit1, tag(",")),
                delimited(tag("a="), digit1, tag(",")),
                preceded(tag("s="), digit1),
            )),
            nomchar('}'),
        ),
        |(x, m, a, s): (&str, &str, &str, &str)| {
            Rating::new(
                x.parse().unwrap(),
                m.parse().unwrap(),
                a.parse().unwrap(),
                s.parse().unwrap(),
            )
        },
    )(inp)
}

fn parse_rating_set(inp: &str) -> IResult<&str, Vec<Rating>> {
    many1(terminated(parse_rating, many0(newline)))(inp)
}

pub fn parse_workflows_and_ratings(inp: &str) -> IResult<&str, (Vec<Workflow>, Vec<Rating>)> {
    separated_pair(parse_workflow_set, many0(newline), parse_rating_set)(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("px{a<2006:qkq,m>2090:A,rfg}", Ok(("",
        Workflow::new(
            "px",
            vec![
                Rule::new(Category::A, Comparison::LessThan, 2006, Operation::Run("qkq")),
                Rule::new(Category::M, Comparison::GreaterThan, 2090, Operation::Accepted)
            ],
            Operation::Run("rfg")
        )
    )); "Workflow1")]
    fn test_parse_workflow(inp: &str, exp: IResult<&str, Workflow>) {
        let actual = parse_workflow(inp);
        assert_eq!(actual, exp);
    }

    #[test_case("{x=787,m=2655,a=1222,s=2876}", Ok(("",
        Rating::new(787, 2655, 1222, 2876)
    )); "Rating1")]
    fn test_parse_rating(inp: &str, exp: IResult<&str, Rating>) {
        let actual = parse_rating(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_all_parsers() {
        let inp = include_str!("../../data/sample_input.txt");
        let parsed = parse_workflows_and_ratings(inp).unwrap();
        let workflows = parsed.1.0;
        let ratings = parsed.1.1;
        assert_eq!(workflows.len(), 11);
        assert_eq!(ratings.len(), 5);
    }
}
