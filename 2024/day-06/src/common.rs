pub use crate::grid::grid::*;
use std::collections::HashMap;
pub use std::fmt::Display;

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
            Content::Guard(c, v) => {
                *c = *c + *v;
            }
        }
    }
    pub fn cell(&mut self) -> Xy {
        match self {
            Content::Empty => (0, 0).into(),
            Content::Obstacle => (0, 0).into(),
            Content::Guard(xy, _) => *xy,
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
    pub grid: Grid<Content>,
    pub guard: Content,
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

    pub fn visited(&mut self) -> HashMap<Xy, ()> {
        let mut visited = HashMap::new();
        visited.insert(self.guard.cell(), ());
        loop {
            if self.next_cell().is_none() {
                return visited;
            }
            if let Some(Content::Obstacle) = self.next_cell() {
                self.guard.turn_right();
                continue;
            }
            self.guard.move_cell();
            visited.insert(self.guard.cell(), ());
        }
    }
}

pub mod parser {
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
