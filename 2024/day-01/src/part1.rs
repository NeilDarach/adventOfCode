use crate::custom_error::AocError;
use itertools::Itertools;
use std::iter::zip;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    for line in input.split("\n") {
        let mut parts = line.split_whitespace();
        if let Some(n1) = parts.next() {
            v1.push(n1.parse()?);
            print!("{} ", n1);
        }
        if let Some(n2) = parts.next() {
            v2.push(n2.parse()?);
            println!("{}", n2);
        }
    }
    Ok(zip(v1.into_iter().sorted(), v2.into_iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
