use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let machines = parser::parse(input);
    let result: i32 = machines.iter().filter_map(|e| e.solve(0)).sum();
    Ok(result.to_string())
}
pub struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

impl Machine {
    pub fn solve(&self, offset: i32) -> Option<i32> {
        let mut result = None;
        let target = (self.prize.0 + offset, self.prize.1 + offset);
        let da = self.button_a.0 - self.button_a.1;
        let db = self.button_b.0 - self.button_b.1;
        let dt = target.0 - target.1;

        let _max_i = ((1 + target.0) / self.button_a.0).min((1 + target.0) / self.button_a.0);
        let max_j = ((1 + target.1) / self.button_a.1).min((1 + target.1) / self.button_a.1);
        let _min_i = ((target.0 - (self.button_b.0 * max_j)) / self.button_a.0)
            .max((target.0 - (self.button_b.0 * max_j)) / self.button_a.0);

        if db == 0 {
            if (target.0 - target.1) % (self.button_a.0 - self.button_a.1) == 0 {
                let i = (target.0 - target.1) / (self.button_a.0 - self.button_a.1);
                if (target.0 - (i * self.button_a.0)) % self.button_b.0 == 0 {
                    let j = (target.0 - (i * self.button_a.0)) / self.button_b.0;
                    let cost = (3 * i) + j;
                    result = match result {
                        None => Some(cost),
                        Some(c) if c >= cost => Some(cost),
                        _ => result,
                    }
                }
            }
        } else {
            for i in 0..=100 {
                if (dt - (da * i)) % db == 0 {
                    let j = (dt - (da * i)) / db;
                    if j > 0
                        && target
                            == (
                                ((self.button_a.0 * i) + (self.button_b.0 * j)),
                                ((self.button_a.1 * i) + (self.button_b.1 * j)),
                            )
                    {
                        let cost = (3 * i) + j;
                        result = match result {
                            None => Some(cost),
                            Some(c) if c >= cost => Some(cost),
                            _ => result,
                        }
                    }
                }
            }
        }
        result
    }
}

mod parser {
    use nom::{
        bytes::complete::tag, character::complete::i32, character::complete::line_ending,
        multi::many1, multi::separated_list1, IResult,
    };

    use super::*;
    pub fn parse(input: &str) -> Vec<Machine> {
        let (_, machines) = separated_list1(many1(line_ending), machine)(input).unwrap();
        machines
    }

    pub fn machine(input: &str) -> IResult<&str, Machine> {
        let (input, (ax, ay)) = button_a(input)?;
        let (input, (bx, by)) = button_b(input)?;
        let (input, (px, py)) = prize(input)?;
        Ok((
            input,
            Machine {
                button_a: (ax, ay),
                button_b: (bx, by),
                prize: (px, py),
            },
        ))
    }

    pub fn button_a(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, _) = tag("Button A: X+")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, y) = i32(input)?;
        let (input, _) = line_ending(input)?;
        Ok((input, (x, y)))
    }

    pub fn button_b(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, _) = tag("Button B: X+")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, y) = i32(input)?;
        let (input, _) = line_ending(input)?;
        Ok((input, (x, y)))
    }

    pub fn prize(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, _) = tag("Prize: X=")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(", Y=")(input)?;
        let (input, y) = i32(input)?;
        Ok((input, (x, y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("480", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let machines = parser::parse(SAMPLE);
        assert_eq!(4, machines.len());
        assert_eq!(22, machines[0].button_b.0);
        assert_eq!(66, machines[1].button_a.1);
        assert_eq!(7870, machines[2].prize.0);
        Ok(())
    }

    #[test]
    fn test_solve() -> miette::Result<()> {
        let machines = parser::parse(SAMPLE);
        assert_eq!(Some(280), machines[0].solve(0));
        assert_eq!(None, machines[1].solve(0));
        assert_eq!(Some(200), machines[2].solve(0));
        assert_eq!(None, machines[3].solve(0));
        Ok(())
    }
}
