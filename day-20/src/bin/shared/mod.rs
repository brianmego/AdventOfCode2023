use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char as nomchar, newline},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Signal {
    High,
    Low,
}

#[derive(Debug, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    Button,
}

#[derive(Debug, PartialEq)]
struct Module<'a> {
    module_type: ModuleType,
    name: &'a str,
    current_state: Signal,
    commands: VecDeque<Signal>,
    destination_modules: Vec<&'a str>,
}
impl<'a> Module<'a> {
    fn new(module_type: ModuleType, name: &'a str, destination_modules: Vec<&'a str>) -> Self {
        let current_state = Signal::Low;
        let commands = VecDeque::new();
        Self {
            module_type,
            name,
            current_state,
            commands,
            destination_modules,
        }
    }

    fn handle_next_command(&mut self) {
        match self.current_state {
            Signal::High => match self.commands.pop_front().unwrap() {
                Signal::High => todo!(),
                Signal::Low => todo!(),
            },
            Signal::Low => match self.commands.pop_front().unwrap() {
                Signal::High => todo!(),
                Signal::Low => todo!(),
            },
        }
    }
}

fn parse_broadcaster(inp: &str) -> IResult<&str, Module> {
    map(
        tuple((
            terminated(tag("broadcaster"), tag(" -> ")),
            many1(terminated(alpha1, opt(tag(", ")))),
        )),
        |(name, destination_modules)| Module::new(ModuleType::Broadcast, name, destination_modules),
    )(inp)
}
fn parse_flip_flop(inp: &str) -> IResult<&str, Module> {
    map(
        tuple((
            delimited(nomchar('%'), alpha1, tag(" -> ")),
            many1(terminated(alpha1, opt(tag(", ")))),
        )),
        |(name, destination_modules)| Module::new(ModuleType::FlipFlop, name, destination_modules),
    )(inp)
}
fn parse_conjunction(inp: &str) -> IResult<&str, Module> {
    map(
        tuple((
            delimited(nomchar('&'), alpha1, tag(" -> ")),
            many1(terminated(alpha1, opt(tag(", ")))),
        )),
        |(name, destination_modules)| Module::new(ModuleType::Conjunction, name, destination_modules),
    )(inp)
}
pub fn parse_configuration(inp: &str) -> IResult<&str, Vec<Module>> {
    many1(
        terminated(alt((
            parse_broadcaster,
            parse_flip_flop,
            parse_conjunction,
        )), newline)
    )(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse_broadcaster() {
        let inp = "broadcaster -> a, b, c";
        let actual = parse_broadcaster(inp);
        let expected = Ok((
            "",
            Module::new(ModuleType::Broadcast, "broadcaster", vec!["a", "b", "c"]),
        ));
        assert_eq!(actual, expected);
    }

    #[test_case("%a -> b", Ok(("", Module::new(ModuleType::FlipFlop, "a", vec!["b"]))))]
    #[test_case("%b -> c", Ok(("", Module::new(ModuleType::FlipFlop, "b", vec!["c"]))))]
    #[test_case("%c -> inv", Ok(("", Module::new(ModuleType::FlipFlop, "c", vec!["inv"]))))]
    fn test_flip_flop(inp: &str, exp: IResult<&str, Module>) {
        let actual = parse_flip_flop(inp);
        assert_eq!(actual, exp);
    }

    #[test_case("&inv -> a", Ok(("", Module::new(ModuleType::Conjunction, "inv", vec!["a"]))))]
    fn test_conjunction(inp: &str, exp: IResult<&str, Module>) {
        let actual = parse_conjunction(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_configuration() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual = parse_configuration(inp).unwrap().1;
        assert_eq!(actual.len(), 5);
    }
}
