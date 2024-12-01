use aoc_2023::aoc::*;
use std::iter::zip;

#[derive(Debug, Clone)]
pub struct Grid(Vec<String>);

impl Grid {
    pub fn horizontal_reflection(&self) -> Option<u32> {
        Self::find_reflection(&self.0)
    }

    pub fn unsmudged_horizontal_reflection(&self) -> Option<u32> {
        if let Some(unsmudged) = Self::unsmudge(&self.0) {
            for g in unsmudged {
                if let Some(r) = Self::find_reflection(&g) {
                    return Some(r);
                }
            }
            None
        } else {
            println!("Can't find horizontal smudge");
            None
        }
    }

    pub fn unsmudged_vertical_reflection(&self) -> Option<u32> {
        let lines = Self::transpose(&self.0);
        if let Some(unsmudged) = Self::unsmudge(&lines) {
            println!("Unsmudged vertical");
            for g in unsmudged {
                if let Some(r) = Self::find_reflection(&g) {
                    return Some(r);
                }
            }
            None
        } else {
            None
        }
    }

    pub fn line_as_int(line: &String) -> u64 {
        line.chars()
            .fold(0, |a, e| (a << 1) + if e == '.' { 0 } else { 1 })
    }

    pub fn has_single_difference(l1: &String, l2: &String) -> bool {
        let v1 = Self::line_as_int(l1);
        let v2 = Self::line_as_int(l2);
        let diff = v1 ^ v2;
        /*
        println!(
            "{} l1\n{} l2\n v1 {}, v2 {}, diff {}",
            l1,
            l2,
            v1,
            v2,
            diff.count_ones()
        );*/
        diff.count_ones() == 1
    }

    pub fn unsmudge(lines: &Vec<String>) -> Option<(Vec<Vec<String>>)> {
        let mut ret = vec![];
        for i in 0..lines.len() - 1 {
            for j in i..lines.len() {
                if Self::has_single_difference(&lines[i], &lines[j]) {
                    let mut left = lines.clone();
                    let mut right = lines.clone();
                    left[i] = lines[j].clone();
                    right[j] = lines[i].clone();
                    ret.push(left);
                    ret.push(right);
                    println!("Replacing {} with {}", i, j);
                }
            }
        }
        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }

    fn find_reflection(lines: &Vec<String>) -> Option<u32> {
        let end = lines.len();
        for i in 1..end {
            println!(
                "Comparing {} and {}\n{}\n{}",
                i - 1,
                i,
                lines[i - 1],
                lines[i]
            );
            if lines[i - 1] == lines[i] {
                println!("Considering {}", i);
                let mut matches = true;
                for (i1, i2) in zip((0..i).rev(), i..end) {
                    println!("{} vs {}", i1, i2);
                    matches = matches && (lines[i1] == lines[i2])
                }
                if matches {
                    return Some(i as u32);
                }
            }
        }
        None
    }
    fn transpose(input: &Vec<String>) -> Vec<String> {
        let intermediate = input
            .iter()
            .map(|e| e.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        (0..intermediate[0].len())
            .map(|i| {
                intermediate
                    .iter()
                    .map(|inner| inner[i])
                    .collect::<Vec<_>>()
            })
            .map(|e| e.iter().collect::<String>())
            .collect()
    }

    pub fn vertical_reflection(&self) -> Option<u32> {
        let lines = Self::transpose(&self.0);
        Self::find_reflection(&lines)
    }
    pub fn score(&self, i: usize) -> u32 {
        if let Some(v) = self.horizontal_reflection() {
            return 100 * v;
        }
        if let Some(v) = self.vertical_reflection() {
            return v;
        }
        panic!("Not found in {} {:?}", i, &self);
    }

    pub fn unsmudged_score(&self, i: usize) -> u32 {
        println!("Processing {}", i);
        if let Some(v) = self.unsmudged_horizontal_reflection() {
            return 100 * v;
        }
        if let Some(v) = self.unsmudged_vertical_reflection() {
            return v;
        }
        panic!("Not found in {} {:?}", i, &self);
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u32 {
    let grids = parse::grids(data).unwrap().1;
    grids.iter().enumerate().map(|(i, e)| e.score(i)).sum()
}

fn part2(data: &str) -> u32 {
    let grids = parse::grids(data).unwrap().1;
    grids
        .iter()
        .enumerate()
        .map(|(i, e)| e.unsmudged_score(i))
        .sum()
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
    use nom::sequence::pair;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn grid(i: &str) -> IResult<&str, Grid> {
        let (i, lines) = separated_list1(
            line_ending,
            map(many1(one_of("#.")), |e| e.iter().collect::<String>()),
        )(i)?;
        Ok((
            i,
            Grid(lines.iter().map(|e| e.to_owned()).collect::<Vec<_>>()),
        ))
    }
    pub fn grids(i: &str) -> IResult<&str, Vec<Grid>> {
        separated_list1(many1(line_ending), grid)(i)
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
        let grids = parse::grids(sample()).unwrap().1;
        println!("{:?}", grids);
        assert_eq!(None, grids[0].horizontal_reflection());
        assert_eq!(Some(4), grids[1].horizontal_reflection());
        assert_eq!(Some(5), grids[0].vertical_reflection());
        assert_eq!(None, grids[1].vertical_reflection());
    }
    #[test]
    fn test_sample_part_1() {
        assert_eq!(405, part1(sample()));
    }

    #[test]
    fn test_smudges() {
        assert!(Grid::has_single_difference(
            &"....#".to_owned(),
            &".#..#".to_owned()
        ));
        assert!(!Grid::has_single_difference(
            &"....#".to_owned(),
            &"##..#".to_owned()
        ));
        assert!(!Grid::has_single_difference(
            &"#.##..##.".to_owned(),
            &"..#.##.#.".to_owned()
        ));

        let unsmudged = Grid::unsmudge(
            &"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
                .lines()
                .map(|e| e.to_owned())
                .collect(),
        );
        let unsmudged = unsmudged.unwrap();
        assert_eq!(unsmudged[0][0], unsmudged[0][1]);

        let unsmudged = Grid::unsmudge(
            &"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
                .lines()
                .map(|e| e.to_owned())
                .collect(),
        );
        let unsmudged = unsmudged.unwrap();
        println!("{:?}", unsmudged[0]);
        assert_eq!(unsmudged[0][0], unsmudged[0][5]);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(400, part2(sample()));
    }
}
