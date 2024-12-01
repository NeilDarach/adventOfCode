use aoc_2023::aoc::*;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Map {
    boxes: Vec<Vec<Entry>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Insert(String, u8),
    Remove(String),
}

impl Operation {
    pub fn label(&self) -> String {
        match self {
            Self::Insert(lbl, _) => lbl.clone(),
            Self::Remove(lbl) => lbl.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Entry {
    Occupied(String, u8),
    Empty,
}

impl Entry {
    pub fn is_occupied(&self, label: &str) -> bool {
        match self {
            Self::Occupied(l, _) => l == label,
            _ => false,
        }
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            boxes: vec![vec![]; 256],
        }
    }

    pub fn power(&self) -> u64 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(boxid, bucket)| {
                bucket
                    .iter()
                    .enumerate()
                    .map(|(slotid, entry)| {
                        (boxid as u64 + 1)
                            * (slotid as u64 + 1)
                            * (if let Entry::Occupied(_, v) = entry {
                                *v as u64
                            } else {
                                0
                            })
                    })
                    .sum::<u64>()
            })
            .sum::<u64>()
    }

    pub fn compact(&mut self) {
        for id in 0..self.boxes.len() {
            self.boxes[id] = self.boxes[id]
                .iter()
                .cloned()
                .filter(|e| e != &Entry::Empty)
                .collect::<Vec<Entry>>();
        }
    }

    pub fn apply(&mut self, op: &Operation) {
        let id = hash(&op.label());
        match op {
            Operation::Insert(label, val) => {
                match self.boxes[id].iter().position(|e| e.is_occupied(label)) {
                    Some(i) => self.boxes[id][i] = Entry::Occupied(label.clone(), *val),
                    None => self.boxes[id].push(Entry::Occupied(label.clone(), *val)),
                }
            }
            Operation::Remove(label) => {
                if let Some(i) = self.boxes[id].iter().position(|e| e.is_occupied(label)) {
                    self.boxes[id][i] = Entry::Empty
                }
            }
        }
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let commands = parse::commands(data).unwrap().1;
    commands.iter().map(|e| hash(e) as u64).sum()
}

fn part2(data: &str) -> u64 {
    let ops = parse::operations(data).unwrap().1;
    let mut map = Map::new();
    ops.iter().map(|e| map.apply(e)).last();
    map.compact();
    map.power()
}

fn hash(input: &String) -> usize {
    let rv = input.chars().map(|e| e as u32).fold(0, |mut a, e| {
        a += e;
        a *= 17;
        a % 256
    }) as usize;
    println!("hash of {} is {}", input, rv);
    rv
}

fn main() {
    println!("Day 15 of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input());
    println!("Part 2 answer is {}", val);
}

mod parse {
    use super::*;
    use nom::branch::alt;
    use nom::character::complete::alpha1;
    use nom::character::complete::satisfy;
    use nom::character::is_alphanumeric;
    use nom::combinator::map;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::preceded;
    use nom::{bytes::complete::tag, IResult};

    pub fn operation(i: &str) -> IResult<&str, Operation> {
        let (i, label) = alpha1(i)?;
        let (i, op) = alt((
            map(tag("-"), |e| Operation::Remove(label.to_owned())),
            preceded(
                tag("="),
                map(value, |e: String| {
                    Operation::Insert(label.to_owned(), e.parse::<u8>().unwrap())
                }),
            ),
        ))(i)?;
        Ok((i, op))
    }

    pub fn value(i: &str) -> IResult<&str, String> {
        map(
            many1(satisfy(|c| {
                is_alphanumeric(c as u8) || c == '=' || c == '-'
            })),
            |e: Vec<char>| e.iter().collect::<String>(),
        )(i)
    }

    pub fn command(i: &str) -> IResult<&str, String> {
        value(i)
    }
    pub fn commands(i: &str) -> IResult<&str, Vec<String>> {
        separated_list1(tag(","), command)(i)
    }
    pub fn operations(i: &str) -> IResult<&str, Vec<Operation>> {
        separated_list1(tag(","), operation)(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_parse_sample() {
        let commands = parse::commands(sample()).unwrap().1;
        assert_eq!(4000, commands.len());
        assert_eq!("lhqrxp=8", commands[4]);
    }

    #[test]
    fn test_parse_operations() {
        let operations = parse::operations("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
            .unwrap()
            .1;
        assert_eq!(11, operations.len());
        assert_eq!(Operation::Remove("cm".to_owned()), operations[1]);
        assert_eq!(Operation::Insert("qp".to_owned(), 3), operations[2]);
    }
    #[test]
    fn test_hash() {
        assert_eq!(52, hash(&"HASH".to_owned()));
    }

    #[test]
    fn test_hashmap() {
        assert_eq!(
            145,
            part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );
    }
    #[test]
    fn test_sample_part_1() {
        assert_eq!(
            1320,
            part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );
    }
}
