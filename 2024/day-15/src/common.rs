use std::fmt::Display;
use std::marker::PhantomData;
use utils::grid::*;

pub trait WarehouseType {
    fn add_item<T>(x: i32, y: i32, c: char, warehouse: &mut Warehouse<T>)
    where
        T: WarehouseType;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Item {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Box => write!(f, "O"),
            Self::BoxLeft => write!(f, "["),
            Self::BoxRight => write!(f, "]"),
            Self::Robot => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}

pub struct Warehouse<T>
where
    T: WarehouseType,
{
    pub grid: Grid<Item>,
    pub robot_location: Xy,
    pub instructions: Vec<Direction4>,
    pub instruction_index: usize,
    marker: PhantomData<T>,
}

impl<T> Warehouse<T>
where
    T: WarehouseType,
{
    pub fn score(&self) -> i32 {
        self.grid
            .all()
            .filter(|(_k, v)| *v == Some(&Item::BoxLeft) || *v == Some(&Item::Box))
            .map(|(k, _v)| k.x + (100 * k.y))
            .sum()
    }
    pub fn done(&self) -> bool {
        self.instruction_index >= self.instructions.len()
    }
    pub fn step(&mut self) {
        let instruction = self.instructions[self.instruction_index];
        self.instruction_index += 1;
        self.step_up_down(instruction);
    }

    pub fn can_move(&self, xy: Xy, direction: Direction4) -> bool {
        let is_up_down = direction == Direction4::S || direction == Direction4::N;
        match self.grid.get(xy) {
            None => panic!("Out of bounds"),
            Some(&Item::Robot) => panic!("Robot invasion"),
            Some(&Item::Wall) => false,
            Some(&Item::Empty) => true,
            Some(&Item::Box) => self.can_move(xy + direction, direction),
            Some(&Item::BoxLeft) if is_up_down => {
                self.can_move(xy + direction, direction)
                    && self.can_move(xy + direction + Direction4::E, direction)
            }
            Some(&Item::BoxLeft) => self.can_move(xy + direction, direction),
            Some(&Item::BoxRight) if is_up_down => {
                self.can_move(xy + direction, direction)
                    && self.can_move(xy + Direction4::W + direction, direction)
            }
            Some(&Item::BoxRight) => self.can_move(xy + direction, direction),
        }
    }

    pub fn do_move(&mut self, xy: Xy, direction: Direction4) {
        let is_up_down = direction == Direction4::S || direction == Direction4::N;
        match self.grid.get(xy) {
            None => panic!("Out of bounds"),
            Some(&Item::Robot) => panic!("Robot invasion"),
            Some(&Item::Wall) => {}
            Some(&Item::Empty) => {}
            Some(&Item::BoxLeft) if is_up_down => {
                self.do_move(xy + direction, direction);
                self.do_move(xy + direction + Direction4::E, direction);
                self.grid.insert(xy + direction, Item::BoxLeft);
                self.grid.insert(xy, Item::Empty);
                self.grid
                    .insert(xy + direction + Direction4::E, Item::BoxRight);
                self.grid.insert(xy + Direction4::E, Item::Empty);
            }
            Some(&Item::BoxLeft) => {
                self.do_move(xy + direction, direction);
                self.grid.insert(xy + direction, Item::BoxLeft);
                self.grid.insert(xy, Item::Empty);
            }
            Some(&Item::BoxRight) if is_up_down => {
                self.do_move(xy + Direction4::W, direction);
            }
            Some(&Item::BoxRight) => {
                self.do_move(xy + direction, direction);
                self.grid.insert(xy + direction, Item::BoxRight);
                self.grid.insert(xy, Item::Empty);
            }
            Some(&Item::Box) => {
                self.do_move(xy + direction, direction);
                self.grid.insert(xy + direction, Item::Box);
                self.grid.insert(xy, Item::Empty);
            }
        }
    }
    pub fn step_up_down(&mut self, instruction: Direction4) {
        let is_up_down = instruction == Direction4::N || instruction == Direction4::S;
        if !self.can_move(self.robot_location + instruction, instruction) {
            return;
        }
        if is_up_down && Some(&Item::BoxRight) == self.grid.get(self.robot_location + instruction) {
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

impl<T> Display for Warehouse<T>
where
    T: WarehouseType,
{
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
    pub fn parse<T>(input: &str) -> Warehouse<T>
    where
        T: WarehouseType,
    {
        let mut warehouse = Warehouse::<T> {
            grid: Grid::empty(),
            robot_location: Xy::new(0, 0),
            instructions: vec![],
            instruction_index: 0,
            marker: PhantomData,
        };
        let lines = input.lines().enumerate().collect::<Vec<_>>();
        let mut index = 0;
        loop {
            let (y, line) = lines[index];
            index += 1;
            if line.is_empty() {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                T::add_item(x as i32, y as i32, c, &mut warehouse);
            }
        }
        loop {
            if index >= lines.len() {
                break;
            }
            let (_y, line) = lines[index];
            index += 1;
            for c in line.chars() {
                warehouse.instructions.push(match c {
                    '<' => Direction4::W,
                    '^' => Direction4::N,
                    '>' => Direction4::E,
                    'v' => Direction4::S,
                    _ => panic!("Bad direction - {}", c),
                })
            }
        }
        warehouse
    }
}
