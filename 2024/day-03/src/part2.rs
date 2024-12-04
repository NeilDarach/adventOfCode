use crate::custom_error::AocError;
mod parser {
    use crate::part2::Token;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, i32},
        combinator::{map, value},
        multi::{many1, many_till},
        sequence::{delimited, separated_pair},
        IResult,
    };

    pub fn parse(input: &str) -> Vec<Token> {
        match tokens(input) {
            Ok((_, tokens)) => tokens,
            _ => vec![],
        }
    }

    pub fn tokens(input: &str) -> IResult<&str, Vec<Token>> {
        many1(token)(input)
    }

    pub fn token(input: &str) -> IResult<&str, Token> {
        map(
            many_till(
                anychar,
                alt((
                    mul,
                    value(Token::Do, tag("do()")),
                    value(Token::Dont, tag("don't()")),
                )),
            ),
            |(_, tok)| tok,
        )(input)
    }

    fn mul(input: &str) -> IResult<&str, Token> {
        map(
            delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")")),
            |(a, b)| Token::Mul(a, b),
        )(input)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Do,
    Dont,
    Mul(i32, i32),
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut switch = true;
    let mut val = 0;
    for item in parser::parse(input) {
        match item {
            Token::Mul(a, b) => {
                if switch {
                    val += a * b;
                }
            }
            Token::Do => switch = true,
            Token::Dont => switch = false,
        }
    }
    Ok(val.to_string())
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
