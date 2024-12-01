use std::ops::Add;

use aoc_2023::aoc::*;

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}
impl Add for Cubes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: (self.red + rhs.red),
            green: (self.green + rhs.green),
            blue: (self.blue + rhs.blue),
        }
    }
}
impl Cubes {
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    pub fn combine(&self, other: &Self) -> Self {
        Self {
            red: (self.red.max(other.red)),
            green: (self.green.max(other.green)),
            blue: (self.blue.max(other.blue)),
        }
    }
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn red(count: u32) -> Self {
        Self {
            red: count,
            green: 0,
            blue: 0,
        }
    }
    pub fn green(count: u32) -> Self {
        Self {
            red: 0,
            green: count,
            blue: 0,
        }
    }
    pub fn blue(count: u32) -> Self {
        Self {
            red: 0,
            green: 0,
            blue: count,
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

fn input_1() -> &'static str {
    include_str!("input-1.txt")
}

fn input_2() -> &'static str {
    include_str!("input-1.txt")
}

fn part1(data: &str) -> u32 {
    let base = Cubes::new(12, 13, 14);
    let result = data
        .lines()
        .map(|l| parse::game(l).unwrap().1)
        .filter(|(_, game)| game.iter().all(|e| base.contains(e)))
        .map(|(id, _)| id)
        .sum();
    result
}

fn part2(data: &str) -> u32 {
    let result = data
        .lines()
        .map(|l| parse::game(l).unwrap().1)
        .map(|(_, game)| game.iter().fold(Cubes::default(), |acc, e| acc.combine(e)))
        .map(|cubes| cubes.power())
        .sum();
    result
}

fn main() {
    println!("Day 1 of {}", version());
    let val = part1(input_1());
    println!("Part 1 answer is {}", val);
    let val = part2(input_2());
    println!("Part 2 answer is {}", val);
}

pub mod parse {
    use super::Cubes;
    use nom::character::complete::digit1;
    use nom::multi::separated_list1;
    use nom::{branch::alt, bytes::complete::tag, IResult};

    pub fn game(i: &str) -> IResult<&str, (u32, Vec<Cubes>)> {
        let (i, _) = tag("Game ")(i)?;
        let (i, id) = digit1(i)?;
        let id = id.parse::<u32>().unwrap();
        let (i, _) = tag(": ")(i)?;
        let (i, cubes) = group(i)?;
        Ok((i, (id, cubes)))
    }

    pub fn set(i: &str) -> IResult<&str, Cubes> {
        let (i, cubes) = separated_list1(tag(", "), alt((red, green, blue)))(i)?;
        Ok((
            i,
            cubes
                .into_iter()
                .fold(Cubes::default(), |acc, each| acc + each),
        ))
    }

    pub fn group(i: &str) -> IResult<&str, Vec<Cubes>> {
        separated_list1(tag("; "), set)(i)
    }

    pub fn red(i: &str) -> IResult<&str, Cubes> {
        let (i, count) = digit1(i)?;
        let (i, _) = tag(" red")(i)?;
        Ok((i, Cubes::red(count.parse::<u32>().unwrap())))
    }
    pub fn green(i: &str) -> IResult<&str, Cubes> {
        let (i, count) = digit1(i)?;
        let (i, _) = tag(" green")(i)?;
        Ok((i, Cubes::green(count.parse::<u32>().unwrap())))
    }
    pub fn blue(i: &str) -> IResult<&str, Cubes> {
        let (i, count) = digit1(i)?;
        let (i, _) = tag(" blue")(i)?;
        Ok((i, Cubes::blue(count.parse::<u32>().unwrap())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_1() -> &'static str {
        include_str!("sample-1.txt")
    }

    fn sample_2() -> &'static str {
        include_str!("sample-1.txt")
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(8, part1(sample_1()));
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(2286, part2(sample_2()));
    }

    #[test]
    fn test_parsegame() {
        let (_, (id, cubes)) =
            parse::game("Game 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        assert_eq!(2, id);
        assert_eq!(3, cubes.len());
        assert_eq!(3, cubes[0].blue);
    }
    #[test]
    fn test_set() {
        let (_, cubes) = parse::set("3 blue, 4 red").unwrap();
        assert_eq!(4, cubes.red);
        assert_eq!(0, cubes.green);
        assert_eq!(3, cubes.blue);
    }
    #[test]
    fn test_group() {
        let (_, cubes) = parse::group("3 blue, 4 red; 1 red, 2 green, 6 blue").unwrap();
        assert_eq!(2, cubes.len());
    }

    #[test]
    fn test_contains() {
        assert!(Cubes::green(3).contains(&Cubes::default()));
        assert!(!Cubes::default().contains(&Cubes::blue(1)));
    }
}
