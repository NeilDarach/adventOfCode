use itertools::Itertools;
use std::collections::HashMap;
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
            .sorted()
            .dedup()
            .collect::<Vec<_>>()
    }

    pub fn antinodes_for_all(&self, c: char, repeating: bool) -> Vec<Xy> {
        match self.antennae.get(&c) {
            None => return vec![],
            Some(v) => v
                .iter()
                .combinations(2)
                .flat_map(|pair| self.antinodes_for_each(pair[0], pair[1], repeating))
                .sorted()
                .dedup()
                .collect::<Vec<Xy>>(),
        }
    }

    pub fn antinodes_for_each(&self, a: &Xy, b: &Xy, repeating: bool) -> Vec<Xy> {
        let separation = *a - *b;
        let mut antinodes = vec![];
        let mut x = *a + separation;
        if repeating {
            antinodes.push(*a);
            antinodes.push(*b);
        }
        loop {
            if x.x >= 0 && x.x <= self.width && x.y >= 0 && x.y <= self.height {
                antinodes.push(x);
                if !repeating {
                    break;
                }
                x = x + separation;
            } else {
                break;
            }
        }

        let mut y = *b - separation;
        loop {
            if y.x >= 0 && y.x <= self.width && y.y >= 0 && y.y <= self.height {
                antinodes.push(y);
                if !repeating {
                    break;
                }
                y = y - separation;
            } else {
                break;
            }
        }
        antinodes
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
