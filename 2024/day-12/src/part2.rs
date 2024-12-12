use crate::common::*;
use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut grid = parser::parse(input);
    let regions = all_regions(&mut grid);
    let cost: u32 = regions.iter().map(|e| e.area() * e.sides()).sum();
    Ok(cost.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "AAAA\nBBCD\nBBCC\nEEEC";
    const SAMPLE_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("1206", process(SAMPLE_3)?);
        Ok(())
    }

    #[test]
    fn test_find_region() -> miette::Result<()> {
        let mut grid = parser::parse(SAMPLE_1);
        let r = find_region(&mut grid);
        assert_eq!(4, r.sides());
        Ok(())
    }
}
