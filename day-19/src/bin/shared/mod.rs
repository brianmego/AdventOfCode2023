use nom::{
    character::complete::{alpha1, char as nomchar, digit1, one_of},
    combinator::map,
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Operation<'a> {
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
struct Workflow<'a> {
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
}
