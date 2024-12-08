use crate::common::*;
use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = parser::parse(input);
    Ok(map.antinodes(true).len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("34", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_antinodes_for() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes_for_each(&(0, 0).into(), &(3, 1).into(), true);
        assert_eq!(4, antinodes.len());
        assert_eq!(Xy::new(0, 0), antinodes[0]);
        assert_eq!(Xy::new(3, 1), antinodes[1]);
        assert_eq!(Xy::new(6, 2), antinodes[2]);
        assert_eq!(Xy::new(9, 3), antinodes[3]);
        Ok(())
    }

    #[test]
    fn test_antinodes() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes(true);
        assert_eq!(34, antinodes.len());
        Ok(())
    }
}
