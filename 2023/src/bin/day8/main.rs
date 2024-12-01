use aoc_2023::aoc::*;
use std::collections::HashMap;

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let (directions, map) = parse::map(data).unwrap().1;
    get_count(&directions, &map, "AAA")
}

fn get_count(directions: &str, map: &HashMap<String, (String, String)>, start: &str) -> u64 {
    let mut count = 0;
    let mut location = start;
    for d in directions.chars().cycle() {
        let node = map.get(location).unwrap();
        count += 1;
        location = if d == 'L' { &node.0 } else { &node.1 };
        if location.ends_with('Z') {
            break;
        }
    }

    count
}

fn part2(data: &str) -> u64 {
    let (directions, map) = parse::map(data).unwrap().1;
    let locations = map.keys().filter(|e| e.ends_with('A')).collect::<Vec<_>>();
    let counts = locations
        .iter()
        .map(|e| get_count(&directions, &map, e))
        .collect::<Vec<_>>();

    counts.into_iter().reduce(num::integer::lcm).unwrap()
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
    use nom::character::complete::alphanumeric1;
    use nom::character::complete::line_ending;
    use nom::multi::many0;
    use nom::multi::many1;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    fn node(i: &str) -> IResult<&str, (String, (String, String))> {
        let (i, key) = terminated(alphanumeric1, tag(" = ("))(i)?;
        let (i, left) = terminated(alphanumeric1, tag(", "))(i)?;
        let (i, right) = terminated(alphanumeric1, tag(")"))(i)?;
        let (i, _) = many0(line_ending)(i)?;
        Ok((i, (key.to_owned(), (left.to_owned(), right.to_owned()))))
    }

    pub fn map(i: &str) -> IResult<&str, (String, HashMap<String, (String, String)>)> {
        let (i, directions) = terminated(alphanumeric1, line_ending)(i)?;
        let (i, _) = many0(line_ending)(i)?;
        let (i, lines) = many1(node)(i)?;
        let mut map = HashMap::new();
        for (key, val) in lines {
            map.insert(key, val);
        }
        Ok((i, (directions.to_owned(), map)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    fn sample2() -> &'static str {
        include_str!("sample2.txt")
    }

    fn sample3() -> &'static str {
        include_str!("sample3.txt")
    }

    #[test]
    fn test_parse_sample() {
        let (directions, map) = parse::map(sample()).unwrap().1;
        assert_eq!("RL", directions);
        assert_eq!(7, map.len());
    }

    #[test]
    fn test_sample() {
        assert_eq!(2, part1(sample()));
    }

    #[test]
    fn test_sample2() {
        assert_eq!(6, part1(sample2()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(6, part2(sample3()));
    }
}
