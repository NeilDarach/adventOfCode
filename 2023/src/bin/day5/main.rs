use aoc_2023::aoc::*;
use core::ops::Range;
use rangemap::RangeMap;
use std::collections::HashMap;

pub struct Almanack {
    seeds: Vec<u64>,
    seed_ranges: Vec<Range<u64>>,
    mappings: HashMap<String, Table>,
}

impl Almanack {
    pub fn follow(&self, table_name: &str, start: u64) -> u64 {
        let table = dbg!(self.mappings.get(table_name).unwrap());
        let val = table.translate(start);
        if table.next == "location" {
            val
        } else {
            self.follow(&table.next, val)
        }
    }
    pub fn follow_ranges(&self, table_name: &str, start: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let table = self.mappings.get(table_name).unwrap();
        println!("looking for {:?} in {:?}", &start, &table);
        let val = reduce_range_list(
            start
                .into_iter()
                .flat_map(|e| table.translate_range(e))
                .collect(),
        );
        dbg!(&val);
        if table.next == "location" {
            val
        } else {
            self.follow_ranges(&table.next, val)
        }
    }
}

#[derive(Default, Debug)]
pub struct Table {
    name: String,
    next: String,
    max: u64,
    translation: RangeMap<u64, Option<u64>>,
}

impl Table {
    pub fn new(name: &str, next: &str) -> Self {
        let name = name.to_owned();
        let next = next.to_owned();
        Table {
            name,
            next,
            ..Default::default()
        }
    }

    pub fn translate(&self, key: u64) -> u64 {
        if let Some((range, entry)) = self.translation.get_key_value(&key) {
            match entry {
                Some(val) => key - range.start + val,
                None => key,
            }
        } else {
            key
        }
    }

    pub fn translate_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut start = range.start;
        let end = range.end;
        let mut result = vec![];
        while start < end {
            if let Some((range, value)) = self.translation.get_key_value(&start) {
                println!("Found range {:?} range pointing to {:?}", range, value);
                let open = match value {
                    Some(v) => *v + (start - range.start),
                    None => start,
                };
                if range.end >= end {
                    start = end;
                } else {
                    start = range.end;
                }
                let close = match value {
                    Some(v) => *v + (end - range.start),
                    None => end,
                };
                result.push(open..close);
            } else {
                result.push(start..end);
                start = end;
            }
        }
        result
    }

    pub fn add(&mut self, start: u64, length: u64, target: u64) {
        println!("Adding {}..{}", start, start + length);
        if start > self.max {
            println!("Adding None for {}..{}", self.max, start);
            self.translation.insert(self.max..start, None);
            self.max = start + length;
        }
        if start + length > self.max {
            self.max = start + length;
        }
        self.translation.insert(start..start + length, Some(target));
    }

    pub fn extend(&mut self, length: u64) {
        if length > self.max {
            self.translation.insert(self.max..length, None);
            self.max = length;
        }
    }
}

fn reduce_range_list(mut list: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut result = vec![];
    list.sort_by(|a, b| (a.start, a.end).cmp(&(b.start, b.end)));
    let mut it = list.into_iter();
    if let Some(mut last) = it.next() {
        for current in it {
            if current.start <= last.end {
                last.end = current.end;
            } else {
                result.push(last);
                last = current;
            }
        }
        result.push(last);
    }
    result
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let almanack = parse::almanack(data).unwrap().1;
    let mut locations = almanack
        .seeds
        .iter()
        .map(|e| almanack.follow("seed", *e))
        .collect::<Vec<_>>();

    locations.sort();
    locations[0]
}

fn part2(data: &str) -> u64 {
    let almanack = parse::almanack(data).unwrap().1;
    let locations = almanack.follow_ranges("seed", almanack.seed_ranges.clone());
    dbg!(&locations);
    locations[0].start
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
    use nom::character::complete::alpha1;
    use nom::character::complete::line_ending;
    use nom::multi::many0;
    use nom::multi::many1;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn seeds(i: &str) -> IResult<&str, Vec<u64>> {
        let (i, _) = tag("seeds: ")(i)?;
        space_separated_numbers(i)
    }

    pub fn table(i: &str) -> IResult<&str, Table> {
        let (i, src) = terminated(alpha1, tag("-to-"))(i)?;
        let (i, dst) = terminated(alpha1, tag(" map:"))(i)?;
        let (i, _) = line_ending(i)?;
        let (i, values) = many1(terminated(space_separated_numbers, line_ending))(i)?;
        let (i, _) = many0(line_ending)(i)?;
        let mut table = Table::new(src, dst);
        for v in values {
            table.add(v[1], v[2], v[0]);
        }
        Ok((i, table))
    }

    pub fn almanack(i: &str) -> IResult<&str, Almanack> {
        let (i, seeds) = seeds(i)?;
        let (i, _) = many1(line_ending)(i)?;
        let (i, tables) = many1(table)(i)?;
        let mappings = tables.into_iter().map(|e| (e.name.to_owned(), e)).collect();
        let mut seed_ranges = vec![];
        let mut it = seeds.iter();
        while let Some(&start) = it.next() {
            seed_ranges.push(start..start + it.next().unwrap());
        }
        Ok((
            i,
            Almanack {
                seeds,
                seed_ranges,
                mappings,
            },
        ))
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
        assert_eq!(35, part1(sample()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(47, part2(sample()));
    }

    #[test]
    fn test_simple_input() {
        let mut almanack = parse::almanack(input()).unwrap().1;
        almanack.seed_ranges = vec![1..2];

        let locations = almanack.follow_ranges("seed", almanack.seed_ranges.clone());
        dbg!(&locations);

        assert_eq!(9, locations[0].start);
    }

    #[test]
    fn test_seeds() {
        assert_eq!(
            vec![79, 14, 55, 13],
            parse::seeds(sample().lines().next().unwrap()).unwrap().1
        );
    }

    #[test]
    fn test_table() {
        let input = "seed-to-soil map:
50 98 2
52 50 48
";
        let table = parse::table(input).unwrap().1;
        //assert_eq!(50, *table.translation.get(&(99_u64)).unwrap());
        assert_eq!(51, table.translate(99));
    }
    #[test]
    fn test_almanack() {
        let almanack = parse::almanack(sample()).unwrap().1;
        assert_eq!(vec![79, 14, 55, 13], almanack.seeds);
        assert_eq!(
            "soil".to_owned(),
            almanack.mappings.get("seed").unwrap().next
        );
        assert_eq!(7, almanack.mappings.len());
    }

    #[test]
    fn test_reduce_range_list() {
        assert_eq!(vec![1..2, 4..5], reduce_range_list(vec![1..2, 4..5]));
        assert_eq!(vec![1..2, 4..8], reduce_range_list(vec![1..2, 4..5, 5..8]));
        assert_eq!(vec![1..2, 4..8], reduce_range_list(vec![1..2, 4..5, 4..8]));
        assert_eq!(vec![1..2, 4..8], reduce_range_list(vec![4..5, 1..2, 4..8]));
        assert_eq!(
            vec![1..6, 8..12],
            reduce_range_list(vec![1..3, 8..12, 2..2, 2..6])
        );
    }
}
