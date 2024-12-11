use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let stones = parser::parse(input);
    let mut memo = Stones::new();
    let result: usize = stones.iter().map(|&e| memo.expand(e, 75)).sum();
    Ok(result.to_string())
}

pub struct Stones {
    memo: HashMap<(usize, usize), usize>,
}

impl Stones {
    pub fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    pub fn expand(&mut self, val: usize, count: usize) -> usize {
        if count == 0 {
            return 1;
        }
        if let Some(v) = self.memo.get(&(val, count)) {
            return *v;
        }
        if val == 0 {
            let v = self.expand(1, count - 1);
            self.memo.insert((val, count), v);
            return v;
        }
        if let Some((a, b)) = self.split(val) {
            let v = self.expand(a, count - 1) + self.expand(b, count - 1);
            self.memo.insert((val, count), v);
            return v;
        }
        let v = self.expand(val * 2024, count - 1);
        self.memo.insert((val, count), v);
        return v;
    }

    fn split(&self, val: usize) -> Option<(usize, usize)> {
        let digits = val.to_string();
        let length = digits.len();
        if length % 2 == 0 {
            return Some((
                digits[0..(length / 2)].parse().unwrap(),
                digits[(length / 2)..].parse().unwrap(),
            ));
        }
        None
    }
}

pub mod parser {
    use nom::{
        bytes::complete::tag, character::complete::u64, combinator::map, multi::separated_list1,
    };

    pub fn parse(input: &str) -> Vec<usize> {
        let (_, result) =
            separated_list1(tag(" "), map(u64::<&str, ()>, |e| e as usize))(input).unwrap();
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("65601038650482", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        assert_eq!(vec![125, 17], parser::parse(SAMPLE));
        Ok(())
    }

    #[test]
    fn test_expand() -> miette::Result<()> {
        let mut memo = Stones::new();
        assert_eq!(1, memo.expand(0, 2));
        assert_eq!(2, memo.expand(0, 3));
        assert_eq!(7, memo.expand(0, 6));
        Ok(())
    }
}
