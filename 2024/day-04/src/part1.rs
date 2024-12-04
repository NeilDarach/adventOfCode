use crate::custom_error::AocError;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cell {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Grid {
    pub cells: HashMap<Cell, char>,
    pub width: i32,
    pub height: i32,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cell = Cell { x: 0, y: 0 };
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.cells.get(&Cell { x, y }).unwrap())?;
                cell = cell.next(Delta::Right);
            }
            writeln!(f, "")?;
            cell = cell.next(Delta::Down);
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Delta {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: HashMap::default(),
            height: 0,
            width: 0,
        }
    }

    pub fn next(&self, cell: Cell, delta: Delta) -> Option<Cell> {
        let new_cell = cell.next(delta);
        if new_cell.x < 0 || new_cell.x >= self.width {
            return None;
        }
        if new_cell.y < 0 || new_cell.y >= self.height {
            return None;
        }
        Some(new_cell)
    }

    pub fn has_word(&self, cell: Option<Cell>, delta: Delta, word: &str) -> bool {
        if word.len() == 0 {
            return true;
        }
        if cell.is_none() {
            return false;
        }
        let cell = cell.unwrap();
        let Some(c) = self.cells.get(&cell) else {
            return false;
        };
        if word.chars().nth(0) != Some(*c) {
            return false;
        }
        self.has_word(self.next(cell, delta), delta, &word[1..])
    }

    pub fn words_from(&self, cell: Option<Cell>, word: &str) -> i32 {
        let mut count = 0;
        for delta in Delta::all() {
            if self.has_word(cell, delta, word) {
                count += 1;
            }
        }
        count
    }
}

impl Cell {
    pub fn next(&self, delta: Delta) -> Self {
        Cell {
            x: self.x + delta.delta().0,
            y: self.y + delta.delta().1,
        }
    }
}

impl Delta {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::UpRight => (1, -1),
            Self::DownRight => (1, 1),
            Self::DownLeft => (-1, 1),
            Self::UpLeft => (-1, -1),
        }
    }
    pub fn all() -> Vec<Self> {
        vec![
            Delta::Up,
            Delta::Down,
            Delta::Left,
            Delta::Right,
            Delta::UpRight,
            Delta::DownRight,
            Delta::DownLeft,
            Delta::UpLeft,
        ]
    }
}

mod parser {
    use crate::part1::{Cell, Delta, Grid};

    pub fn parse(input: &str) -> Grid {
        let mut cell = Cell { x: 0, y: 0 };
        let mut grid = Grid::new();
        for line in input.lines() {
            for c in line.chars() {
                grid.cells.insert(cell, c);
                cell = cell.next(Delta::Right);
            }
            grid.width = cell.x;
            cell.x = 0;
            cell = cell.next(Delta::Down);
        }
        grid.height = cell.y;
        grid
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = parser::parse(input);
    let mut count = 0;
    for x in 0..=grid.width {
        for y in 0..=grid.height {
            count += grid.words_from(Some(Cell { x, y }), "XMAS");
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::parser::parse;
    use super::*;
    #[test]
    fn test_parse_small_grid() -> miette::Result<()> {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let grid = parse(input);
        assert_eq!('X', *grid.cells.get(&Cell { x: 2, y: 0 }).unwrap());
        assert_eq!('A', *grid.cells.get(&Cell { x: 2, y: 1 }).unwrap());
        Ok(())
    }

    #[test]
    fn test_next_cell() -> miette::Result<()> {
        let grid = parse("abc\ndef\nghi");
        assert_eq!(
            Some(Cell { x: 1, y: 1 }),
            grid.next(Cell { x: 0, y: 0 }, Delta::DownRight)
        );
        assert_eq!(None, grid.next(Cell { x: 0, y: 0 }, Delta::Up));
        assert_eq!(None, grid.next(Cell { x: 2, y: 2 }, Delta::Right));
        assert_eq!(
            Some(Cell { x: 1, y: 1 }),
            grid.next(Cell { x: 0, y: 0 }, Delta::DownRight)
        );
        assert_eq!(
            Some(Cell { x: 1, y: 0 }),
            grid.next(Cell { x: 0, y: 0 }, Delta::Right)
        );
        Ok(())
    }

    fn test_has_word() -> miette::Result<()> {
        let grid = parse("abc\ndef\nghi");
        assert!(grid.has_word(Some(Cell { x: 0, y: 0 }), Delta::Right, "abc"));
        assert!(grid.has_word(Some(Cell { x: 0, y: 0 }), Delta::Down, "adg"));
        assert!(grid.has_word(Some(Cell { x: 0, y: 0 }), Delta::DownRight, "aei"));
        assert!(!grid.has_word(Some(Cell { x: 0, y: 0 }), Delta::DownRight, "aeix"));
        Ok(())
    }

    fn test_words_from() -> miette::Result<()> {
        let grid = parse(
            "..XMAS..
..M..A..
..A..M..
..S..X..
",
        );

        assert_eq!(0, grid.words_from(Some(Cell { x: 0, y: 0 }), "XMAS"));
        assert_eq!(2, grid.words_from(Some(Cell { x: 2, y: 0 }), "XMAS"));
        assert_eq!(1, grid.words_from(Some(Cell { x: 5, y: 3 }), "XMAS"));

        Ok(())
    }

    #[test]
    fn test_dual() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let grid = parse(input);

        assert!(grid.has_word(Some(Cell { x: 3, y: 9 }), Delta::UpRight, "XMAS"));
        assert!(grid.has_word(Some(Cell { x: 3, y: 9 }), Delta::UpLeft, "XMAS"));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
