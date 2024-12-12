use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use utils::grid::*;

pub trait Bounds {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
    fn sides(&self) -> u32;
}

impl<T> Bounds for Grid<T>
where
    T: Debug + Display,
{
    fn sides(&self) -> u32 {
        let has_side = |visited: &mut HashSet<(Xy, Direction4)>, xy: Xy, d: Direction4| {
            if !visited.insert((xy, d)) || self.get(xy + d).is_some() {
                return false;
            }
            for dir in [d.clockwise(), d.anticlockwise()] {
                let mut next = xy;
                loop {
                    next = next + dir;
                    if self.get(next).is_some() {
                        visited.insert((next, d));
                        if self.get(next + d).is_some() {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            true
        };
        let mut visited = HashSet::new();
        self.all()
            .filter(|(_xy, v)| v.is_some())
            .map(|(xy, _)| {
                Direction4::all()
                    .into_iter()
                    .filter(|&d| has_side(&mut visited, xy, d))
                    .count()
            })
            .sum::<usize>() as u32
    }
    fn area(&self) -> u32 {
        self.all().filter(|(_k, v)| v.is_some()).count() as u32
    }

    fn perimeter(&self) -> u32 {
        self.all()
            .filter(|(_xy, v)| v.is_some())
            .map(|(xy, _)| {
                Direction4::all()
                    .iter()
                    .map(|d| xy + *d)
                    .filter(|e| self.get(*e).is_none())
                    .count()
            })
            .sum::<usize>() as u32
    }
}

pub mod parser {
    use super::*;

    pub fn parse(input: &str) -> Grid<char> {
        let mut grid = Grid::<char>::empty();
        for (j, line) in input.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                grid.insert((i, j).into(), c);
            }
        }
        grid
    }
}

pub fn expand_region(xy: Xy, c: char, source: &mut Grid<char>, dest: &mut Grid<char>) {
    dest.insert(xy, c);
    source.remove(xy);
    for d in Direction4::all() {
        if source.get(xy + d) == Some(&c) {
            expand_region(xy + d, c, source, dest)
        }
    }
}
pub fn all_regions(grid: &mut Grid<char>) -> Vec<Grid<char>> {
    let mut regions = vec![];
    loop {
        let r = find_region(grid);
        if r.is_empty() {
            break;
        }
        regions.push(r);
    }
    regions
}

pub fn find_region(grid: &mut Grid<char>) -> Grid<char> {
    let mut result = Grid::<char>::empty();
    let start = { grid.all().find(|(_k, v)| v.is_some()) };
    if let Some((xy, Some(c))) = start {
        expand_region(xy, *c, grid, &mut result);
    }
    result
}
