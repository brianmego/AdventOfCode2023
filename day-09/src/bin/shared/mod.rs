pub fn get_next_in_pattern(row: Vec<i32>) -> i32 {
    let derivative = get_next_derivative(&row);
    let derivative_last: i32;
    if derivative.iter().all(|i| i == &0) {
        derivative_last = 0;
    } else {
        derivative_last = get_next_in_pattern(derivative);
    }
    row.last().unwrap() + derivative_last
}
fn get_next_derivative(row: &Vec<i32>) -> Vec<i32> {
    let mut new_row = vec![];
    for i in 0..row.len() - 1 {
        let left = row[i];
        let right = row[i + 1];
        new_row.push(right - left);
    }
    new_row
}

pub fn parse_line_as_vec(line: &str) -> Vec<i32> {
    line.split(' ').map(|i| i.parse::<i32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3])]
    #[test_case(vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0])]
    #[test_case(vec![1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6])]
    #[test_case(vec![4, -1, -9, -7], vec![-5, -8, 2])]
    fn test_get_next_derivative(inp: Vec<i32>, exp: Vec<i32>) {
        let actual = get_next_derivative(&inp);
        assert_eq!(actual, exp);
    }

    #[test_case(vec![0, 0, 0], 0)]
    #[test_case(vec![1, 2, 3], 4)]
    #[test_case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[test_case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[test_case(vec![10, 13, 16, 21, 30, 45], 68)]
    fn test_get_next_in_pattern(inp: Vec<i32>, exp: i32) {
        let actual = get_next_in_pattern(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_line_as_vec() {
        let actual = parse_line_as_vec("4 -1 -9 -7");
        let expected = vec![4, -1, -9, -7];
        assert_eq!(actual, expected);
    }
}
