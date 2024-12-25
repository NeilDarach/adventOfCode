use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (keys, locks) = parser::parse(input);

    let mut result = 0;
    for l in &locks[..] {
        for k in &keys[..] {
            if l.may_fit(k) {
                result += 1;
            }
        }
    }
    Ok(result.to_string())
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Item {
    Key([i32; 5]),
    Lock([i32; 5]),
}

impl Item {
    pub fn may_fit(&self, item: &Self) -> bool {
        if let Item::Lock(l) = self {
            if let Item::Key(k) = item {
                for i in 0..5 {
                    if k[i] + l[i] > 5 {
                        return false;
                    }
                }
                return true;
            }
        }
        false
    }
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, one_of},
        multi::{count, separated_list1},
        sequence::terminated,
        IResult,
    };
    pub fn parse(input: &str) -> (Vec<Item>, Vec<Item>) {
        let (_input, items) = separated_list1(line_ending, key_or_lock)(input).unwrap();
        let mut keys = vec![];
        let mut locks = vec![];
        for i in items {
            match i {
                Item::Key(_) => keys.push(i),
                Item::Lock(_) => locks.push(i),
            }
        }
        (keys, locks)
    }

    pub fn key_or_lock(input: &str) -> IResult<&str, Item> {
        alt((key, lock))(input)
    }

    pub fn key(input: &str) -> IResult<&str, Item> {
        let (input, _) = terminated(tag("....."), line_ending)(input)?;
        let (input, pins) = body(input)?;
        let (input, _) = terminated(tag("#####"), line_ending)(input)?;
        Ok((input, Item::Key(pins)))
    }

    pub fn lock(input: &str) -> IResult<&str, Item> {
        let (input, _) = terminated(tag("#####"), line_ending)(input)?;
        let (input, pins) = body(input)?;
        let (input, _) = terminated(tag("....."), line_ending)(input)?;
        Ok((input, Item::Lock(pins)))
    }

    pub fn body(input: &str) -> IResult<&str, [i32; 5]> {
        let (input, lines) = count(terminated(count(one_of(".#"), 5), line_ending), 5)(input)?;
        let mut result = [0, 0, 0, 0, 0];
        for i in 0..5 {
            for j in 0..5 {
                if lines[j][i] == '#' {
                    result[i] += 1;
                }
            }
        }
        Ok((input, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("3", process(SAMPLE_1)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let (keys, locks) = parser::parse(SAMPLE_1);
        assert_eq!(3, keys.len());
        assert_eq!(2, locks.len());
        assert_eq!(Item::Lock([0, 5, 3, 4, 3]), locks[0]);
        assert_eq!(Item::Key([5, 0, 2, 1, 3]), keys[0]);
        assert!(!locks[0].may_fit(&keys[0]));
        assert!(!locks[0].may_fit(&keys[1]));
        assert!(locks[0].may_fit(&keys[2]));
        Ok(())
    }
}
