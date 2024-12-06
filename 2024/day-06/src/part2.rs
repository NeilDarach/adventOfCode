use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = parser::parse(input);
    let count = grid
        .cells
        .keys()
        .filter(|e| process_candidate(&grid, e))
        .count();
    Ok(count.to_string())
}
pub fn process_candidate(grid: &Grid, pos: &Coord) -> bool {
    if let Content::Guard(p, _) = grid.guard {
        if p == *pos {
            //dbg!("guard pos");
            return false;
        }
    }
    if let Some(&Content::Obstacle) = grid.cells.get(pos) {
        //dbg!("obstacle");
        return false;
    }

    let mut grid = grid.clone();
    grid.cells.insert(*pos, Content::Obstacle);
    let mut visited = HashMap::new();
    loop {
        if grid.next_cell().is_none() {
            //println!("off the map");
            return false;
        }
        if let Some(Content::Obstacle) = grid.next_cell() {
            //println!("turn");
            grid.guard.turn_right();
            continue;
        }
        if let Some(_) = visited.get(&grid.guard) {
            //println!("loop");
            return true;
        } else {
            //println!("visited {:?}", grid.guard);
            visited.insert(grid.guard, ());
        }
        grid.guard.move_cell();
        //println!("moved {:?}", grid.guard);
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Coord(i32, i32);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Content {
    Empty,
    Obstacle,
    Guard(Coord, Vector),
}

impl Content {
    pub fn look(&self) -> Option<Coord> {
        match self {
            Content::Empty => None,
            Content::Obstacle => None,
            Content::Guard(c, v) => {
                let delta = v.delta();
                Some(Coord(c.0 + delta.0, c.1 + delta.1))
            }
        }
    }
    pub fn move_cell(&mut self) {
        match self {
            Content::Empty => {}
            Content::Obstacle => {}
            Content::Guard(c, v) => {
                let delta = v.delta();
                *c = Coord(c.0 + delta.0, c.1 + delta.1)
            }
        }
    }
    pub fn cell(&mut self) -> Coord {
        match self {
            Content::Empty => Coord(0, 0),
            Content::Obstacle => Coord(0, 0),
            Content::Guard(c, _) => *c,
        }
    }
    pub fn turn_right(&mut self) {
        match self {
            Content::Empty => {}
            Content::Obstacle => {}
            Content::Guard(_, v) => {
                *v = v.turn_right();
            }
        };
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Vector {
    Up,
    Down,
    Left,
    Right,
}

impl Vector {
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    pub fn delta(self) -> Coord {
        match self {
            Self::Up => Coord(0, -1),
            Self::Right => Coord(1, 0),
            Self::Down => Coord(0, 1),
            Self::Left => Coord(-1, 0),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Grid {
    cells: HashMap<Coord, Content>,
    guard: Content,
}

impl Grid {
    pub fn default() -> Self {
        Grid {
            cells: HashMap::default(),
            guard: Content::Empty,
        }
    }

    pub fn next_cell(&self) -> Option<&Content> {
        if let Some(cell) = self.guard.look() {
            self.cells.get(&cell)
        } else {
            None
        }
    }
}

mod parser {
    use super::*;
    pub fn parse(input: &str) -> Grid {
        let mut grid = Grid::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let item = match c {
                    '.' => Content::Empty,
                    '#' => Content::Obstacle,
                    '^' => Content::Guard(Coord(x as i32, y as i32), Vector::Up),
                    _ => panic!("Unrecognised symbol {} at ({},{})", c, x, y),
                };
                if let Content::Guard(..) = item {
                    grid.guard = item;
                    grid.cells.insert(Coord(x as i32, y as i32), Content::Empty);
                } else {
                    grid.cells.insert(Coord(x as i32, y as i32), item);
                };
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_candidate() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let grid = parser::parse(input);
        assert!(process_candidate(&grid, &Coord(3, 6)));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let map = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let grid = parser::parse(map);
        assert_eq!(grid.cells.get(&Coord(0, 0)), Some(&Content::Empty));
        assert_eq!(grid.cells.get(&Coord(2, 3)), Some(&Content::Obstacle));
        Ok(())
    }

    #[test]
    fn test_look() -> miette::Result<()> {
        let map = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut grid = parser::parse(map);
        assert_eq!(grid.next_cell(), Some(&Content::Empty));
        grid.guard = Content::Guard(Coord(4, 1), Vector::Up);
        assert_eq!(grid.next_cell(), Some(&Content::Obstacle));
        grid.guard = Content::Guard(Coord(3, 0), Vector::Right);
        assert_eq!(grid.next_cell(), Some(&Content::Obstacle));
        grid.guard = Content::Guard(Coord(3, 0), Vector::Left);
        assert_eq!(grid.next_cell(), Some(&Content::Empty));
        grid.guard.turn_right();
        assert_eq!(grid.next_cell(), None);

        grid.guard.turn_right();
        grid.guard.turn_right();
        grid.guard.move_cell();
        if let Content::Guard(c, v) = grid.guard {
            assert_eq!(Vector::Down, v);
            assert_eq!(Coord(3, 1), c);
        } else {
            panic!("Not a guard");
        }

        Ok(())
    }
}
