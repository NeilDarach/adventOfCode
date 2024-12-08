use crate::common::*;
use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = parser::parse(input);
    Ok(map.antinodes(false).len().to_string())
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
        assert_eq!("14", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        assert_eq!(2, map.antennae.keys().len());
        let zero: Vec<Xy> = vec![(8, 1).into(), (5, 2).into(), (7, 3).into(), (4, 4).into()];
        assert_eq!(&zero, map.antennae.get(&'0').unwrap());
        assert_eq!(11, map.width);
        assert_eq!(11, map.height);
        Ok(())
    }

    #[test]
    fn test_antinodes_for() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes_for_each((&(5, 2).into(), &(7, 3).into()));
        assert_eq!(2, antinodes.len());
        assert_eq!(Xy::new(3, 1), antinodes[0]);
        assert_eq!(Xy::new(9, 4), antinodes[1]);
        Ok(())
    }

    #[test]
    fn test_antinodes_off_the_map() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes_for_each((&(6, 5).into(), &(9, 9).into()));
        assert_eq!(1, antinodes.len());
        assert_eq!(Xy::new(3, 1), antinodes[0]);
        Ok(())
    }

    #[test]
    fn test_antinodes() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes(false);
        assert_eq!(14, antinodes.len());
        Ok(())
    }
}
