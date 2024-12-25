use crate::custom_error::AocError;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Operation {
    AND,
    XOR,
    OR,
}

#[derive(Debug)]
pub struct Gate {
    operation: Operation,
    input1: String,
    input2: String,
    output: String,
    value: Option<bool>,
}

impl Gate {
    pub fn value(&self, inputs: &Inputs) -> Option<bool> {
        if let Some(b) = self.value {
            return Some(b);
        }
        let i1 = inputs.get(&self.input1)?;
        let i2 = inputs.get(&self.input2)?;
        match self.operation {
            Operation::AND => Some(*i1 & *i2),
            Operation::OR => Some(*i1 | *i2),
            Operation::XOR => Some(*i1 ^ *i2),
        }
    }

    pub fn update(&mut self, inputs: &mut Inputs, val: bool) {
        self.value = Some(val);
        inputs.insert(self.output.clone(), val);
    }

    pub fn can_move(&self, inputs: &Inputs) -> bool {
        //dbg!("Considering {:?}", &self);
        self.value.is_none() & inputs.contains_key(&self.input1) & inputs.contains_key(&self.input2)
    }
}

pub fn step(inputs: &mut Inputs, gates: &mut [Gate]) -> bool {
    if let Some(index) = gates.iter().position(|e| e.can_move(inputs)) {
        let v = gates[index].value(inputs);
        gates[index].update(inputs, v.unwrap());
        return true;
    }
    false
}
type Inputs = HashMap<String, bool>;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (mut gates, mut inputs) = parser::parse(input);
    while step(&mut inputs, &mut gates) {}
    let mut zs = inputs
        .iter()
        .filter(|(k, _v)| k.starts_with('z'))
        .map(|(k, &v)| (k[1..].parse::<u32>().unwrap(), v))
        .collect::<Vec<_>>();
    zs.sort_by(|a, b| b.0.cmp(&a.0));
    let res = zs.iter().fold(0, |t, e| (t * 2) + e.1 as usize);

    Ok(res.to_string())
}

mod parser {
    use super::*;
    use nom::{
        bytes::complete::{tag, take},
        character::complete::{i32, line_ending, one_of},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{separated_pair, terminated},
        IResult,
    };
    pub fn parse(input: &str) -> (Vec<Gate>, Inputs) {
        let (input, inputs) = parse_inputs(input).unwrap();
        let (_, gates) = separated_list1(line_ending, gate)(input).unwrap();
        (gates, inputs.into_iter().collect::<Inputs>())
    }

    pub fn parse_inputs(input: &str) -> IResult<&str, Inputs> {
        terminated(
            map(
                many1(separated_pair(
                    map(take(3_usize), |s: &str| s.to_string()),
                    tag(": "),
                    terminated(map(i32, |i| i == 1), line_ending),
                )),
                |v| v.into_iter().collect::<Inputs>(),
            ),
            line_ending,
        )(input)
    }
    pub fn gate(input: &str) -> IResult<&str, Gate> {
        let (input, input1) =
            map(terminated(take(3_usize), tag(" ")), |s: &str| s.to_string())(input)?;
        let (input, operation) = terminated(
            map(many1(one_of("ANDXOR")), |v| v.iter().collect::<String>()),
            tag(" "),
        )(input)?;
        let (input, input2) = map(terminated(take(3_usize), tag(" -> ")), |s: &str| {
            s.to_string()
        })(input)?;
        let (input, output) = map(take(3_usize), |s: &str| s.to_string())(input)?;
        let operation = match &operation[..] {
            "AND" => Operation::AND,
            "XOR" => Operation::XOR,
            "OR" => Operation::OR,
            _ => panic!("Bad operation"),
        };
        let gate = Gate {
            input1,
            input2,
            output,
            operation,
            value: None,
        };
        Ok((input, gate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const SAMPLE_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("2024", process(SAMPLE_2)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let (gates, inputs) = parser::parse(SAMPLE_1);
        assert_eq!(6, inputs.len());
        assert_eq!(3, gates.len());
        Ok(())
    }

    #[test]
    fn test_value() -> miette::Result<()> {
        let (gates, inputs) = parser::parse(SAMPLE_1);
        assert_eq!(Some(false), gates[0].value(&inputs));
        assert_eq!(Some(false), gates[1].value(&inputs));
        assert_eq!(Some(true), gates[2].value(&inputs));
        Ok(())
    }

    #[test]
    fn test_step() -> miette::Result<()> {
        let (mut gates, mut inputs) = parser::parse(SAMPLE_2);
        dbg!(&gates[1]);
        assert_eq!(None, gates[1].value);
        assert!(step(&mut inputs, &mut gates));
        assert_eq!(Some(true), gates[1].value);

        Ok(())
    }
}
