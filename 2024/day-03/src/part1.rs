use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, i32};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let muls = muls(input);
    dbg!("{:?}", &muls);
    Ok(muls.iter().map(|(a, b)| a * b).sum::<i32>().to_string())
}
pub fn muls(input: &str) -> Vec<(i32, i32)> {
    let candidates = input.split("mul");
    candidates
        .map(|e| mul_suffix(e))
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().1)
        .collect::<Vec<_>>()
}
pub fn find_mul(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(
        many0(anychar),
        delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")")),
    )(input)
}

pub fn mul_suffix(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(tag("("), separated_pair(i32, tag(","), i32), tag(")"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
