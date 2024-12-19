use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut puzzle = parser::parse(input);
    let possible = puzzle.process();
    Ok(possible.iter().sum::<i64>().to_string())
}

pub struct Puzzle {
    towels: Vec<String>,
    patterns: Vec<String>,
    cache: HashMap<String, i64>,
}

impl Puzzle {
    pub fn process(&mut self) -> Vec<i64> {
        self.patterns
            .clone()
            .iter()
            .map(|e| self.arrangements(e))
            .collect::<Vec<_>>()
    }
    pub fn arrangements(&mut self, pattern: &str) -> i64 {
        let mut count = 0;
        if pattern.is_empty() {
            return 1;
        }
        if let Some(v) = self.cache.get(pattern) {
            return *v;
        }
        for t in self.towels.clone() {
            if pattern.starts_with(&t) {
                count += self.arrangements(&pattern[t.len()..])
            }
        }
        self.cache.insert(pattern.to_string(), count);
        count
    }
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
        let cache = HashMap::<String, i64>::default();
        Puzzle {
            towels,
            patterns,
            cache,
        }
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
        assert_eq!("16", process(SAMPLE)?);
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
}
