use aoc_2023::aoc::*;
use colored::Colorize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::{self, Debug};

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord(i32, i32);

#[derive(Debug, Default)]
pub struct Grid {
    cells: HashMap<Coord, Pipe>,
    start: Option<Coord>,
    width: i32,
    height: i32,
}

impl Grid {
    fn start(&self) -> Coord {
        self.start.unwrap()
    }

    fn colored(&self, path: &[Coord]) {
        let last = path[path.len() - 1];
        let start = self.start();
        for y in 0..=self.height {
            for x in 0..=self.width {
                let c = self.cell_at(Coord(x, y)).to_string();
                print!(
                    "{}",
                    if Coord(x, y) == last {
                        c.blue()
                    } else if Coord(x, y) == start {
                        c.green()
                    } else if path.contains(&Coord(x, y)) {
                        c.red()
                    } else {
                        c.white()
                    }
                );
            }
            println!();
        }
    }

    fn add_cell(&mut self, coord: Coord, pipe: Pipe) {
        self.cells.insert(coord, pipe);
        if pipe == Pipe::Start {
            self.start = Some(coord);
        }
        self.width = self.width.max(coord.0);
        self.height = self.height.max(coord.1);
    }

    fn len(&self) -> usize {
        self.cells.keys().len()
    }

    fn cell_at(&self, coord: Coord) -> Pipe {
        *self.cells.get(&coord).unwrap()
    }

    fn apply_delta(&self, coord: Coord, delta: (i32, i32)) -> Option<Coord> {
        let x = coord.0 + delta.0;
        let y = coord.1 + delta.1;
        if x < 0 || y < 0 || y > self.height || x > self.width {
            return None;
        }
        Some(Coord(x, y))
    }

    fn links_back(&self, c1: Coord, c2: Coord) -> bool {
        self.exits(c2).iter().any(|&e| e == c1)
    }

    fn extend_loop(
        &self,
        (path, prev, current): (Vec<Coord>, Coord, Coord),
    ) -> Vec<(Vec<Coord>, Coord, Coord)> {
        let exits = self.forward(prev, current);
        //println!("Exits from {} are {:?}", current, exits);
        if exits.is_empty() {
            return vec![];
        }
        exits
            .iter()
            .map(|&e| {
                let mut v = path.clone();
                v.push(prev);
                (v, current, e)
            })
            .collect()
    }

    fn derive_start_pipe(&self, path: &Vec<Coord>) -> Pipe {
        let last = path[path.len() - 1];
        let first = path[1];
        let begin = (first.0 - self.start().0, first.1 - self.start().1);
        let end = (last.0 - self.start().0, last.1 - self.start().1);
        match (begin, end) {
            ((0, -1), (-1, 0)) => Pipe::StoW,
            ((0, -1), (1, 0)) => Pipe::StoE,
            ((0, -1), (0, 1)) => Pipe::NtoS,

            ((0, 1), (-1, 0)) => Pipe::NtoW,
            ((0, 1), (1, 0)) => Pipe::NtoE,
            ((0, 1), (0, -1)) => Pipe::NtoS,

            ((1, 0), (0, 1)) => Pipe::NtoE,
            ((1, 0), (0, -1)) => Pipe::StoE,
            ((1, 0), (-1, 0)) => Pipe::EtoW,

            ((-1, 0), (0, 1)) => Pipe::EtoW,
            ((-1, 0), (0, -1)) => Pipe::StoW,
            ((-1, 0), (1, 0)) => Pipe::EtoW,
            _ => panic!("Bad diff {:?} {:?} ", begin, end),
        }
    }

    fn count_inside(&self, path: &Vec<Coord>) -> i32 {
        let mut count = 0;
        for y in 0..self.height {
            let mut state = 0;
            for x in 0..self.width {
                let cell = self.cell_at(Coord(x, y));
                if path.contains(&Coord(x, y)) {
                    state += cell.flip();
                } else {
                    count += state % 2
                }
            }
        }
        count
    }

    fn find_loop(&self) -> Vec<Coord> {
        let start = self.start();
        let mut current: Vec<(Vec<Coord>, Coord, Coord)> = self
            .exits(start)
            .iter()
            .filter(|&e| self.links_back(start, *e))
            .map(|&e| (vec![], start, e))
            .collect();
        //println!("find_loop starting at {}, going into {:?}", start, current);
        loop {
            current = current
                .iter()
                .flat_map(|e| self.extend_loop(e.clone()))
                .collect();
            //println!("current updated to {:?}", current);
            if let Some(e) = current.iter().find(|e| e.2 == start) {
                let mut r = e.0.clone();
                r.push(e.1);
                return r;
            }
        }
    }

    fn valid_exits(&self, coord: Coord) -> Vec<Coord> {
        let r = self
            .exits(coord)
            .into_iter()
            .filter(|&e| self.links_back(coord, e))
            .collect();
        r
    }

    fn exits(&self, coord: Coord) -> Vec<Coord> {
        self.cell_at(coord)
            .exits()
            .into_iter()
            .filter_map(|e| self.apply_delta(coord, e))
            .collect()
    }

    fn forward(&self, last: Coord, current: Coord) -> Vec<Coord> {
        self.valid_exits(current)
            .into_iter()
            .filter(|&e| e != last)
            .collect()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pipe {
    NtoE,
    NtoW,
    StoE,
    StoW,
    NtoS,
    EtoW,
    Start,
    Ground,
}
impl Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::Start => "S",
                Pipe::NtoS => "|",
                Pipe::EtoW => "-",
                Pipe::StoW => "7",
                Pipe::NtoE => "L",
                Pipe::NtoW => "J",
                Pipe::StoE => "F",
                Pipe::Ground => ".",
            }
        )
    }
}

impl Pipe {
    pub fn exits(&self) -> Vec<(i32, i32)> {
        match self {
            Pipe::Start => [(-1, 0), (1, 0), (0, 1), (0, -1)].to_vec(),
            Pipe::NtoS => [(0, 1), (0, -1)].to_vec(),
            Pipe::EtoW => [(1, 0), (-1, 0)].to_vec(),
            Pipe::StoW => [(0, 1), (-1, 0)].to_vec(),
            Pipe::NtoE => [(0, -1), (1, 0)].to_vec(),
            Pipe::NtoW => [(0, -1), (-1, 0)].to_vec(),
            Pipe::StoE => [(0, 1), (1, 0)].to_vec(),
            Pipe::Ground => [].to_vec(),
        }
    }
    pub fn flip(&self) -> i32 {
        match self {
            Pipe::Start => panic!("Didn't expect start"),
            Pipe::NtoS => 1,
            Pipe::EtoW => 0,
            Pipe::StoW => 1,
            Pipe::NtoE => 0,
            Pipe::NtoW => 0,
            Pipe::StoE => 1,
            Pipe::Ground => 0,
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            'S' => Pipe::Start,
            '|' => Pipe::NtoS,
            '-' => Pipe::EtoW,
            '7' => Pipe::StoW,
            'L' => Pipe::NtoE,
            'J' => Pipe::NtoW,
            'F' => Pipe::StoE,
            '.' => Pipe::Ground,
            _ => panic!("Bad pipe {}", value),
        }
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let grid = parse::grid(data);
    let v = grid.find_loop();

    v.len() as u64 / 2
}

fn part2(data: &str) -> u64 {
    let mut grid = parse::grid(data);
    let v = grid.find_loop();
    let start_pipe = grid.derive_start_pipe(&v);
    grid.cells
        .entry(grid.start())
        .and_modify(|e| *e = start_pipe);
    grid.colored(&v);
    grid.count_inside(&v) as u64
}

fn main() {
    println!("Day x of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input());
    println!("Part 2 answer is {}", val);
}

mod parse {
    use super::*;

    pub fn grid(i: &str) -> Grid {
        let mut grid = Grid::default();
        for (y, line) in i.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.add_cell(Coord(x as i32, y as i32), c.into());
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    fn sample2() -> &'static str {
        include_str!("sample2.txt")
    }

    fn sample3() -> &'static str {
        include_str!("sample3.txt")
    }

    #[test]
    fn test_parse_grid() {
        let grid = parse::grid(sample());
        assert_eq!(25, grid.len());
        assert_eq!(Pipe::Start, grid.cell_at(Coord(1, 1)));
        assert_eq!(Coord(1, 1), grid.start());
    }

    #[test]
    fn test_exits() {
        let grid = parse::grid(sample());
        assert_eq!(4, grid.exits(grid.start()).len());
        assert_eq!(2, grid.valid_exits(grid.start()).len());
    }
    #[test]
    fn test_forward() {
        let grid = parse::grid(sample());
        assert_eq!(vec![Coord(1, 3)], grid.forward(grid.start(), Coord(1, 2)));
    }

    #[test]
    fn test_sample() {
        assert_eq!(4, part1(sample()));
        assert_eq!(8, part1(sample2()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(0, part2(sample()));
    }

    #[test]
    fn test_count_inside() {
        let mut grid = parse::grid(sample3());
        let v = grid.find_loop();
        let start_pipe = grid.derive_start_pipe(&v);
        grid.cells
            .entry(grid.start())
            .and_modify(|e| *e = start_pipe);
        assert_eq!(8, grid.count_inside(&v));
    }
}
