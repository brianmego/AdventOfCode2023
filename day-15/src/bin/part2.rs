use nom::{
    character::complete::{alpha1, one_of},
    combinator::opt,
    sequence::tuple,
    IResult,
};

mod shared;
use shared::hash;

fn main() {
    let inp = include_str!("../data/puzzle_input.txt");
    let actual = run_hashmap_algorithm(inp);
    println!("{}", actual);
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

enum Operation {
    Remove,
    Add(i32),
}

struct Command {
    label: String,
    operation: Operation,
}

#[derive(Default, Clone, Debug)]
struct LensBox(Vec<Lens>);
impl LensBox {
    fn remove_lens(&mut self, lens_label: String) {
        let existing = self.0.iter().position(|l| l.label == lens_label);
        if let Some(idx) = existing {
            self.0.remove(idx);
        }
    }

    fn add_lens(&mut self, lens: Lens) {
        let existing = self.0.iter().position(|l| l.label == lens.label);
        match existing {
            Some(idx) => {
                self.0[idx] = lens;
            }
            None => self.0.push(lens),
        }
    }
}

fn run_hashmap_algorithm(inp: &str) -> usize {
    let mut boxes = vec![LensBox(vec![]); 256];
    let commands: Vec<_> = inp.split(',').collect();
    for cmd_str in commands {
        let command = parse_command(cmd_str).unwrap().1;
        let box_num = hash(&command.label);
        let this_box = &mut boxes[box_num];
        match command.operation {
            Operation::Remove => {
                this_box.remove_lens(command.label);
            }
            Operation::Add(focal_length) => {
                this_box.add_lens(Lens {
                    label: command.label,
                    focal_length: focal_length as u8,
                });
            }
        }
    }
    let mut sum = 0;
    for (box_num, lens_box) in boxes.iter().enumerate() {
        for (lens_num, lens) in lens_box.0.iter().enumerate() {
            sum += (box_num + 1) * (lens_num + 1) * (lens.focal_length as usize)
        }
    }
    sum
}

fn parse_command(inp: &str) -> IResult<&str, Command> {
    let (inp, (label, op_code, focal_length)) =
        tuple((alpha1, one_of("-="), opt(one_of("0123456789"))))(inp)?;
    let operation = match op_code {
        '-' => Operation::Remove,
        '=' => Operation::Add(focal_length.unwrap().to_digit(10).unwrap() as i32),
        _ => unreachable!(),
    };
    Ok((
        inp,
        Command {
            label: label.to_string(),
            operation,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_hashmap_algorithm() {
        let inp = include_str!("../data/sample_input.txt");
        let actual = run_hashmap_algorithm(inp);
        assert_eq!(actual, 145);
    }
}
