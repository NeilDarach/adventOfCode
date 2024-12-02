use crate::custom_error::AocError;
use itertools::Itertools;
use nom::character::complete::{i32, newline, space1};
use nom::multi::separated_list1;
use nom::IResult;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(parse_input(input)
        .unwrap()
        .1
        .iter()
        .filter(|e| is_safe(e))
        .count()
        .to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, separated_list1(space1, i32))(input)
}

fn copy_without(vec: &[i32], index: usize) -> Vec<i32> {
    vec.iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, val)| *val)
        .collect::<Vec<_>>()
}

fn is_safe(vec: &[i32]) -> bool {
    if basic_is_safe(vec) {
        return true;
    }
    for i in 0..vec.len() {
        let short = copy_without(vec, i);
        if basic_is_safe(&short[..]) {
            return true;
        }
    }
    false
}
fn basic_is_safe(vec: &[i32]) -> bool {
    if vec.len() < 2 {
        return false;
    }
    let sign = (vec[0] - vec[1]).signum();
    if sign == 0 {
        return false;
    }
    vec.iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .all(|a| (a.abs() <= 3) && (a.signum() == sign))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
