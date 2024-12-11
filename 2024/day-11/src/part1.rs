use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let stones = parser::parse(input);
    let result = (0..25).fold(stones, |acc, _i| {
        acc.iter().flat_map(|&e| step(e)).collect::<Vec<_>>()
    });
    Ok(result.len().to_string())
}

pub fn step(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let digits = stone.to_string();
    let length = digits.len();
    if length % 2 == 0 {
        return vec![
            digits[0..(length / 2)].parse().unwrap(),
            digits[(length / 2)..].parse().unwrap(),
        ];
    }
    vec![stone * 2024]
}

pub mod parser {
    use nom::{bytes::complete::tag, character::complete::u64, multi::separated_list1};

    pub fn parse(input: &str) -> Vec<u64> {
        let (_, result) = separated_list1(tag(" "), u64::<&str, ()>)(input).unwrap();
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("55312", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        assert_eq!(vec![125, 17], parser::parse(SAMPLE));
        Ok(())
    }

    #[test]
    fn test_step() -> miette::Result<()> {
        assert_eq!(vec![253000], step(125));
        assert_eq!(vec![1, 7], step(17));
        assert_eq!(vec![1], step(0));
        Ok(())
    }
}
