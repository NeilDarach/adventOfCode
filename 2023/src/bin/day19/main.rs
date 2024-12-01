use aoc_2023::aoc::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    pub fn get(&self, attr: &str) -> u32 {
        match attr {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("bad attr"),
        }
    }
    pub fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
pub enum Destination {
    Rule(String),
    Accept,
    Reject,
}

#[derive(Clone, Debug)]
pub enum Rule {
    Gt(String, u32, Destination),
    Lt(String, u32, Destination),
    Eq(String, u32, Destination),
    Default(Destination),
}

impl Rule {
    pub fn matches(&self, part: &Part) -> Option<Destination> {
        match self {
            Self::Default(d) => Some(d.clone()),
            Self::Gt(attr, val, d) => {
                if part.get(attr) > *val {
                    Some(d.clone())
                } else {
                    None
                }
            }
            Self::Lt(attr, val, d) => {
                if part.get(attr) < *val {
                    Some(d.clone())
                } else {
                    None
                }
            }
            Self::Eq(attr, val, d) => {
                if part.get(attr) == *val {
                    Some(d.clone())
                } else {
                    None
                }
            }
        }
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn process(map: &HashMap<String, Vec<Rule>>, part: &Part) -> Destination {
    let mut rules = map.get("in").unwrap();
    loop {
        for rule in rules {
            println!("Checking {:?} against {:?}", &part, &rule);
            match rule.matches(part) {
                Some(Destination::Accept) => return Destination::Accept,
                Some(Destination::Reject) => return Destination::Reject,
                Some(Destination::Rule(r)) => {
                    println!("Jump to {}", &r);
                    rules = map.get(&r).unwrap();
                    break;
                }
                None => {}
            }
        }
    }
}

fn part1(data: &str) -> u64 {
    let (map, parts) = parse::input(data).unwrap().1;
    let mut accepted = vec![];
    let mut rejected = vec![];

    for part in parts {
        if let Destination::Accept = process(&map, &part) {
            accepted.push(part);
        } else {
            rejected.push(part);
        }
    }

    accepted.iter().map(|e| e.value() as u64).sum()
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
    use nom::branch::alt;
    use nom::character::complete::alpha1;
    use nom::character::complete::digit1;
    use nom::character::complete::line_ending;
    use nom::character::complete::one_of;
    use nom::combinator::map;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn rule_default(i: &str) -> IResult<&str, Rule> {
        let (i, dest) = alpha1(i)?;
        let dest = match dest {
            "A" => Rule::Default(Destination::Accept),
            "R" => Rule::Default(Destination::Reject),
            _ => Rule::Default(Destination::Rule(dest.to_owned())),
        };
        Ok((i, dest))
    }

    pub fn rule_test(i: &str) -> IResult<&str, Rule> {
        let (i, attr) = map(one_of("amsx"), |e| {
            let mut s = "".to_owned();
            s.push(e);
            s
        })(i)?;
        let (i, test) = one_of("<=>")(i)?;
        let (i, val) = map(digit1, |e: &str| e.parse::<u32>().unwrap())(i)?;
        let (i, dest) = preceded(tag(":"), alpha1)(i)?;
        let dest = match dest {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            _ => Destination::Rule(dest.to_owned()),
        };

        let rule = match test {
            '>' => Rule::Gt(attr, val, dest),
            '<' => Rule::Lt(attr, val, dest),
            '=' => Rule::Eq(attr, val, dest),
            _ => panic!("Bad op"),
        };

        Ok((i, rule))
    }
    pub fn rule(i: &str) -> IResult<&str, (String, Vec<Rule>)> {
        let (i, name) = terminated(alpha1, tag("{"))(i)?;
        let (i, rules) = terminated(
            separated_list1(one_of(","), alt((rule_test, rule_default))),
            tag("}"),
        )(i)?;
        Ok((i, (name.to_owned(), rules)))
    }

    pub fn rules(i: &str) -> IResult<&str, HashMap<String, Vec<Rule>>> {
        let (i, items) = separated_list1(line_ending, rule)(i)?;
        let mut map = HashMap::default();
        for (name, list) in items {
            map.insert(name, list);
        }
        Ok((i, map))
    }

    pub fn part_components(i: &str) -> IResult<&str, Part> {
        let (i, x) = delimited(
            tag("x="),
            map(digit1, |e: &str| e.parse::<u32>().unwrap()),
            tag(","),
        )(i)?;
        let (i, m) = delimited(
            tag("m="),
            map(digit1, |e: &str| e.parse::<u32>().unwrap()),
            tag(","),
        )(i)?;
        let (i, a) = delimited(
            tag("a="),
            map(digit1, |e: &str| e.parse::<u32>().unwrap()),
            tag(","),
        )(i)?;
        let (i, s) = preceded(tag("s="), map(digit1, |e: &str| e.parse::<u32>().unwrap()))(i)?;
        Ok((i, Part { x, m, a, s }))
    }
    pub fn part(i: &str) -> IResult<&str, Part> {
        delimited(tag("{"), part_components, tag("}"))(i)
    }

    pub fn parts(i: &str) -> IResult<&str, Vec<Part>> {
        separated_list1(line_ending, part)(i)
    }

    pub fn input(i: &str) -> IResult<&str, (HashMap<String, Vec<Rule>>, Vec<Part>)> {
        let (i, rules) = rules(i)?;
        let (i, _) = many1(line_ending)(i)?;
        let (i, parts) = parts(i)?;
        Ok((i, (rules, parts)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }
    #[test]
    fn test_parse_parts() {
        let parts = parse::parts(
            "{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}",
        )
        .unwrap()
        .1;
        assert_eq!(2, parts.len());
        assert_eq!(44, parts[1].m);
    }

    #[test]
    fn test_parse_rules() {
        let rules = parse::rules(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}",
        )
        .unwrap()
        .1;
        assert_eq!(3, rules.len());
        assert_eq!(2, rules.get("pv").unwrap().len());
    }

    #[test]
    fn test_sample() {
        assert_eq!(19114, part1(sample()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(0, part2(sample()));
    }
}
