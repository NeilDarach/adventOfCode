use crate::common::*;
use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let robots = parser::parse(input);
    let mut zone = Zone::new(101, 103, robots);
    for _i in 0..100 {
        zone.step();
    }
    Ok(zone.safety().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let file = include_str!("../input2.txt");
        assert_eq!("216027840", process(file)?);
        Ok(())
    }
}
