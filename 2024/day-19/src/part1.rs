use crate::custom_error::AocError;
use std::collections::VecDeque;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let puzzle = parser::parse(input);
    let unique = unique_towels(&puzzle.towels);
    let possible = puzzle
        .patterns
        .iter()
        .filter(|&e| can_make(e, &unique))
        .count();
    Ok(possible.to_string())
}

pub struct Puzzle {
    towels: Vec<String>,
    patterns: Vec<String>,
}

pub fn unique_towels(towels: &[String]) -> Vec<String> {
    let mut towels = towels.to_vec();
    towels.sort_by_key(|a| a.len());
    towels.reverse();
    let clone = towels.clone();
    towels
        .into_iter()
        .enumerate()
        .filter(|(i, e)| !can_make(e, &clone[(*i + 1)..]))
        .map(|(_i, e)| e.to_string())
        .collect::<Vec<_>>()
}

pub fn can_make(pattern: &str, towels: &[String]) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(pattern);
    while let Some(slice) = queue.pop_front() {
        if slice.is_empty() {
            return true;
        }
        for next_towel in towels {
            if slice.starts_with(next_towel) {
                queue.push_back(&slice[next_towel.len()..]);
            }
        }
    }

    false
}

mod parser {
    use super::*;

    pub fn parse(input: &str) -> Puzzle {
        let mut lines = input.lines();
        let towels = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        lines.next();
        let patterns = lines.map(|e| e.to_string()).collect::<Vec<_>>();
        Puzzle { towels, patterns }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("6", process(SAMPLE)?);
        Ok(())
    }
    #[test]
    fn test_parse() -> miette::Result<()> {
        let puzzle = parser::parse(SAMPLE);
        assert_eq!(8, puzzle.towels.len());
        assert_eq!("wr", puzzle.towels[1]);
        assert_eq!(8, puzzle.patterns.len());
        assert_eq!("bggr", puzzle.patterns[1]);
        Ok(())
    }

    #[test]
    fn test_can_make() -> miette::Result<()> {
        let puzzle = parser::parse(SAMPLE);
        assert!(can_make("brwrr", &puzzle.towels));
        assert!(!can_make("ubwu", &puzzle.towels));
        Ok(())
    }

    #[test]
    fn test_unique() -> miette::Result<()> {
        let puzzle = parser::parse(SAMPLE);
        assert_eq!(
            vec!["bwu", "wr", "g", "b", "r"],
            unique_towels(&puzzle.towels[..])
        );
        Ok(())
    }
}
