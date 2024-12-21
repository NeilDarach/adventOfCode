use crate::custom_error::AocError;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use utils::grid::{Direction4, Grid, Path, Xy};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = parser::parse(input);
    let result = map
        .cheats()
        .iter()
        .map(|(_, _, l)| *l)
        .filter(|l| *l >= 100)
        .count();
    Ok(result.to_string())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Item {
    Empty,
    Wall,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Empty => write!(f, ".")?,
            Item::Wall => write!(f, "#")?,
        }
        Ok(())
    }
}

pub struct Map {
    grid: Grid<Item>,
    start: Xy,
    end: Xy,
}

impl Map {
    pub fn cheats(&self) -> Vec<(Xy, Xy, usize)> {
        let shortest = self.shortest_path();
        let extent = shortest.len();
        let mut cheats = vec![];
        for start in 0..extent {
            let current = shortest[start];
            for d in Direction4::all() {
                let destination = current + d + d;
                if Some(&Item::Wall) == self.grid.get(current + d)
                    && Some(&Item::Empty) == self.grid.get(destination)
                {
                    if let Some(i) = shortest.iter().position(|&e| e == destination) {
                        if i > start {
                            cheats.push((current + d, current + d + d, (i - start - 2)));
                        }
                    }
                }
            }
        }
        cheats
    }
    pub fn shortest_path(&self) -> Vec<Xy> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(Path::new(self.start));
        while let Some(tip) = queue.pop_front() {
            if tip.head() == self.end {
                let mut ret = tip.to_vec();
                ret.reverse();
                return ret;
            }
            if !(visited.contains(&tip.head()) || Some(&Item::Wall) == self.grid.get(tip.head())) {
                visited.insert(tip.0);
                for d in Direction4::all() {
                    queue.push_back(&tip + (tip.head() + d));
                }
            }
        }
        vec![]
    }
}

pub mod parser {
    use super::*;
    pub fn parse(input: &str) -> Map {
        let mut grid = Grid::empty();
        let mut start = Xy::new(0, 0);
        let mut end = Xy::new(0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let item = match c {
                    '#' => Item::Wall,
                    '.' => Item::Empty,
                    'S' => {
                        start = Xy::new(x as i32, y as i32);
                        Item::Empty
                    }
                    'E' => {
                        end = Xy::new(x as i32, y as i32);
                        Item::Empty
                    }
                    _ => panic!("Bad map"),
                };
                grid.insert(Xy::new(x as i32, y as i32), item);
            }
        }
        Map { grid, start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("0", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_sortest_path() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        assert_eq!(Some((5, 7).into()), map.shortest_path().pop());
        assert_eq!(85, map.shortest_path().len());
        Ok(())
    }

    #[test]
    fn test_cheats() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        assert_eq!(44, map.cheats().len());
        Ok(())
    }
}
