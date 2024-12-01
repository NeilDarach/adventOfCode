use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Item {
    Empty,
    Number(u32),
    Symbol(char),
}

impl Item {
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }
    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }
}

#[derive(Default, Debug)]
pub struct Engine {
    items: Vec<Vec<Item>>,
}

impl Engine {
    pub fn new(items: Vec<Vec<Item>>) -> Self {
        Self { items }
    }
    pub fn from_data(data: &str) -> Self {
        Engine::new(
            data.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '0'..='9' => Item::Number((c as u8 - b'0') as u32),
                            '.' => Item::Empty,
                            _ => Item::Symbol(c),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn gears(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for x in 0..self.items[0].len() {
            for y in 0..self.items.len() {
                if let Item::Symbol('*') = self.items[y][x] {
                    result.push((x, y))
                }
            }
        }
        result
    }

    pub fn adjacent_numbers(&self, x: usize, y: usize) -> Vec<u32> {
        let mut result = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(num) = self.number_at(x as i32 + dx, y as i32 + dy) {
                    result.push(num);
                }
            }
        }
        result.into_iter().unique().map(|e| e.2).collect()
    }

    pub fn number_at(&self, x: i32, y: i32) -> Option<(usize, usize, u32)> {
        if x < 0 || y < 0 || x >= self.items[0].len() as i32 || y >= self.items.len() as i32 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        match self.items[y][x] {
            Item::Empty => None,
            Item::Symbol(_) => None,
            Item::Number(_) => {
                let mut start = x;
                let mut end = x;
                while start > 0 && self.items[y][start - 1].is_number() {
                    start -= 1;
                }
                while end < self.items[y].len() - 1 && self.items[y][end + 1].is_number() {
                    end += 1;
                }
                let slice = &self.items[y][start..=end];
                Some((
                    start,
                    y,
                    slice.iter().fold(0, |a, e| {
                        if let Item::Number(v) = e {
                            v + (a * 10)
                        } else {
                            a
                        }
                    }),
                ))
            }
        }
    }

    pub fn part_numbers(&self) -> Vec<u32> {
        let mut result = vec![];
        for x in 0..self.items[0].len() {
            for y in 0..self.items.len() {
                if let Item::Symbol(s) = self.items[y][x] {
                    let mut numbers = self.adjacent_numbers(x, y);
                    dbg!(format!("{} - {:?}", s, &numbers));
                    result.append(&mut numbers);
                }
            }
        }
        result
    }
}

fn input_1() -> &'static str {
    include_str!("input-1.txt")
}

pub fn main() {
    println!("Part 1: {}", part1(input_1()));
    println!("Part 2: {}", part2(input_1()));
}

pub fn part1(data: &str) -> u32 {
    let engine = Engine::from_data(data);
    let numbers = engine.part_numbers();
    numbers.into_iter().sum()
}

pub fn part2(data: &str) -> u32 {
    let engine = Engine::from_data(data);
    let gears = engine.gears();
    gears
        .iter()
        .map(|g| engine.adjacent_numbers(g.0, g.1))
        .filter(|e| e.len() == 2)
        .map(|e| e[0] * e[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_sample() {
        assert_eq!(4361, part1(sample()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(467835, part2(sample()));
    }

    #[test]
    fn test_number_at() {
        let engine = Engine::from_data(sample());
        assert_eq!((0, 0, 467), engine.number_at(0, 0).unwrap());
        assert_eq!((0, 0, 467), engine.number_at(1, 0).unwrap());
        assert_eq!((0, 0, 467), engine.number_at(2, 0).unwrap());
        assert_eq!(None, engine.number_at(3, 0));
        assert_eq!((6, 2, 633), engine.number_at(7, 2).unwrap());
    }
    #[test]
    fn test_adjacent_number() {
        let engine = Engine::from_data(sample());

        let empty: Vec<u32> = vec![];
        assert_eq!(vec![467, 35], engine.adjacent_numbers(3, 1));
        assert_eq!(empty, engine.adjacent_numbers(0, 7));
    }
    #[test]
    fn test_part_numbers() {
        let engine = Engine::from_data(sample());
        let numbers = engine.part_numbers();
        dbg!(&numbers);
        assert_eq!(8, numbers.len());
    }
}
