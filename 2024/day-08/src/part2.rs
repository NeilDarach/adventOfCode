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
        let mut x = *a + separation;
        antinodes.push(*a);
        antinodes.push(*b);
        loop {
            if x.x >= 0 && x.x <= self.width && x.y >= 0 && x.y <= self.height {
                antinodes.push(x);
                x = x + separation;
            } else {
                break;
            }
        }

        let mut y = *b - separation;
        loop {
            if y.x >= 0 && y.x <= self.width && y.y >= 0 && y.y <= self.height {
                antinodes.push(y);
                y = y - separation;
            } else {
                break;
            }
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
        assert_eq!("34", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_antinodes_for() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        let antinodes = map.antinodes_for_each(&(0, 0).into(), &(3, 1).into());
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
        let antinodes = map.antinodes();
        assert_eq!(34, antinodes.len());
        Ok(())
    }
}
