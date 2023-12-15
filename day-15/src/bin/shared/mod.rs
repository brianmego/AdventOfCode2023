pub fn hash(inp: &str) -> usize {
    let mut current_value: usize = 0;

    inp.chars().for_each(|c| {
        if !c.is_whitespace() {
            current_value += c as usize;
            current_value *= 17;
            current_value %= 256;
        }
    });
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("HASH", 52)]
    #[test_case("rn=1", 30)]
    #[test_case("cm-", 253)]
    #[test_case("qp=3", 97)]
    #[test_case("cm=2", 47)]
    #[test_case("pc=4", 180)]
    #[test_case("ot=9", 9)]
    #[test_case("ab=5", 197)]
    #[test_case("pc-", 48)]
    #[test_case("pc=6", 214)]
    #[test_case("ot=7", 231)]
    #[test_case("rn", 0)]
    #[test_case("cm", 0)]
    #[test_case("qp", 1)]
    #[test_case("pc", 3)]
    fn test_hash(inp: &str, exp: usize) {
        let actual = hash(inp);
        assert_eq!(actual, exp);
    }
}
