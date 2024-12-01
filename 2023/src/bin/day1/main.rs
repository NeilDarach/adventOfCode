use aoc_2023::aoc::*;
use itertools::Itertools;

fn input_1() -> &'static str {
    include_str!("input-1.txt")
}

fn input_2() -> &'static str {
    include_str!("input-1.txt")
}

fn part1(data: &str) -> u64 {
    let mut value: u64 = 0;
    for line in data.split('\n') {
        if line.len() >= 2 {
            let mut first = line.clone().chars();
            let _ = first
                .take_while_ref(|x| !x.is_numeric())
                .collect::<String>();
            let first = first.next().unwrap();
            let mut last = line.clone().chars().rev();
            let _ = last.take_while_ref(|x| !x.is_numeric()).collect::<String>();
            let last = last.next().unwrap();
            let line_value = ((first as u8 - b'0') * 10) + (last as u8 - b'0');
            value += line_value as u64;
        }
    }
    value
}

fn part2(data: &str) -> u64 {
    let mut val: u64 = 0;
    for line in data.split('\n') {
        if !line.is_empty() {
            let (_, number) = parse::line_value(line).unwrap();
            println!("{} - {} : {}", line, number, (number + val));
            val += number;
        }
    }
    val
}

fn main() {
    println!("Day 1 of {}", version());
    let val = part1(input_1());
    println!("Part 1 answer is {}", val);
    let val = part2(input_2());
    println!("Part 2 answer is {}", val);
}

mod parse {
    use nom::character::complete::anychar;
    use nom::combinator::{peek, value};
    use nom::multi::many1;
    use nom::multi::many_till;
    use nom::{branch::alt, bytes::complete::tag, IResult};

    pub fn number(i: &str) -> IResult<&str, u8> {
        alt((
            value(1, alt((tag("one"), tag("1")))),
            value(2, alt((tag("two"), tag("2")))),
            value(3, alt((tag("three"), tag("3")))),
            value(4, alt((tag("four"), tag("4")))),
            value(5, alt((tag("five"), tag("5")))),
            value(6, alt((tag("six"), tag("6")))),
            value(7, alt((tag("seven"), tag("7")))),
            value(8, alt((tag("eight"), tag("8")))),
            value(9, alt((tag("nine"), tag("9")))),
        ))(i)
    }

    pub fn next_number(i: &str) -> IResult<&str, u8> {
        let (i, (_, val)) = many_till(anychar, peek(number))(i)?;
        let (i, _) = anychar(i)?;
        Ok((i, val))
    }

    pub fn number_list(i: &str) -> IResult<&str, Vec<u8>> {
        many1(next_number)(i)
    }

    pub fn line_value(i: &str) -> IResult<&str, u64> {
        let (i, numbers) = number_list(i)?;
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        Ok((i, ((first * 10) + last) as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_1() -> &'static str {
        include_str!("sample-1.txt")
    }

    fn sample_2() -> &'static str {
        include_str!("sample-2.txt")
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(142, part1(sample_1()));
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(281, part2(sample_2()));
    }

    #[test]
    fn test_numbers() {
        assert_eq!(parse::number("one").unwrap(), ("", 1));
        assert_eq!(parse::number("five6").unwrap(), ("6", 5));
        assert_eq!(parse::number("6").unwrap(), ("", 6));
        assert_eq!(parse::number("7seven").unwrap(), ("seven", 7));
    }
    #[test]
    fn test_nextnumber() {
        assert_eq!(parse::next_number("xoney").unwrap(), ("ney", 1));
        assert_eq!(parse::next_number("onetwoy").unwrap(), ("netwoy", 1));
    }

    #[test]
    fn test_numberlist() {
        assert_eq!(
            parse::number_list("1asdftwoasdfeight8x").unwrap(),
            ("x", vec![1, 2, 8, 8])
        );
        assert_eq!(
            parse::number_list("two1nine").unwrap(),
            ("ine", vec![2, 1, 9])
        );
        assert_eq!(
            parse::number_list("eighttwothree").unwrap(),
            ("hree", vec![8, 2, 3])
        );
        assert_eq!(
            parse::number_list("abcone2threexyz").unwrap(),
            ("hreexyz", vec![1, 2, 3])
        );
        assert_eq!(
            parse::number_list("123456789").unwrap(),
            ("", vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
        assert_eq!(
            parse::number_list("oonetwothreefourfivesixseveneightninen").unwrap(),
            ("inen", vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
    }

    #[test]
    fn test_values() {
        assert_eq!(parse::line_value("1asdftwoasdfeight8x").unwrap(), ("x", 18));
        assert_eq!(parse::line_value("two1nine").unwrap(), ("ine", 29));
        assert_eq!(parse::line_value("eighttwothree").unwrap(), ("hree", 83));
        assert_eq!(
            parse::line_value("abcone2threexyz").unwrap(),
            ("hreexyz", 13)
        );
        assert_eq!(parse::line_value("123456789").unwrap(), ("", 19));
        assert_eq!(
            parse::line_value("oonetwothreefourfivesixseveneightninen").unwrap(),
            ("inen", 19)
        );

        assert_eq!(parse::line_value("oneight").unwrap(), ("ight", 18));
    }
}
