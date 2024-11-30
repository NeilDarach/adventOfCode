use crate::custom_error::AocError;

#[tracing::instrument(skip(_input))]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    panic!("Part 1 not done")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
