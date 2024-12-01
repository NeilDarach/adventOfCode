use aoc_2023::aoc::*;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Rock {
    Round,
    Square,
    Empty,
}

fn cycle(mut plate: Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let mut plate = plate;
    let mut tilted;
    for _ in 0..4 {
        tilted = plate.into_iter().map(tilt).collect::<Vec<Vec<Rock>>>();
        plate = transpose(&tilted);
    }
    println!("{:?}", &plate);
    plate
}

fn tilt(mut row: Vec<Rock>) -> Vec<Rock> {
    let mut current = 0;
    let end = row.len();
    let mut empty;
    loop {
        while current < end && row[current] != Rock::Empty {
            current += 1
        }
        println!("empty is {}", current);
        empty = current;
        while current < end && row[current] == Rock::Empty {
            current += 1
        }
        if current == end {
            return row;
        }
        if row[current] == Rock::Round {
            println!("round is {}, swapping", current);
            row[empty] = Rock::Round;
            row[current] = Rock::Empty;
            current = empty + 1
        }
        if row[current] == Rock::Square {
            println!("square is {}, moving on", current);
            current += 1;
        }
    }
}

fn load(row: &Vec<Rock>) -> u64 {
    row.iter().rev().enumerate().fold(0, |a, (i, &e)| {
        if e == Rock::Round {
            a + (i + 1) as u64
        } else {
            a
        }
    })
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let plate = parse::plate(data).unwrap().1;
    let tilted = plate.into_iter().map(tilt).collect::<Vec<Vec<_>>>();
    tilted.iter().map(load).sum::<u64>()
}

fn part2(data: &str, count: u64) -> u64 {
    let mut plate = parse::plate(data).unwrap().1;
    for _ in 0..count {
        plate = cycle(plate);
    }
    println!("{:?}", plate);
    plate.iter().map(load).sum::<u64>()
}

fn main() {
    println!("Day x of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input(), 1000000000);
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
    use nom::sequence::pair;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn row(i: &str) -> IResult<&str, Vec<Rock>> {
        many1(map(one_of("O.#"), |e| match e {
            '.' => Rock::Empty,
            'O' => Rock::Round,
            '#' => Rock::Square,
            _ => panic!("Bad match"),
        }))(i)
    }

    pub fn plate(i: &str) -> IResult<&str, Vec<Vec<Rock>>> {
        let (i, lines) = separated_list1(line_ending, row)(i)?;
        Ok((i, transpose(&lines)))
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
        let plate = parse::plate(sample()).unwrap().1;
        assert_eq!(Rock::Round, plate[0][0]);
        assert_eq!(Rock::Round, plate[0][1]);
        assert_eq!(Rock::Empty, plate[0][2]);
    }

    #[test]
    fn test_tilt() {
        let row = vec![
            Rock::Empty,
            Rock::Empty,
            Rock::Round,
            Rock::Square,
            Rock::Round,
            Rock::Empty,
            Rock::Round,
        ];
        let row = tilt(row);
        assert_eq!(
            vec![
                Rock::Round,
                Rock::Empty,
                Rock::Empty,
                Rock::Square,
                Rock::Round,
                Rock::Round,
                Rock::Empty
            ],
            row
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(136, part1(sample()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(64, part2(sample(), 1));
    }
}
