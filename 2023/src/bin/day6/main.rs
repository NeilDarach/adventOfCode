use aoc_2023::aoc::*;

fn input() -> &'static str {
    include_str!("input.txt")
}

pub fn winning_results_count(distance: u64, time: u64) -> u64 {
    let mut c = 0;
    for i in 1..time {
        if (time - i) * i > distance {
            c += 1
        };
    }
    c
}

fn part1(data: &str) -> u64 {
    let races = parse::races(data).unwrap().1;
    races
        .into_iter()
        .map(|(time, distance)| winning_results_count(distance, time))
        .product()
}

fn part2(data: &str) -> u64 {
    let (time, distance) = parse::races2(data).unwrap().1;
    winning_results_count(distance, time)
}

fn main() {
    println!("Day 6 of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input());
    println!("Part 2 answer is {}", val);
}

mod parse {
    use super::*;
    use nom::character::complete::anychar;
    use nom::character::complete::line_ending;
    use nom::character::complete::multispace1;
    use nom::multi::many_till;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn races(i: &str) -> IResult<&str, Vec<(u64, u64)>> {
        let (i, _) = terminated(tag("Time:"), multispace1)(i)?;
        let (i, times) = space_separated_numbers(i)?;
        let (i, _) = line_ending(i)?;
        let (i, _) = terminated(tag("Distance:"), multispace1)(i)?;
        let (i, distances) = space_separated_numbers(i)?;
        let pairs = times.into_iter().zip(distances).collect();
        Ok((i, pairs))
    }

    pub fn races2(i: &str) -> IResult<&str, (u64, u64)> {
        let (i, _) = terminated(tag("Time:"), multispace1)(i)?;
        let (i, (time, _)) = many_till(anychar, line_ending)(i)?;
        let (i, _) = terminated(tag("Distance:"), multispace1)(i)?;
        let (i, (distance, _)) = many_till(anychar, line_ending)(i)?;
        let time = time
            .into_iter()
            .filter(|&e| e != ' ')
            .collect::<String>()
            .parse()
            .unwrap();
        dbg!(&time);
        let distance = distance
            .into_iter()
            .filter(|&e| e != ' ')
            .collect::<String>()
            .parse()
            .unwrap();
        Ok((i, (time, distance)))
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
        assert_eq!(288, part1(sample()));
    }

    #[test]
    fn test_parse() {
        let races = parse::races(sample()).unwrap().1;
        assert_eq!(3, races.len());
        assert_eq!((15, 40), races[1]);
    }

    #[test]
    fn test_parse2() {
        let race = parse::races2(sample()).unwrap().1;
        assert_eq!((71530, 940200), race);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(71503, part2(sample()));
    }
}
