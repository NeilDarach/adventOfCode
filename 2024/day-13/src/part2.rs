use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let machines = parser::parse(input);
    let result: i128 = machines
        .into_iter()
        .filter_map(|mut e| e.solve(10000000000000))
        .sum();
    Ok(result.to_string())
}
pub struct Machine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    cost_a: i128,
    cost_b: i128,
    prize: (i128, i128),
}

impl Machine {
    pub fn solve(&mut self, offset: i128) -> Option<i128> {
        let prize = (self.prize.0 + offset, self.prize.1 + offset);
        let num = (self.button_a.0 * prize.1) - (self.button_a.1 * prize.0);
        let den = (self.button_a.0 * self.button_b.1) - (self.button_a.1 * self.button_b.0);
        if num % den != 0 {
            return None;
        }

        let j = num / den;
        let i = (prize.0 - (self.button_b.0 * j)) / self.button_a.0;

        Some((self.cost_b * j) + (self.cost_a * i))
    }
}

mod parser {
    use nom::{
        bytes::complete::tag, character::complete::i128, character::complete::line_ending,
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
                cost_a: 3,
                cost_b: 1,
            },
        ))
    }

    pub fn button_a(input: &str) -> IResult<&str, (i128, i128)> {
        let (input, _) = tag("Button A: X+")(input)?;
        let (input, x) = i128(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, y) = i128(input)?;
        let (input, _) = line_ending(input)?;
        Ok((input, (x, y)))
    }

    pub fn button_b(input: &str) -> IResult<&str, (i128, i128)> {
        let (input, _) = tag("Button B: X+")(input)?;
        let (input, x) = i128(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, y) = i128(input)?;
        let (input, _) = line_ending(input)?;
        Ok((input, (x, y)))
    }

    pub fn prize(input: &str) -> IResult<&str, (i128, i128)> {
        let (input, _) = tag("Prize: X=")(input)?;
        let (input, x) = i128(input)?;
        let (input, _) = tag(", Y=")(input)?;
        let (input, y) = i128(input)?;
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

    const SAMPLE_2: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279
";

    #[test]
    fn test_process() -> miette::Result<()> {
        let file = include_str!("../input1.txt");
        let machines = parser::parse(file);
        let result: i128 = machines.into_iter().filter_map(|mut e| e.solve(0)).sum();
        assert_eq!(31623, result);
        Ok(())
    }

    #[test]
    fn test_solve() -> miette::Result<()> {
        let mut machines = parser::parse(SAMPLE);

        assert_eq!(Some(280), machines[0].solve(0));
        assert_eq!(None, machines[1].solve(0));
        assert_eq!(Some(200), machines[2].solve(0));
        assert_eq!(None, machines[3].solve(0));
        assert_eq!(Some(459236326669), machines[1].solve(10000000000000));

        let mut machines = parser::parse(SAMPLE_2);
        assert_eq!(None, machines[0].solve(0));
        assert_eq!(Some(459236326669), machines[1].solve(0));
        assert_eq!(None, machines[2].solve(0));
        assert_eq!(Some(416082282239), machines[3].solve(0));
        Ok(())
    }
}
