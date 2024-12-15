use crate::custom_error::AocError;
use std::fmt::Display;
use utils::grid::*;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut warehouse = parser::parse(input);
    while !warehouse.done() {
        warehouse.step();
    }
    Ok(warehouse.score().to_string())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Item {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::BoxLeft => write!(f, "["),
            Self::BoxRight => write!(f, "]"),
            Self::Robot => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
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
            .filter(|(_k, v)| *v == Some(&Item::BoxLeft))
            .map(|(k, _v)| k.x + (100 * k.y))
            .sum()
    }
    pub fn done(&self) -> bool {
        self.instruction_index >= self.instructions.len()
    }
    pub fn step(&mut self) {
        let instruction = self.instructions[self.instruction_index];
        self.instruction_index += 1;
        match instruction {
            Direction4::E | Direction4::W => self.step_left_right(instruction),
            Direction4::N | Direction4::S => self.step_up_down(instruction),
        }
    }

    pub fn step_left_right(&mut self, instruction: Direction4) {
        let mut cell = self.robot_location;
        loop {
            cell = cell + instruction;
            match self.grid.get(cell) {
                None => panic!("should have hit a wall"),
                Some(&Item::Robot) => panic!("How is a robot on the map?"),
                Some(&Item::Wall) => {
                    return;
                }
                Some(&Item::BoxRight) => {}
                Some(&Item::BoxLeft) => {}
                Some(&Item::Empty) => {
                    break;
                }
            }
        }

        while cell != self.robot_location {
            self.grid
                .insert(cell, self.grid.get(cell - instruction).cloned().unwrap());
            cell = cell - instruction;
        }
        self.grid.insert(cell, Item::Empty);
        self.robot_location = cell + instruction;
    }

    pub fn can_move(&self, xy: Xy, direction: Direction4) -> bool {
        match self.grid.get(xy) {
            None => panic!("Out of bounds"),
            Some(&Item::Robot) => panic!("Robot invasion"),
            Some(&Item::Wall) => false,
            Some(&Item::Empty) => true,
            Some(&Item::BoxLeft) => {
                self.can_move(xy + direction, direction)
                    && self.can_move(xy + direction + Direction4::E, direction)
            }
            Some(&Item::BoxRight) => {
                self.can_move(xy + direction, direction)
                    && self.can_move(xy + Direction4::W + direction, direction)
            }
        }
    }

    pub fn do_move(&mut self, xy: Xy, direction: Direction4) {
        match self.grid.get(xy) {
            None => panic!("Out of bounds"),
            Some(&Item::Robot) => panic!("Robot invasion"),
            Some(&Item::Wall) => {}
            Some(&Item::Empty) => {}
            Some(&Item::BoxLeft) => {
                self.do_move(xy + direction, direction);
                self.do_move(xy + direction + Direction4::E, direction);
                self.grid.insert(xy + direction, Item::BoxLeft);
                self.grid.insert(xy, Item::Empty);
                self.grid
                    .insert(xy + direction + Direction4::E, Item::BoxRight);
                self.grid.insert(xy + Direction4::E, Item::Empty);
            }
            Some(&Item::BoxRight) => {
                self.do_move(xy + Direction4::W, direction);
            }
        }
    }
    pub fn step_up_down(&mut self, instruction: Direction4) {
        if !self.can_move(self.robot_location + instruction, instruction) {
            return;
        }
        if Some(&Item::BoxRight) == self.grid.get(self.robot_location + instruction) {
            self.do_move(
                self.robot_location + instruction + Direction4::W,
                instruction,
            );
        } else {
            self.do_move(self.robot_location + instruction, instruction);
        }
        self.grid.insert(self.robot_location, Item::Empty);
        self.robot_location = self.robot_location + instruction;
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
                match c {
                    '#' => {
                        grid.insert((x * 2, y).into(), Item::Wall);
                        grid.insert(((x * 2) + 1, y).into(), Item::Wall);
                    }
                    'O' => {
                        grid.insert((x * 2, y).into(), Item::BoxLeft);
                        grid.insert(((x * 2) + 1, y).into(), Item::BoxRight);
                    }
                    '.' => {
                        grid.insert((x * 2, y).into(), Item::Empty);
                        grid.insert(((x * 2) + 1, y).into(), Item::Empty);
                    }
                    '@' => {
                        robot_location = Xy::new(x as i32 * 2, y as i32);
                        grid.insert((x * 2, y).into(), Item::Empty);
                        grid.insert(((x * 2) + 1, y).into(), Item::Empty);
                    }
                    _ => panic!("Bad map"),
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
    pub const SAMPLE_1: &str = "########
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

    pub const SAMPLE_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("9021", process(SAMPLE_2)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let warehouse = parser::parse(SAMPLE_1);
        assert_eq!(16, warehouse.grid.width());
        assert_eq!(8, warehouse.grid.height());
        assert_eq!(Xy::new(4, 2), warehouse.robot_location);
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
        assert_eq!(9021, warehouse.score());
        Ok(())
    }

    #[test]
    fn test_sample3() -> miette::Result<()> {
        let mut warehouse = parser::parse(SAMPLE_3);
        while !warehouse.done() {
            warehouse.step();
        }
        assert_eq!(618, warehouse.score());
        Ok(())
    }

    #[test]
    fn test_step() -> miette::Result<()> {
        let mut warehouse = parser::parse(SAMPLE_1);
        warehouse.instructions = vec![
            Direction4::E,
            Direction4::E,
            Direction4::E,
            Direction4::E,
            Direction4::S,
            Direction4::S,
            Direction4::S,
        ];
        while !warehouse.done() {
            warehouse.step();
        }
        assert_eq!(Xy::new(8, 3), warehouse.robot_location);
        assert_eq!(1949, warehouse.score());
        Ok(())
    }
}
