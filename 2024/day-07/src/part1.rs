use crate::custom_error::AocError;

pub enum Operator {
    Plus,
    Times,
}

pub struct Equation {
    result: i64,
    components: Vec<i64>,
}

impl Equation {
    pub fn calibration(&self) -> i64 {
        self.components.iter().sum()
    }

    pub fn is_valid(&self) -> bool {
        self.can_make(self.components[0], Operator::Plus, &self.components[1..])
            || self.can_make(self.components[0], Operator::Times, &self.components[1..])
    }

    pub fn can_make(&self, total: i64, operator: Operator, components: &[i64]) -> bool {
        if total > self.result {
            return false;
        }
        if components.len() == 0 {
            return total == self.result;
        }
        let total = match operator {
            Operator::Plus => total + components[0],
            Operator::Times => total * components[0],
        };
        self.can_make(total, Operator::Plus, &components[1..])
            || self.can_make(total, Operator::Times, &components[1..])
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let eqs = parser::parse(input);
    Ok(eqs
        .iter()
        .filter(|e| e.is_valid())
        .map(|e| e.result)
        .sum::<i64>()
        .to_string())
}

mod parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{i64, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    pub fn parse(input: &str) -> Vec<Equation> {
        separated_list1(line_ending, equation)(input).unwrap().1
    }

    fn equation(input: &str) -> IResult<&str, Equation> {
        map(
            separated_pair(i64, tag(": "), separated_list1(tag(" "), i64)),
            |(a, b)| Equation {
                result: a,
                components: b,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("3749", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let eqs = parser::parse(SAMPLE);
        assert_eq!(9, eqs.len());
        assert_eq!(3267, eqs[1].result);
        assert_eq!(vec![17, 5], eqs[2].components);
        Ok(())
    }

    #[test]
    fn test_is_valid() -> miette::Result<()> {
        let eqs = parser::parse(SAMPLE);
        assert!(eqs[0].is_valid());
        assert!(eqs[1].is_valid());
        assert!(!eqs[2].is_valid());
        Ok(())
    }
}
