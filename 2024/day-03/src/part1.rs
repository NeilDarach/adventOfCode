use crate::custom_error::AocError;

mod parser {
    use crate::part1::Token;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, i32},
        combinator::{map, value},
        multi::many1,
        sequence::{delimited, separated_pair},
        IResult,
    };

    pub fn parse(input: &str) -> Vec<Token> {
        if let Ok((_, vec)) = many1(token)(input) {
            vec
        } else {
            vec![]
        }
    }

    fn token(input: &str) -> IResult<&str, Token> {
        alt((
            map(mul, |(a, b)| Token::Mul(a, b)),
            value(Token::Noise, anychar),
        ))(input)
    }

    fn mul(input: &str) -> IResult<&str, (i32, i32)> {
        delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")"))(input)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Mul(i32, i32),
    Noise,
}
#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let tokens = parser::parse(input);
    let mut val = 0;
    for token in tokens {
        if let Token::Mul(a, b) = token {
            val += a * b;
        }
    }
    Ok(val.to_string())
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
