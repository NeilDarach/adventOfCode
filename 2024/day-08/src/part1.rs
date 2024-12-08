use crate::custom_error::AocError;
use crate::grid::grid::Xy;
use itertools::Itertools;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = parser::parse(input);
    Ok(map.antinodes().len().to_string())
}

pub struct Map {
    antennae: HashMap<char, Vec<Xy>>,
    width: i32,
    height: i32,
}

impl Map {
    pub fn antinodes(&self) -> Vec<Xy> {
        self.antennae
            .keys()
            .flat_map(|&e| self.antinodes_for_all(e))
            .sorted()
            .dedup()
            .collect::<Vec<_>>()
    }

    pub fn antinodes_for_all(&self, c: char) -> Vec<Xy> {
        match self.antennae.get(&c) {
            None => return vec![],
            Some(v) => v
                .iter()
                .combinations(2)
                .flat_map(|pair| self.antinodes_for_each(pair[0], pair[1]))
                .sorted()
                .dedup()
                .collect::<Vec<Xy>>(),
        }
    }

    pub fn antinodes_for_each(&self, a: &Xy, b: &Xy) -> Vec<Xy> {
        let separation = *a - *b;
        let mut antinodes = vec![];
        let x = *a + separation;
        let y = *b - separation;
        if x.x >= 0 && x.x <= self.width && x.y >= 0 && x.y <= self.height {
            antinodes.push(x);
        }
        if y.x >= 0 && y.x <= self.width && y.y >= 0 && y.y <= self.height {
            antinodes.push(y);
        }
        antinodes
    }
}

mod parser {
    use super::*;
    pub fn parse(input: &str) -> Map {
        let mut antennae = HashMap::default();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            height = y;
            width = line.len() - 1;
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let xy: Xy = (x, y).into();
                    antennae
                        .entry(c)
                        .and_modify(|e: &mut Vec<Xy>| e.push(xy))
                        .or_insert(vec![xy]);
                }
            }
        }
        Map {
            antennae,
            width: width as i32,
            height: height as i32,
        }
    }
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
        let antinodes = map.antinodes_for_each(&(5, 2).into(), &(7, 3).into());
        assert_eq!(2, antinodes.len());
        assert_eq!(Xy::new(3, 1), antinodes[0]);
        assert_eq!(Xy::new(9, 4), antinodes[1]);
        Ok(())
    }

    #[test]
    fn test_antinodes_off_the_map() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes_for_each(&(6, 5).into(), &(9, 9).into());
        assert_eq!(1, antinodes.len());
        assert_eq!(Xy::new(3, 1), antinodes[0]);
        Ok(())
    }

    #[test]
    fn test_antinodes() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes();
        assert_eq!(14, antinodes.len());
        Ok(())
    }
}
