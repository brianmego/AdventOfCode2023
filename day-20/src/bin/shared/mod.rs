use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char as nomchar, newline},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::fmt::Debug;
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
enum FlipFlopState {
    On,
    Off,
}
#[derive(Debug, PartialEq)]
struct FlipFlop {
    name: String,
    current_state: FlipFlopState,
    commands: VecDeque<Signal>,
    destination_modules: Vec<String>,
}

impl FlipFlop {
    fn new(name: &str, destination_modules: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            current_state: FlipFlopState::Off,
            commands: VecDeque::new(),
            destination_modules: destination_modules.iter().map(|s| s.to_string()).collect(),
        }
    }
}
#[derive(Default, Debug, PartialEq)]
struct Broadcaster {
    destination_modules: Vec<String>,
}

impl Broadcaster {
    fn new(destination_modules: Vec<&str>) -> Self {
        Self {
            destination_modules: destination_modules.iter().map(|s| s.to_string()).collect(),
        }
    }
}
#[derive(Debug, PartialEq)]
struct Conjunction {
    name: String,
    commands: VecDeque<Signal>,
    last_command: Option<Signal>,
    destination_modules: Vec<String>,
}

impl Conjunction {
    fn new(name: &str, destination_modules: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            commands: VecDeque::new(),
            last_command: None,
            destination_modules: destination_modules.iter().map(|s| s.to_string()).collect(),
        }
    }
}

trait Module {
    fn queue_command(&mut self, signal: Signal) {
        self.commands().push_back(signal);
    }
    fn commands(&mut self) -> VecDeque<Signal>;
}
impl Module for FlipFlop {
    fn commands(&mut self) -> VecDeque<Signal> {
        self.commands
    }
}
impl Module for Conjunction {
    fn commands(&mut self) -> VecDeque<Signal> {
        self.commands
    }
}
impl Module for Broadcaster {
    fn commands(&mut self) -> VecDeque<Signal> {
        todo!()
    }
}
// #[derive(Debug, PartialEq)]
// struct Module<'a> {
//     module_type: ModuleType,
//     name: &'a str,
//     current_state: Signal,
//     commands: VecDeque<Signal>,
//     destination_modules: Vec<&'a str>,
// }
// impl<'a> Module<'a> {
//     fn new(module_type: ModuleType, name: &'a str, destination_modules: Vec<&'a str>) -> Self {
//         let current_state = Signal::Low;
//         let commands = VecDeque::new();
//         Self {
//             module_type,
//             name,
//             current_state,
//             commands,
//             destination_modules,
//         }
//     }

//     fn queue_command(&mut self, signal: Signal) {
//         self.commands.push_back(signal);
//     }

//     fn handle_next_command(&mut self, mut modules: Vec<Module>) {
//         match self.module_type {
//             ModuleType::Broadcast => {
//                 self.destination_modules.iter().for_each(|name| {
//                     let target_module = modules.iter_mut().find(|m|&m.name == name).unwrap();
//                     target_module.queue_command(Signal::Low);
//                 })
//             },
//             ModuleType::FlipFlop => {

//             },
//             ModuleType::Conjunction => todo!(),
//             ModuleType::Button => todo!(),
//         }
//         match self.current_state {
//             Signal::High => match self.commands.pop_front().unwrap() {
//                 Signal::High => todo!(),
//                 Signal::Low => todo!(),
//             },
//             Signal::Low => match self.commands.pop_front().unwrap() {
//                 Signal::High => todo!(),
//                 Signal::Low => todo!(),
//             },
//         }
//     }
// }

fn parse_broadcaster(inp: &str) -> IResult<&str, impl Module + Debug + PartialEq> {
    map(
        tuple((
            terminated(tag("broadcaster"), tag(" -> ")),
            many1(terminated(alpha1, opt(tag(", ")))),
        )),
        |(name, destination_modules)| Broadcaster::default(),
    )(inp)
}
fn parse_flip_flop(inp: &str) -> IResult<&str, impl Module + Debug + PartialEq> {
    map(
        tuple((
            delimited(nomchar('%'), alpha1, tag(" -> ")),
            many1(terminated(alpha1, opt(tag(", ")))),
        )),
        |(name, destination_modules): (&str, Vec<&str>)| FlipFlop::new(name, destination_modules),
    )(inp)
}
fn parse_conjunction(inp: &str) -> IResult<&str, impl Module + Debug + PartialEq> {
    map(
        tuple((
            delimited(nomchar('&'), alpha1, tag(" -> ")),
            many1(terminated(alpha1, opt(tag(", ")))),
        )),
        |(name, destination_modules)| Conjunction::new(name, destination_modules),
    )(inp)
}
pub fn parse_configuration(inp: &str) -> IResult<&str, Vec<impl Module>> {
    many1(terminated(
        alt((parse_broadcaster, parse_flip_flop, parse_conjunction)),
        newline,
    ))(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse_broadcaster() {
        let inp = "broadcaster -> a, b, c";
        let actual = parse_broadcaster(inp);
        let expected = Ok(("", Broadcaster::new(vec!["a", "b", "c"])));
        assert_eq!(actual, expected);
    }

    #[test_case("%a -> b", Ok(("", FlipFlop::new("a", vec!["b"]))))]
    #[test_case("%b -> c", Ok(("", FlipFlop::new("b", vec!["c"]))))]
    #[test_case("%c -> inv", Ok(("", FlipFlop::new("c", vec!["inv"]))))]
    fn test_flip_flop(inp: &str, exp: IResult<&str, FlipFlop>) {
        let actual = parse_flip_flop(inp);
        assert_eq!(actual, exp);
    }

    #[test_case("&inv -> a", Ok(("", Conjunction::new("inv", vec!["a"]))))]
    fn test_conjunction(inp: &str, exp: IResult<&str, Conjunction>) {
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
