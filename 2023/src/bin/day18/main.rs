use aoc_2023::aoc::*;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Color(String);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    direction: Direction,
    distance: i32,
    color: Color,
}

#[derive(Default, Eq, PartialEq, Clone)]
pub enum Item {
    #[default]
    Ground,
    Trench(Option<Direction>, Option<Direction>, Color),
    Hole(Option<Color>),
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => write!(f, ".")?,
            Self::Trench(_, _, _) => write!(f, "#")?,
            Self::Hole(_) => write!(f, "%")?,
        }
        Ok(())
    }
}

impl Item {
    pub fn set_exit(&mut self, exit: Direction) {
        match self {
            Self::Ground => {}
            Self::Trench(_, ref mut dir, _) => *dir = Some(exit),
            Self::Hole(_) => {}
        }
    }
    pub fn exit(&self) -> Option<Direction> {
        match self {
            Self::Ground => None,
            Self::Trench(_, dir, _) => *dir,
            Self::Hole(_) => None,
        }
    }
    pub fn entrance(&self) -> Option<Direction> {
        match self {
            Self::Ground => None,
            Self::Trench(dir, _, _) => *dir,
            Self::Hole(_) => None,
        }
    }

    pub fn set_entrance(&mut self, entrance: Direction) {
        match self {
            Self::Ground => {}
            Self::Trench(ref mut dir, _, _) => *dir = Some(entrance),
            Self::Hole(_) => {}
        }
    }
    pub fn set_color(&mut self, color: Color) {
        match self {
            Self::Ground => {}
            Self::Trench(_, _, ref mut col) => *col = color,
            Self::Hole(ref mut col) => *col = Some(color),
        }
    }
}

#[derive(Debug, Default)]
pub struct Field {
    grid: Grid<Item>,
    instructions: Vec<Instruction>,
    pos: Pos,
}

impl Field {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn process(&mut self) {
        let instructions = self.instructions.clone();
        let mut pos = self.pos;
        for instruction in instructions {
            println!("Processing {:?} at {:?}", instruction, pos);
            for _ in 0..instruction.distance {
                self.grid[pos].set_exit(instruction.direction);
                pos = pos + instruction.direction.delta();
                self.grid.ensure(pos);
                let current = self.grid[pos].clone();
                println!("{:?}", pos);
                if let Item::Ground = current {
                    println!("digging trench");
                    self.grid[pos] = Item::Trench(None, None, instruction.color.clone());
                    self.grid[pos].set_entrance(instruction.direction.reverse());
                }
                if let Item::Trench(_, _, _) = current {
                    println!("setting trench");
                    self.grid[pos].set_entrance(instruction.direction.reverse());
                }
            }
        }
        self.grid[pos].set_exit(self.instructions[0].direction);
        self.pos = pos;
        println!(
            "final pos {:?} = {:?} {:?}",
            pos,
            self.grid[pos].entrance(),
            self.grid[pos].exit()
        );
    }

    pub fn count_holes(&mut self) -> (i32, i32) {
        let mut holes = 0;
        let mut trenches = 0;
        for y in self.grid.top_left.y..=self.grid.bottom_right.y {
            for x in self.grid.top_left.x..=self.grid.bottom_right.x {
                let item = &self.grid[(x, y).into()];
                if let Item::Trench(_, _, _) = item {
                    trenches += 1;
                }
                if let Item::Hole(_) = item {
                    holes += 1;
                }
            }
        }
        (holes, trenches)
    }
    pub fn dig_holes(&mut self) {
        let mut is_inside = false;

        for y in self.grid.top_left.y..=self.grid.bottom_right.y {
            for x in self.grid.top_left.x..=self.grid.bottom_right.x {
                let item = &self.grid[(x, y).into()];
                if let Item::Trench(Some(entrance), Some(exit), _) = item {
                    is_inside = match (entrance, exit) {
                        (Direction::North, Direction::South) => !is_inside,
                        (Direction::South, Direction::North) => !is_inside,
                        (Direction::South, Direction::West) => !is_inside,
                        (Direction::West, Direction::South) => !is_inside,
                        (Direction::South, Direction::East) => !is_inside,
                        (Direction::East, Direction::South) => !is_inside,
                        _ => is_inside,
                    }
                }
                if is_inside {
                    if let Item::Ground = item {
                        self.grid[(x, y).into()] = Item::Hole(None);
                    }
                }
            }
        }
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let instructions = parse::instructions(data).unwrap().1;
    let mut field = Field::new(instructions);
    field.process();
    println!("{:?}", field.grid);
    field.dig_holes();
    println!("{:?}", field.grid);
    let (holes, trenches) = field.count_holes();
    (holes + trenches) as u64
}

fn part2(data: &str) -> u64 {
    0
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
    use nom::character::complete::digit1;
    use nom::character::complete::line_ending;
    use nom::character::complete::one_of;
    use nom::combinator::map;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::{bytes::complete::tag, IResult};

    pub fn instruction(i: &str) -> IResult<&str, Instruction> {
        let (i, direction) = map(one_of("UDRL"), |e| match e {
            'U' => Direction::North,
            'D' => Direction::South,
            'R' => Direction::East,
            'L' => Direction::West,
            _ => panic!("Bad match"),
        })(i)?;
        let (i, distance) = map(delimited(tag(" "), digit1, tag(" ")), |e: &str| {
            e.parse::<i32>().unwrap()
        })(i)?;
        let (i, color) = map(
            delimited(tag("(#"), many1(one_of("01234567890abcdef")), tag(")")),
            |col| {
                let mut ret = "#".to_owned();
                ret.push_str(&col.iter().collect::<String>());
                Color(ret)
            },
        )(i)?;
        Ok((
            i,
            Instruction {
                direction,
                distance,
                color,
            },
        ))
    }

    pub fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(line_ending, instruction)(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_parse() {
        let instructions = parse::instructions(sample()).unwrap().1;
        assert_eq!(Direction::South, instructions[1].direction);
    }

    #[test]
    fn test_sample() {
        assert_eq!(62, part1(sample()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(0, part2(sample()));
    }
}
