use crate::common::*;
use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = parser::parse(input);
    let count: usize = grid
        .keys()
        .filter(|&xy| grid.get(xy) == Some(&0))
        .map(|e| paths(&grid, e).iter().map(|e| e[0]).unique().count())
        .sum();
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRAIL_2: &str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("36", process(TRAIL_2)?);
        Ok(())
    }
}
