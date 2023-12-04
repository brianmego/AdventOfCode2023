use nom::{
    branch::alt,
    character::complete::{char, digit1, one_of},
    multi::{many1, many1_count},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub row: i32,
    pub start_col: i32,
    pub end_col: i32,
}

impl Location {
    fn new(row: i32, start_col: i32, end_col: i32) -> Self {
        Self {
            row,
            start_col,
            end_col,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnginePart {
    Symbol(char),
    Part(i32),
    Empty(i32),
}
impl EnginePart {
    pub fn get_value(&self) -> i32 {
        match self {
            EnginePart::Symbol(_) => 0,
            EnginePart::Part(p) => *p,
            EnginePart::Empty(_) => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnginePOI {
    pub location: Location,
    pub part: EnginePart,
}

impl EnginePOI {
    fn new(location: Location, value: EnginePart) -> Self {
        Self {
            location,
            part: value,
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct EngineSchematic {
    pub parts: Vec<EnginePOI>,
    pub symbols: Vec<EnginePOI>,
}
impl EngineSchematic {
    fn add_part(&mut self, part: EnginePOI) {
        self.parts.push(part);
    }
    fn add_symbol(&mut self, part: EnginePOI) {
        self.symbols.push(part);
    }
}

pub fn parse_engine_schematic(inp: &str) -> IResult<&str, EngineSchematic> {
    let mut schematic = EngineSchematic::default();
    for (row, line) in inp.lines().enumerate() {
        let (_, line_poi_list) = parse_line(line, row as i32)?;
        let _: Vec<_> = line_poi_list
            .iter()
            .map(|poi| match poi.part {
                EnginePart::Symbol(_) => {
                    schematic.add_symbol(poi.clone());
                }
                EnginePart::Part(_) => {
                    schematic.add_part(poi.clone());
                }
                EnginePart::Empty(_) => {}
            })
            .collect();
    }
    Ok(("", schematic))
}

fn parse_line(inp: &str, row: i32) -> IResult<&str, Vec<EnginePOI>> {
    let (inp, parts) = many1(alt((parse_spaces, parse_symbol, parse_part)))(inp)?;
    let mut start_loc: i32 = 0;

    let list_of_poi: Vec<EnginePOI> = parts
        .iter()
        .map(|part| match part {
            EnginePart::Symbol(c) => {
                let len = 1;
                let poi = EnginePOI::new(
                    Location::new(row, start_loc, start_loc + len),
                    EnginePart::Symbol(*c),
                );
                start_loc += len;
                poi
            }
            EnginePart::Part(p) => {
                let len = (p.checked_ilog10().unwrap() + 1) as i32;
                let poi = EnginePOI::new(
                    Location::new(row, start_loc, start_loc + len),
                    EnginePart::Part(*p),
                );
                start_loc += len;
                poi
            }
            EnginePart::Empty(periods) => {
                let len = periods;
                let poi = EnginePOI::new(
                    Location::new(row, start_loc, start_loc + len),
                    EnginePart::Empty(*periods),
                );
                start_loc += len;
                poi
            }
        })
        .collect();
    Ok((inp, list_of_poi))
}

fn parse_spaces(inp: &str) -> IResult<&str, EnginePart> {
    let (inp, count) = many1_count(char('.'))(inp)?;
    Ok((inp, EnginePart::Empty(count as i32)))
}

fn parse_symbol(inp: &str) -> IResult<&str, EnginePart> {
    let (inp, symbol) = one_of("-=!@#$%^&*({})_+?><:;',\"\\/")(inp)?;
    Ok((inp, EnginePart::Symbol(symbol)))
}
fn parse_part(inp: &str) -> IResult<&str, EnginePart> {
    let (inp, part) = digit1(inp)?;
    Ok((inp, EnginePart::Part(part.parse().unwrap())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("&", ("", EnginePart::Symbol('&')); "ampersand")]
    #[test_case("^", ("", EnginePart::Symbol('^')); "caret")]
    fn test_parse_symbol(inp: &str, exp: (&str, EnginePart)) {
        let actual = parse_symbol(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test_case("l"; "l")]
    #[test_case("."; "period")]
    fn test_parse_symbol_error(inp: &str) {
        let actual = parse_symbol(inp);
        assert!(actual.is_err());
    }

    #[test_case("123", ("", EnginePart::Part(123)))]
    #[test_case("8", ("", EnginePart::Part(8)))]
    fn test_parse_part(inp: &str, exp: (&str, EnginePart)) {
        let actual = parse_part(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test_case(".", ("", EnginePart::Empty(1)); "one")]
    #[test_case("...", ("", EnginePart::Empty(3)); "three")]
    fn test_count_periods(inp: &str, exp: (&str, EnginePart)) {
        let actual = parse_spaces(inp);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test_case("...*....22", ("", vec![
        EnginePOI::new(Location::new(1, 0, 3), EnginePart::Empty(3)),
        EnginePOI::new(Location::new(1, 3, 4), EnginePart::Symbol('*')),
        EnginePOI::new(Location::new(1, 4, 8), EnginePart::Empty(4)),
        EnginePOI::new(Location::new(1, 8, 10), EnginePart::Part(22)),
    ]); "asterisk")]
    fn test_parse_line(inp: &str, exp: (&str, Vec<EnginePOI>)) {
        let actual = parse_line(inp, 1);
        let expected = Ok(exp);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_engine_schematic() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual = parse_engine_schematic(inp);
        assert!(actual.is_ok());
        let actual = actual.unwrap().1;
        assert_eq!(actual.parts.len(), 10);
        assert_eq!(actual.symbols.len(), 6);
    }

    #[test]
    fn test_get_valid_parts() {
        let inp = include_str!("../../data/sample_input.txt");
        let schematic = parse_engine_schematic(inp).unwrap().1;
        let actual = schematic.get_valid_parts();
        assert_eq!(actual.len(), 8);
    }

    #[test]
    fn test_get_gears() {
        let inp = include_str!("../../data/sample_input.txt");
        let schematic = parse_engine_schematic(inp).unwrap().1;
        let actual = schematic.get_gears();
        dbg!(&actual);
        assert_eq!(actual.len(), 2);
    }
}
