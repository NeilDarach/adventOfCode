use crate::custom_error::AocError;
use std::ops::BitXor;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let score = input
        .lines()
        .map(|e| e.parse::<i64>().unwrap())
        .map(|e| nth_secret(e, 2000))
        .sum::<i64>();
    Ok(score.to_string())
}

pub fn next_secret(i: i64) -> i64 {
    let mut secret = i;
    secret = (secret * 64).bitxor(secret) % 16777216;
    secret = (secret / 32).bitxor(secret) % 16777216;
    secret = (secret * 2048).bitxor(secret) % 16777216;
    secret
}

pub fn nth_secret(i: i64, count: i64) -> i64 {
    if count == 0 {
        i
    } else {
        nth_secret(next_secret(i), count - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1
10
100
2024";

    #[test]
    fn test_123() -> miette::Result<()> {
        let mut next = next_secret(123);
        assert_eq!(15887950, next);
        next = next_secret(next);
        assert_eq!(16495136, next);

        assert_eq!(5908254, nth_secret(123, 10));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let score = process(SAMPLE)?;
        assert_eq!("37327623", score);
        Ok(())
    }
}
