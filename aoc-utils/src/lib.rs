use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct BadTileTypeError;

pub fn parse_tile_type<T>(inp: &str) -> IResult<&str, T>
where
    T: ParseableCharacters + TryFrom<char>,
    <T as TryFrom<char>>::Error: Debug,
{
    let valid_tile_chars = T::valid_chars().iter().join("");
    let res = map(one_of(valid_tile_chars.as_str()), |c| {
        T::try_from(c).unwrap()
    })(inp);
    res
}

pub fn parse_collection<T>(inp: &str) -> IResult<&str, Collection<T>>
where
    T: ParseableCharacters + TryFrom<char> + Copy,
    <T as TryFrom<char>>::Error: Debug,
{
    let (inp, rows) = many1(terminated(many1(parse_tile_type), newline))(inp)?;
    let mut collection = Collection(vec![]);
    for (row_num, row) in rows.iter().enumerate() {
        for (col_num, tile) in row.iter().enumerate() {
            collection.push(Tile::new(*tile, Loc::new(col_num, row_num)));
        }
    }
    Ok((inp, collection))
}

pub fn parse_collection_group<T>(inp: &str) -> IResult<&str, CollectionGroup<T>>
where
    T: ParseableCharacters + TryFrom<char> + Copy,
    <T as TryFrom<char>>::Error: Debug,
{
    many1(terminated(parse_collection, many0(newline)))(inp)
}

// MODELS
pub trait ParseableCharacters {
    fn valid_chars() -> Vec<char>;
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Debug)]
struct Row<'a, T>(Vec<&'a Tile<T>>);
impl<T> Display for Row<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = self.0.iter().map(|t| t.to_string()).join("");
        f.write_str(&out)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Collection<T>(Vec<Tile<T>>);
impl<T> Collection<T> {
    fn push(&mut self, tile: Tile<T>) {
        self.0.push(tile)
    }
    pub fn get_row(&self, row_num: usize) -> Row<T> {
        Row(self.0.iter().filter(|t| t.loc.y == row_num).collect())
    }
    pub fn get_column(&self, col_num: usize) -> Row<T> {
        Row(self.0.iter().filter(|t| t.loc.x == col_num).collect())
    }
    pub fn count_rows(&self) -> usize {
        self.0.iter().unique_by(|t| t.loc.y).count()
    }
    pub fn count_columns(&self) -> usize {
        self.0.iter().unique_by(|t| t.loc.x).count()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
pub type CollectionGroup<T> = Vec<Collection<T>>;

#[derive(Debug, Copy, Clone)]
struct Tile<T> {
    tile_type: T,
    loc: Loc,
}
impl<T> PartialEq for Tile<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.tile_type == other.tile_type
    }
}

impl<T> Tile<T> {
    fn new(tile_type: T, loc: Loc) -> Self {
        Self { tile_type, loc }
    }
}

impl<T> Display for Tile<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.tile_type.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum LavaTile {
        Ash,
        Rocks,
    }
    impl Display for LavaTile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                LavaTile::Ash => ".",
                LavaTile::Rocks => "#",
            })
        }
    }

    impl TryFrom<char> for LavaTile {
        type Error = BadTileTypeError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(Self::Ash),
                '#' => Ok(Self::Rocks),
                _ => unreachable!(),
            }
        }
    }
    impl ParseableCharacters for LavaTile {
        fn valid_chars() -> Vec<char> {
            vec!['#', '.']
        }
    }

    #[test_case(".", Ok(("", LavaTile::Ash)); "Ash")]
    #[test_case("#", Ok(("", LavaTile::Rocks)); "Rocks")]
    fn test_tile_type(inp: &str, exp: IResult<&str, LavaTile>) {
        let actual = parse_tile_type(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_collection() {
        let inp = include_str!("./data/lava_sample.txt");
        let actual: Collection<LavaTile> = parse_collection(inp).unwrap().1;
        assert_eq!(actual.0.len(), 63);
    }

    #[test]
    fn test_parse_collection_group() {
        let inp = include_str!("./data/lava_sample.txt");
        let actual = parse_collection_group::<LavaTile>(inp);
        assert!(actual.is_ok());
        let unwrapped = actual.unwrap();
        assert_eq!(unwrapped.0, "");
        assert_eq!(unwrapped.1[0].0.len(), 63);
        assert_eq!(unwrapped.1[1].0.len(), 63);
    }
}
