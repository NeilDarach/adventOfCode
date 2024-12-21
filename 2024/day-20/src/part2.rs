use crate::custom_error::AocError;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

use utils::grid::{Direction4, Grid, Path, Xy};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut map = parser::parse(input);
    map.shortest_path();
    let result = map.cheats(100).len();
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
    shortest_path: Vec<Xy>,
    shortest_path_map: HashMap<Xy, usize>,
}

impl Map {
    fn distance(a: Xy, b: Xy) -> i32 {
        (b.x - a.x).abs() + (b.y - a.y).abs()
    }
    pub fn shortest_path(&mut self) -> &Vec<Xy> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(Path::new(self.start));
        while let Some(tip) = queue.pop_front() {
            if tip.head() == self.end {
                let mut ret = tip.to_vec();
                ret.reverse();
                self.shortest_path = ret;
                self.shortest_path.iter().enumerate().for_each(|(i, &e)| {
                    self.shortest_path_map.insert(e, i);
                });
                return &self.shortest_path;
            }
            if !(visited.contains(&tip.head()) || Some(&Item::Wall) == self.grid.get(tip.head())) {
                visited.insert(tip.0);
                for d in Direction4::all() {
                    queue.push_back(&tip + (tip.head() + d));
                }
            }
        }
        &self.shortest_path
    }

    pub fn cheats(&self, limit: i32) -> Vec<(Xy, Xy, i32)> {
        let mut found = HashSet::new();
        let empty_cells = self
            .grid
            .all()
            .filter(|(_k, v)| v == &Some(&Item::Empty))
            .map(|(k, _v)| k)
            .collect::<Vec<_>>();
        //dbg!(&self.shortest_path_map);
        for start in 0..self.shortest_path.len() {
            let current = self.shortest_path[start];
            empty_cells
                .iter()
                .map(|&e| (e, Map::distance(current, e)))
                .filter(|(_e, d)| *d <= 20)
                .filter(|(e, _d)| *self.shortest_path_map.get(e).unwrap() > start)
                .map(|(e, d)| {
                    (
                        e,
                        (*self.shortest_path_map.get(&e).unwrap() as i32 - start as i32 - d),
                    )
                })
                .filter(|(_e, d)| *d >= limit)
                .for_each(|(e, d)| {
                    found.insert((current, e, d));
                });
        }

        found.into_iter().collect::<Vec<_>>()
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
        Map {
            grid,
            start,
            end,
            shortest_path: vec![],
            shortest_path_map: HashMap::default(),
        }
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
        //assert_eq!("0", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_sortest_path() -> miette::Result<()> {
        let mut map = parser::parse(SAMPLE);
        map.shortest_path();

        assert_eq!(
            Xy::new(5, 7),
            map.shortest_path[map.shortest_path.len() - 1]
        );
        assert_eq!(85, map.shortest_path().len());
        Ok(())
    }

    #[test]
    fn test_cheats() -> miette::Result<()> {
        let mut map = parser::parse(SAMPLE);
        map.shortest_path();
        assert_eq!(285, map.cheats(50).len());
        Ok(())
    }
}
