use crate::custom_error::AocError;
use std::collections::HashMap;

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
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for n in v2 {
        freq.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }
    let res = v1
        .into_iter()
        .map(|i| freq.get(&i).or(Some(&0_i32)).unwrap() * i)
        .sum::<i32>();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
