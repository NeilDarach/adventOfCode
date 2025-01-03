use itertools::chain;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::successors;
pub use utils::grid::Xy;

pub struct Map {
    pub antennae: HashMap<char, Vec<Xy>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn antinodes(&self, repeating: bool) -> Vec<Xy> {
        self.antennae
            .keys()
            .flat_map(|&e| self.antinodes_for_all(e, repeating))
            .unique()
            .collect::<Vec<_>>()
    }

    pub fn antinodes_for_all(&self, c: char, repeating: bool) -> Vec<Xy> {
        match self.antennae.get(&c) {
            None => return vec![],
            Some(v) => v
                .iter()
                .combinations(2)
                .flat_map(|pair| {
                    if repeating {
                        self.repeating_antinodes_for_each((pair[0], pair[1]))
                    } else {
                        self.antinodes_for_each((pair[0], pair[1]))
                    }
                })
                .collect::<Vec<Xy>>(),
        }
    }

    fn contains(&self, pos: Xy) -> bool {
        pos.x >= 0 && pos.x <= self.width && pos.y >= 0 && pos.y <= self.height
    }

    pub fn antinodes_for_each(&self, (a, b): (&Xy, &Xy)) -> Vec<Xy> {
        let sep = *a - *b;
        [*a + sep, *b - sep]
            .into_iter()
            .filter(|e| self.contains(*e))
            .collect::<Vec<_>>()
    }

    pub fn repeating_antinodes_for_each(&self, (a, b): (&Xy, &Xy)) -> Vec<Xy> {
        let sep = *a - *b;
        let contains_or_none = |xy| if self.contains(xy) { Some(xy) } else { None };
        chain(
            successors(Some(*a), |&e| contains_or_none(e + sep)),
            successors(Some(*b), |&e| contains_or_none(e - sep)),
        )
        .collect::<Vec<_>>()
    }
}

pub mod parser {
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
