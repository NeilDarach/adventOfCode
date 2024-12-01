use aoc_2023::aoc::*;

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    0
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
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::sequence::pair;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_sample() {
        assert_eq!(0, part1(sample()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(0, part2(sample()));
    }
}
