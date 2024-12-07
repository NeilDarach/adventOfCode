use crate::custom_error::AocError;
use crate::grid::grid::*;
use std::collections::HashMap;
use std::fmt::Display;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut count = 0;
    let mut map = parser::parse(input);
    let guard = map.guard.clone();
    let visited = visited(&mut map);
    map.guard = guard;
    for c in map.grid.keys() {
        let content = map.grid.get(c).cloned().unwrap();
        let guard = map.guard.clone();
        if visited.contains_key(&c) {
            if process_candidate(&mut map, 0, 0, c) {
                count += 1;
            }
            map.grid.insert(c, content);
            map.guard = guard;
        }
    }

    Ok(count.to_string())
}
pub fn visited(map: &mut Map) -> HashMap<Xy, ()> {
    let mut visited = HashMap::new();
    loop {
        if map.next_cell().is_none() {
            return visited;
        }
        if let Some(Content::Obstacle) = map.next_cell() {
            map.guard.turn_right();
            continue;
        }
        map.guard.move_cell();
        visited.insert(map.guard.cell(), ());
    }
}

pub fn process_candidate(map: &mut Map, _count: usize, _total: usize, pos: Xy) -> bool {
    if let Content::Guard(p, _) = map.guard {
        if p == pos {
            return false;
        }
    }
    if let Some(&Content::Obstacle) = map.grid.get(pos) {
        return false;
    }

    map.grid.insert(pos, Content::Obstacle);
    let mut visited = HashMap::new();
    loop {
        while map.next_cell() == Some(&Content::Empty) {
            map.guard.move_cell();
        }
        let next = map.next_cell();
        if next.is_none() {
            return false;
        }
        if visited.contains_key(&map.guard) {
            return true;
        }
        visited.insert(map.guard, ());
        map.guard.turn_right();
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Content {
    Empty,
    Obstacle,
    Guard(Xy, Direction4),
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "~",
                Self::Obstacle => "#",
                Self::Guard(_, _) => "G",
            }
        )
    }
}

impl Content {
    pub fn look(&self) -> Option<Xy> {
        match self {
            Content::Empty => None,
            Content::Obstacle => None,
            Content::Guard(c, v) => Some(*c + *v),
        }
    }
    pub fn move_cell(&mut self) {
        match self {
            Content::Empty => {}
            Content::Obstacle => {}
            Content::Guard(c, v) => *c = *c + *v,
        }
    }
    pub fn cell(&mut self) -> Xy {
        match self {
            Content::Empty => (0, 0).into(),
            Content::Obstacle => (0, 0).into(),
            Content::Guard(c, _) => *c,
        }
    }
    pub fn turn_right(&mut self) {
        match self {
            Content::Empty => {}
            Content::Obstacle => {}
            Content::Guard(_, v) => {
                *v = v.clockwise();
            }
        };
    }
}

#[derive(Debug)]
pub struct Map {
    grid: Grid<Content>,
    guard: Content,
}

impl Map {
    pub fn default() -> Self {
        Map {
            grid: Grid::empty(),
            guard: Content::Empty,
        }
    }

    pub fn next_cell(&self) -> Option<&Content> {
        if let Some(cell) = self.guard.look() {
            self.grid.get(cell)
        } else {
            None
        }
    }
}
mod parser {
    use super::*;
    pub fn parse(input: &str) -> Map {
        let mut map = Map::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let item = match c {
                    '.' => Content::Empty,
                    '#' => Content::Obstacle,
                    '^' => Content::Guard((x, y).into(), Direction4::N),
                    _ => panic!("Unrecognised symbol {} at ({},{})", c, x, y),
                };
                if let Content::Guard(..) = item {
                    map.guard = item;
                    map.grid.insert((x, y).into(), Content::Empty);
                } else {
                    map.grid.insert((x, y).into(), item);
                };
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_process_candidate() -> miette::Result<()> {
        let mut map = parser::parse(SAMPLE);
        assert!(process_candidate(&mut map, 0, 0, Xy::new(3, 6)));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("6", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        assert_eq!(map.grid.get((0, 0).into()), Some(&Content::Empty));
        assert_eq!(map.grid.get((2, 3).into()), Some(&Content::Obstacle));
        Ok(())
    }

    #[test]
    fn test_look() -> miette::Result<()> {
        let mut map = parser::parse(SAMPLE);
        assert_eq!(map.next_cell(), Some(&Content::Empty));
        map.guard = Content::Guard((4, 1).into(), Direction4::N);
        assert_eq!(map.next_cell(), Some(&Content::Obstacle));
        map.guard = Content::Guard((3, 0).into(), Direction4::E);
        assert_eq!(map.next_cell(), Some(&Content::Obstacle));
        map.guard = Content::Guard((3, 0).into(), Direction4::W);
        assert_eq!(map.next_cell(), Some(&Content::Empty));
        map.guard.turn_right();
        assert_eq!(map.next_cell(), None);

        map.guard.turn_right();
        map.guard.turn_right();
        map.guard.move_cell();
        if let Content::Guard(c, v) = map.guard {
            assert_eq!(Direction4::S, v);
            assert_eq!(Xy::new(3, 1), c);
        } else {
            panic!("Not a guard");
        }

        Ok(())
    }
}
