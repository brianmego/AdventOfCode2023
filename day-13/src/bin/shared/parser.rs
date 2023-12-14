use crate::shared::{Collection, CollectionGroup, Loc, Tile, TileType};
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, many0},
    sequence::terminated,
    IResult,
};

fn parse_tile_type(inp: &str) -> IResult<&str, TileType> {
    parse_tile_type_generic(inp, "#.")
}

fn parse_tile_type_generic<'a>(
    inp: &'a str,
    valid_tile_chars: &'a str,
) -> IResult<&'a str, TileType> {
    map(one_of(valid_tile_chars), |c| {
        TileType::try_from(&c).unwrap()
    })(inp)
}

pub fn parse_collection(inp: &str) -> IResult<&str, Collection> {
    let (inp, rows) = many1(terminated(many1(parse_tile_type), newline))(inp)?;
    let mut collection = Collection::default();
    for (row_num, row) in rows.iter().enumerate() {
        for (col_num, tile) in row.iter().enumerate() {
            collection.push(Tile::new(*tile, Loc::new(col_num, row_num)));
        }
    }
    Ok((inp, collection))
}

pub fn parse_collection_group(inp: &str) -> IResult<&str, CollectionGroup> {
    many1(terminated(parse_collection, many0(newline)))(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(".", Ok(("", TileType::Ash)); "Ash")]
    #[test_case("#", Ok(("", TileType::Rocks)); "Rocks")]
    fn test_tile_type(inp: &str, exp: IResult<&str, TileType>) {
        let actual = parse_tile_type(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_collection() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual = parse_collection(inp).unwrap().1;
        assert_eq!(actual.0.len(), 63);
    }

    #[test]
    fn test_parse_collection_group() {
        let inp = include_str!("../../data/sample_input.txt");
        let actual = parse_collection_group(inp);
        assert!(actual.is_ok());
        let unwrapped = actual.unwrap();
        assert_eq!(unwrapped.0, "");
        assert_eq!(unwrapped.1[0].0.len(), 63);
        assert_eq!(unwrapped.1[1].0.len(), 63);
    }
}
