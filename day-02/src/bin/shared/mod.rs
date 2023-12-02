pub mod parser;
use std::str::FromStr;

pub struct GameConfig {
    pub max_blue: u32,
    pub max_red: u32,
    pub max_green: u32
}

impl GameConfig {
    pub fn new(max_blue: u32, max_red: u32, max_green: u32) -> Self { Self { max_blue, max_red, max_green } }
}
#[derive(Debug, PartialEq, Default)]
pub struct Round {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

#[derive(Debug, PartialEq, Default)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    fn new(id: u32, rounds: Vec<Round>) -> Self {
        Self { id, rounds }
    }

}

impl Round {
    fn new(blue: u32, red: u32, green: u32) -> Self {
        Self { blue, red, green }
    }
}
#[derive(Debug, PartialEq)]
enum BeadColor {
    Blue,
    Red,
    Green,
}
#[derive(Debug)]
struct BadBeadColor;
impl FromStr for BeadColor {
    type Err = BadBeadColor;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            _ => Err(BadBeadColor),
        }
    }
}
