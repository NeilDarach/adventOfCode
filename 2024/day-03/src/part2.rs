use crate::custom_error::AocError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::i32;
use nom::error::Error;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug)]
pub enum Parsed {
    Do,
    Dont,
    Mul(i32, i32),
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    //dbg!("{:?}", parsed(input));
    let mut switch = true;
    let mut val = 0;
    for item in parsed(input) {
        match item {
            Parsed::Mul(a, b) => {
                if switch {
                    val += a * b;
                }
            }
            Parsed::Do => switch = true,
            Parsed::Dont => switch = false,
        }
    }
    Ok(val.to_string())
}

pub fn parsed(input: &str) -> Vec<Parsed> {
    mulvec(input)
        .unwrap()
        .1
        .into_iter()
        .filter_map(|e| e)
        .collect::<Vec<_>>()
}
pub fn mulvec(input: &str) -> IResult<&str, Vec<Option<Parsed>>> {
    many1(mul)(input)
}

pub fn mul(input: &str) -> IResult<&str, Option<Parsed>> {
    let res = alt((
        tag::<&str, &str, Error<&str>>("mul("),
        tag("do()"),
        tag("don't()"),
    ))(input);
    if !res.is_ok() {
        let (input, _) = anychar(input)?;
        return Ok((input, None));
    }
    let (input, inst) = res.unwrap();
    if inst == "do()" {
        return Ok((input, Some(Parsed::Do)));
    }
    if inst == "don't()" {
        return Ok((input, Some(Parsed::Dont)));
    }
    let res = separated_pair(i32::<&str, Error<&str>>, tag(","), i32)(input);
    if !res.is_ok() {
        return Ok((input, None));
    }
    let (input, (a, b)) = res.unwrap();
    let res = tag::<&str, &str, Error<&str>>(")")(input);
    if !res.is_ok() {
        return Ok((input, None));
    }
    let (input, _) = res.unwrap();
    Ok((input, Some(Parsed::Mul(a, b))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
