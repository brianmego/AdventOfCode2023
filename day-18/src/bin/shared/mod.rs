use aoc_utils::{Direction, Loc};
use nom::{
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{digit1, hex_digit1, newline, one_of},
    combinator::{map, map_res},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

pub struct Polygon(Vec<Loc>);
impl Polygon {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get area using the Shoelace formula (https://en.wikipedia.org/wiki/Shoelace_formula)
    pub fn get_area(&self) -> usize {
        let mut sum: isize = 0;
        for i in 0..self.len() - 1 {
            let point_1 = self.0[i];
            let point_2 = self.0[i + 1];
            sum += (point_1.get_x() * point_2.get_y()) - (point_2.get_x() * point_1.get_y());
        }
        let first = self.0[0];
        let last = self.0.last().unwrap();
        sum += (last.get_x() * first.get_y()) - (first.get_x() * last.get_y());
        (sum / 2) as usize
    }
}
impl std::fmt::Debug for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_x = self.0[0].get_x();
        let mut min_y = self.0[0].get_y();
        let mut max_x = self.0[0].get_x();
        let mut max_y = self.0[0].get_y();
        for loc in self.0.iter() {
            min_x = loc.get_x().min(min_x);
            min_y = loc.get_y().min(min_y);
            max_x = loc.get_x().max(max_x);
            max_y = loc.get_y().max(max_y);
        }

        let mut out = String::new();
        out.push('\n');
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.0.contains(&Loc::new(x, y)) {
                    true => out.push('#'),
                    false => out.push('.'),
                }
            }
            out.push('\n');
        }
        f.write_str(&out)
    }
}

pub struct InstructionSet(Vec<Instruction>);
impl InstructionSet {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_perimiter(&self) -> usize {
        self.0.iter().map(|i| i.count).sum()
    }

    pub fn draw_polygon(&self) -> Polygon {
        let mut instructions = vec![];
        let mut current_loc = Loc::new(0, 0);
        self.0.iter().for_each(|i| {
            current_loc = current_loc.get_nearby(i.direction, i.count as isize);
            instructions.push(current_loc);
        });
        Polygon(instructions)
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    count: usize,
    color: Color,
}

impl Instruction {
    fn new(direction: Direction, count: usize, color: Color) -> Self {
        Self {
            direction,
            count,
            color,
        }
    }
}

fn parse_instruction(inp: &str) -> IResult<&str, Instruction> {
    let (inp, instruction) = map(
        tuple((
            terminated(one_of("UDLR"), tag(" ")),
            terminated(digit1, tag(" ")),
            delimited(tag("("), hex_color, tag(")")),
        )),
        |(direction, count, color)| {
            let direction = match direction {
                'U' => Direction::North,
                'D' => Direction::South,
                'L' => Direction::West,
                'R' => Direction::East,
                _ => unreachable!(),
            };
            Instruction::new(direction, count.parse::<usize>().unwrap(), color)
        },
    )(inp)?;
    Ok((inp, instruction))
}

fn parse_true_instruction(inp: &str) -> IResult<&str, Instruction> {
    let (inp, (_, _, hex)) = tuple((
        terminated(one_of("UDLR"), tag(" ")),
        terminated(digit1, tag(" ")),
        delimited(tag("(#"), hex_digit1, tag(")")),
    ))(inp)?;
    // |(direction, count, hex)| {
    let direction = match hex.chars().last().unwrap() {
        '0' => Direction::East,
        '1' => Direction::South,
        '2' => Direction::West,
        '3' => Direction::North,
        _ => unreachable!(),
    };

    let count = usize::from_str_radix(&hex[..5], 16).unwrap();
    let instruction = Instruction::new(direction, count, Color::default());
    Ok((inp, instruction))
}

pub fn parse_instruction_set(inp: &str) -> IResult<&str, InstructionSet> {
    map(
        many1(terminated(parse_instruction, newline)),
        InstructionSet,
    )(inp)
}

pub fn parse_true_instruction_set(inp: &str) -> IResult<&str, InstructionSet> {
    map(
        many1(terminated(parse_true_instruction, newline)),
        InstructionSet,
    )(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("R 6 (#2f14df)", Ok(("", Instruction::new(Direction::East, 6, Color{red: 47, green: 20, blue: 223}))); "Line1")]
    #[test_case("D 2 (#d2c081)", Ok(("", Instruction::new(Direction::South, 2, Color{red: 210, green: 192, blue: 129}))); "Line2")]
    fn test_parse_instruction(inp: &str, exp: IResult<&str, Instruction>) {
        let actual = parse_instruction(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_instruction_set() {
        let inp = include_str!("../../data/sample_input.txt");
        let instruction_set = parse_instruction_set(inp).unwrap().1;
        assert_eq!(instruction_set.0.len(), 14);
    }

    #[test]
    fn test_get_area() {
        let inp = include_str!("../../data/sample_input.txt");
        let instruction_set = parse_instruction_set(inp).unwrap().1;
        assert_eq!(instruction_set.0.len(), 14);
        let outline = instruction_set.draw_polygon();
        let perimiter = instruction_set.get_perimiter();
        let actual = outline.get_area() + (perimiter / 2) + 1;
        assert_eq!(actual, 62);
    }

    #[test]
    fn test_parse_true_instruction() {
        let inp = "R 6 (#70c710)";
        let actual = parse_true_instruction(inp);
        assert_eq!(
            actual,
            Ok((
                "",
                Instruction::new(Direction::East, 461937, Color::default())
            ))
        );
    }
}
