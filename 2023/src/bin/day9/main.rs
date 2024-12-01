use aoc_2023::aoc::*;
use itertools::Itertools;

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> i64 {
    let lines = parse::sequences(data).unwrap().1;
    lines.iter().map(|e| process(e)).sum()
}

fn part2(data: &str) -> i64 {
    let lines = parse::sequences(data).unwrap().1;
    lines.iter().map(|e| preprocess(e)).sum()
}

fn intervals(i: &[i64]) -> Vec<i64> {
    i.iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect::<Vec<_>>()
}

fn process(i: &[i64]) -> i64 {
    let last = i[i.len() - 1];
    let next = intervals(i);
    last + if next.iter().all(|e| *e == 0) {
        0
    } else {
        process(&next)
    }
}
fn preprocess(i: &[i64]) -> i64 {
    let first = i[0];
    let next = intervals(i);
    first
        - if next.iter().all(|e| *e == 0) {
            0
        } else {
            preprocess(&next)
        }
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
    use nom::character::complete::line_ending;
    use nom::multi::many1;
    use nom::sequence::terminated;
    use nom::IResult;

    pub fn sequences(i: &str) -> IResult<&str, Vec<Vec<i64>>> {
        many1(terminated(space_separated_signed_numbers, line_ending))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_sample() {
        let lines = parse::sequences(sample()).expect("A list of numbers").1;

        assert_eq!(18, process(&lines[0]));
        assert_eq!(114, lines.iter().map(|e| process(e)).sum::<i64>())
    }

    #[test]
    fn test_sample_part_2() {
        let lines = parse::sequences(sample()).expect("A list of numbers").1;

        assert_eq!(5, preprocess(&lines[2]));
    }
}
