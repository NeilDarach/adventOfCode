use crate::custom_error::AocError;
use std::fmt::Display;
use utils::grid::*;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut warehouse = parser::parse(input);
    while !warehouse.done() {
        warehouse.step();
    }
    println!("{}", warehouse);
    Ok(warehouse.score().to_string())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Item {
    Wall,
    Box,
    Robot,
    Empty,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Box => write!(f, "O"),
            Self::Robot => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            '.' => Self::Empty,
            _ => panic!("Bad item type- {}", value),
        }
    }
}

pub struct Warehouse {
    grid: Grid<Item>,
    robot_location: Xy,
    instructions: Vec<Direction4>,
    instruction_index: usize,
}

impl Warehouse {
    pub fn score(&self) -> i32 {
        self.grid
            .all()
            .filter(|(_k, v)| *v == Some(&Item::Box))
            .map(|(k, _v)| k.x + (100 * k.y))
            .sum()
    }
    pub fn done(&self) -> bool {
        self.instruction_index >= self.instructions.len()
    }
    pub fn step(&mut self) {
        let instruction = self.instructions[self.instruction_index];
        self.instruction_index += 1;
        let mut next_cell = self.robot_location;
        loop {
            next_cell = next_cell + instruction;
            match self.grid.get(next_cell) {
                None => {
                    println!("{}", self);
                    println!(
                        "loc: {}, cell: {}, dir: {}",
                        self.robot_location, next_cell, instruction,
                    );
                    panic!("Should have hit a wall ");
                }
                Some(&Item::Wall) => {
                    return;
                }
                Some(&Item::Empty) => {
                    break;
                }
                Some(&Item::Box) => {}
                Some(&Item::Robot) => {}
            }
        }
        self.robot_location = self.robot_location + instruction;
        self.grid.insert(next_cell, Item::Box);
        self.grid.insert(self.robot_location, Item::Empty);
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.grid.height() {
            write!(f, "{:<04} ", y)?;
            for x in 0..self.grid.width() {
                if Xy::new(x, y) == self.robot_location {
                    write!(f, "{}", Item::Robot)?;
                } else {
                    write!(
                        f,
                        "{}",
                        self.grid.get(Xy::new(x, y)).unwrap_or(&Item::Empty)
                    )?;
                }
            }
            writeln!(f)?;
        }
        let mut index = 0;
        for each in self.instructions.chunks(80) {
            for d in each.iter() {
                if index == self.instruction_index {
                    write!(f, " {} ", d)?
                } else {
                    write!(f, "{}", d)?;
                }
                index += 1;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub mod parser {
    use super::*;
    pub fn parse(input: &str) -> Warehouse {
        let mut grid: Grid<Item> = Grid::empty();
        let mut robot_location = Xy::new(0, 0);
        let mut instructions = vec![];
        let lines = input.lines().enumerate().collect::<Vec<_>>();
        let mut index = 0;
        loop {
            let (y, line) = lines[index];
            index += 1;
            if line.is_empty() {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                let item = c.into();
                if item == Item::Robot {
                    robot_location = Xy::new(x as i32, y as i32);
                    grid.insert((x, y).into(), Item::Empty);
                } else {
                    grid.insert((x, y).into(), item)
                }
            }
        }
        loop {
            if index >= lines.len() {
                break;
            }
            let (_y, line) = lines[index];
            index += 1;
            for c in line.chars() {
                instructions.push(match c {
                    '<' => Direction4::W,
                    '^' => Direction4::N,
                    '>' => Direction4::E,
                    'v' => Direction4::S,
                    _ => panic!("Bad direction - {}", c),
                })
            }
        }

        Warehouse {
            grid,
            robot_location,
            instructions,
            instruction_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const SAMPLE_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("10092", process(SAMPLE_2)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let warehouse = parser::parse(SAMPLE_1);
        assert_eq!(8, warehouse.grid.width());
        assert_eq!(8, warehouse.grid.height());
        assert_eq!(Xy::new(2, 2), warehouse.robot_location);
        assert_eq!(15, warehouse.instructions.len());
        assert_eq!(&Direction4::W, warehouse.instructions.first().unwrap());
        Ok(())
    }

    #[test]
    fn test_score() -> miette::Result<()> {
        let mut warehouse = parser::parse(SAMPLE_2);
        while !warehouse.done() {
            warehouse.step();
        }
        println!("{}", warehouse);
        assert_eq!(10092, warehouse.score());
        Ok(())
    }

    #[test]
    fn test_step() -> miette::Result<()> {
        let mut warehouse = parser::parse(SAMPLE_1);
        assert_eq!(Xy::new(2, 2), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(2, 2), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(2, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(2, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(3, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(4, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(4, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(4, 2), warehouse.robot_location);
        println!("{}", warehouse);
        while !warehouse.done() {
            warehouse.step();
        }
        println!("{}", warehouse);
        assert_eq!(Xy::new(4, 4), warehouse.robot_location);
        assert_eq!(2028, warehouse.score());
        Ok(())
    }
}
