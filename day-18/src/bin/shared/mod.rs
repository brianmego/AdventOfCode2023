use aoc_utils::{Direction, Loc};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, newline, one_of},
    combinator::{map, map_res},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
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

    pub fn draw_polygon(&self) -> Polygon {
        let mut instructions = vec![];
        let mut current_loc = Loc::new(0, 0);
        self.0.iter().for_each(|i| {
            for _ in 0..i.count {
                instructions.push(current_loc);
                current_loc = current_loc.get_neighbor(i.direction).unwrap();
            }
        });
        Polygon(instructions)
    }
    pub fn fill_polygon(&self, polygon: Polygon) -> Polygon {
        let mut min_x = polygon.0[0].get_x();
        let mut min_y = polygon.0[0].get_y();
        let mut max_x = polygon.0[0].get_x();
        let mut max_y = polygon.0[0].get_y();
        for loc in polygon.0.iter() {
            min_x = loc.get_x().min(min_x);
            min_y = loc.get_y().min(min_y);
            max_x = loc.get_x().max(max_x);
            max_y = loc.get_y().max(max_y);
        }
        let mut inside_points = vec![];
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = Loc::new(x, y);
                let all_this_row: Vec<_> = polygon.0.iter().filter(|l| l.get_y() == y).collect();
                let all_this_column: Vec<_> = polygon.0.iter().filter(|l| l.get_x() == x).collect();
                let horizontal_line = Loc::new(max_x + 1, y);
                let horizontal_line_2 = Loc::new(min_x - 1, y);
                let vertical_line = Loc::new(x, max_y + 1);
                let vertical_line_2 = Loc::new(x, min_y - 1);
                let line_1 = point.connect_with_line(horizontal_line);
                let line_2 = point.connect_with_line(horizontal_line_2);
                let line_3 = point.connect_with_line(vertical_line);
                let line_4 = point.connect_with_line(vertical_line_2);
                let intersections_1 = line_1.iter().filter(|p| all_this_row.contains(p));
                let intersections_2 = line_2.iter().filter(|p| all_this_row.contains(p));
                let intersections_3 = line_3.iter().filter(|p| all_this_column.contains(p));
                let intersections_4 = line_4.iter().filter(|p| all_this_column.contains(p));
                match intersections_1.count() == all_this_row.len()
                    || intersections_2.count() == all_this_row.len()
                    || intersections_3.count() == all_this_column.len()
                    || intersections_4.count() == all_this_column.len()
                {
                    true => {}
                    false => inside_points.push(point),
                }
            }
        }
        Polygon(inside_points)
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    count: u8,
    color: Color,
}

impl Instruction {
    fn new(direction: Direction, count: u8, color: Color) -> Self {
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
            Instruction::new(direction, count.parse::<u8>().unwrap(), color)
        },
    )(inp)?;
    Ok((inp, instruction))
}

pub fn parse_instruction_set(inp: &str) -> IResult<&str, InstructionSet> {
    map(
        many1(terminated(parse_instruction, newline)),
        |instructions| InstructionSet(instructions),
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
        let actual = instruction_set.draw_polygon();
        assert_eq!(actual.len(), 38);
    }

    #[test]
    fn test_fill_polygon() {
        let inp = include_str!("../../data/sample_input.txt");
        let instruction_set = parse_instruction_set(inp).unwrap().1;
        assert_eq!(instruction_set.0.len(), 14);
        let outline = instruction_set.draw_polygon();
        dbg!(&outline);
        let mut actual = instruction_set.fill_polygon(outline);
        actual.0.sort();
        dbg!(&actual);
        assert_eq!(actual.len(), 62);
    }
}
