use crate::custom_error::AocError;
use std::fmt::Display;
use std::ops::BitXor;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut device = parser::parse(input);
    while !device.step() {}
    println!("{}", device);
    Ok("".to_string())
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instruction::Adv => 0,
                Instruction::Bxl => 1,
                Instruction::Bst => 2,
                Instruction::Jnz => 3,
                Instruction::Bxc => 4,
                Instruction::Out => 5,
                Instruction::Bdv => 6,
                Instruction::Cdv => 7,
            }
        )
    }
}

impl From<i32> for Instruction {
    fn from(value: i32) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Bad instruction {}", value),
        }
    }
}

#[derive(Default)]
pub struct Device {
    register_a: i32,
    register_b: i32,
    register_c: i32,
    program: Vec<Instruction>,
    pointer: usize,
    out: Vec<i32>,
}

impl Device {
    pub fn reset(&mut self) {
        self.out = vec![];
        self.pointer = 0;
    }
    pub fn run(&mut self) {
        while !self.step() {}
    }

    pub fn step(&mut self) -> bool {
        if self.pointer > self.program.len() {
            return true;
        }
        let (a, b, c, p, out, halt) = self.program[self.pointer].execute(self);
        self.register_a = a;
        self.register_b = b;
        self.register_c = c;
        self.pointer = p;
        if let Some(out) = out {
            self.out.push(out);
        }
        halt
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Register A: {}", self.register_a)?;
        writeln!(f, "Register B: {}", self.register_b)?;
        writeln!(f, "Register C: {}", self.register_c)?;
        writeln!(f)?;
        write!(f, "Program ({}): ", self.pointer)?;
        for i in 0..self.program.len() {
            if i == self.pointer {
                write!(f, " ")?;
            }
            write!(f, "{},", self.program[i])?;
            if i == self.pointer {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        write!(f, "Output: ")?;
        if !self.out.is_empty() {
            write!(f, "{}", self.out[0])?;
            for i in 1..self.out.len() {
                write!(f, ",{}", self.out[i])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Instruction {
    pub fn combo(&self, device: &Device) -> u32 {
        match self {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => device.register_a as u32,
            Instruction::Out => device.register_b as u32,
            Instruction::Bdv => device.register_c as u32,
            Instruction::Cdv => panic!("Bad combo"),
        }
    }

    pub fn literal(&self, _device: &Device) -> i32 {
        match self {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => 4,
            Instruction::Out => 5,
            Instruction::Bdv => 6,
            Instruction::Cdv => 7,
        }
    }

    pub fn execute(&self, device: &Device) -> (i32, i32, i32, usize, Option<i32>, bool) {
        let op = device.program[device.pointer + 1];
        let mut register_a = device.register_a;
        let mut register_b = device.register_b;
        let mut register_c = device.register_c;
        let mut pointer = device.pointer + 2;
        let mut out = None;
        match self {
            Instruction::Adv => {
                register_a = register_a / (2_i32.pow(op.combo(device)));
            }
            Instruction::Bxl => {
                register_b = register_b.bitxor(op.literal(device));
            }
            Instruction::Bst => {
                register_b = op.combo(device) as i32 % 8;
            }
            Instruction::Jnz => {
                if register_a > 0 {
                    pointer = op.literal(device) as usize;
                }
            }
            Instruction::Bxc => {
                register_b = register_b.bitxor(register_c);
            }
            Instruction::Out => out = Some(op.combo(device) as i32 % 8),
            Instruction::Bdv => {
                register_b = dbg!(register_a / (2_i32.pow(op.combo(device))));
            }
            Instruction::Cdv => {
                register_c = register_a / (2_i32.pow(op.combo(device)));
            }
        }
        (
            register_a,
            register_b,
            register_c,
            pointer,
            out,
            pointer >= device.program.len(),
        )
    }
}

pub mod parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{i32, newline},
        multi::separated_list1,
        sequence::{preceded, terminated},
        IResult,
    };

    pub fn parse(input: &str) -> Device {
        let (_input, device) = parse_device(input).unwrap();
        device
    }
    pub fn parse_device(input: &str) -> IResult<&str, Device> {
        let (input, register_a) = terminated(preceded(tag("Register A: "), i32), newline)(input)?;
        let (input, register_b) = terminated(preceded(tag("Register B: "), i32), newline)(input)?;
        let (input, register_c) = terminated(preceded(tag("Register C: "), i32), newline)(input)?;
        let (input, _) = newline(input)?;
        let (input, program) = preceded(tag("Program: "), separated_list1(tag(","), i32))(input)?;
        let program = program.iter().map(|&e| e.into()).collect::<Vec<_>>();
        let device = Device {
            register_a,
            register_b,
            register_c,
            program,
            pointer: 0,
            out: vec![],
        };
        Ok((input, device))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    /*
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("", process("")?);
        Ok(())
    }
    */

    #[test]
    fn test_parse() -> miette::Result<()> {
        let mut device = parser::parse(SAMPLE_1);
        while !device.step() {
            println!("{}", device);
        }
        assert_eq!(device.out, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
        Ok(())
    }

    #[test]
    fn test_adv() -> miette::Result<()> {
        let mut device = Device {
            register_a: 8,
            program: vec![Instruction::Adv, 0.into()],
            ..Default::default()
        };
        device.step();
        assert_eq!(8, device.register_a);
        device.reset();
        device.program[1] = 1.into();
        device.step();
        assert_eq!(4, device.register_a);
        device.reset();
        device.program[1] = 2.into();
        device.register_a = 9;
        device.step();
        assert_eq!(2, device.register_a);
        Ok(())
    }

    #[test]
    fn test_bxl() -> miette::Result<()> {
        let mut device = Device {
            register_b: 3,
            program: vec![Instruction::Bxl, 0.into()],
            ..Default::default()
        };
        device.step();
        assert_eq!(3, device.register_b);
        device.reset();
        device.register_b = 3;
        device.program[1] = 3.into();
        device.step();
        assert_eq!(0, device.register_b);
        device.reset();
        device.register_b = 3;
        device.program[1] = 1.into();
        device.step();
        assert_eq!(2, device.register_b);
        Ok(())
    }

    #[test]
    fn test_combo() -> miette::Result<()> {
        let device = Device {
            register_a: 18,
            register_b: 19,
            register_c: 20,
            ..Default::default()
        };

        assert_eq!(0, Instruction::Adv.combo(&device));
        assert_eq!(1, Instruction::Bxl.combo(&device));
        assert_eq!(2, Instruction::Bst.combo(&device));
        assert_eq!(3, Instruction::Jnz.combo(&device));
        assert_eq!(18, Instruction::Bxc.combo(&device));
        assert_eq!(19, Instruction::Out.combo(&device));
        assert_eq!(20, Instruction::Bdv.combo(&device));
        Ok(())
    }

    #[test]
    fn test_bst() -> miette::Result<()> {
        let mut device = Device {
            register_a: 18,
            program: vec![Instruction::Bst, 0.into()],
            ..Default::default()
        };
        device.step();
        assert_eq!(0, device.register_b);
        device.reset();
        device.program[1] = 4.into();
        device.step();
        assert_eq!(2, device.register_b);
        Ok(())
    }

    #[test]
    fn test_jnz() -> miette::Result<()> {
        let mut device = Device {
            register_a: 0,
            program: vec![
                Instruction::Jnz,
                4.into(),
                5.into(),
                1.into(),
                5.into(),
                3.into(),
            ],
            ..Default::default()
        };
        assert_eq!(0, device.pointer);
        device.step();
        assert_eq!(2, device.pointer);
        device.reset();
        device.register_a = 1;
        device.step();
        assert_eq!(4, device.pointer);
        Ok(())
    }

    #[test]
    fn test_bxc() -> miette::Result<()> {
        let mut device = Device {
            register_b: 4,
            register_c: 1,
            program: vec![Instruction::Bxc, 0.into()],
            ..Default::default()
        };
        assert_eq!(4, device.register_b);
        device.step();
        assert_eq!(5, device.register_b);
        assert_eq!(2, device.pointer);
        Ok(())
    }

    #[test]
    fn test_out() -> miette::Result<()> {
        let mut device = Device {
            register_a: 18,
            program: vec![Instruction::Out, 4.into()],
            ..Default::default()
        };
        assert!(device.out.is_empty());
        device.step();
        assert_eq!(vec![2], device.out);
        assert_eq!(2, device.pointer);
        Ok(())
    }

    #[test]
    fn test_bdv() -> miette::Result<()> {
        let mut device = Device {
            register_a: 8,
            program: vec![Instruction::Bdv, 0.into()],
            ..Default::default()
        };
        device.step();
        assert_eq!(8, device.register_a);
        assert_eq!(8, device.register_b);
        device.reset();
        device.program[1] = 1.into();
        device.step();
        assert_eq!(8, device.register_a);
        assert_eq!(4, device.register_b);
        device.reset();
        device.program[1] = 2.into();
        device.register_a = 9;
        device.step();
        assert_eq!(9, device.register_a);
        assert_eq!(2, device.register_b);
        Ok(())
    }
    #[test]
    fn test_cdv() -> miette::Result<()> {
        let mut device = Device {
            register_a: 8,
            program: vec![Instruction::Cdv, 0.into()],
            ..Default::default()
        };
        device.step();
        assert_eq!(8, device.register_a);
        assert_eq!(8, device.register_c);
        device.reset();
        device.program[1] = 1.into();
        device.step();
        assert_eq!(8, device.register_a);
        assert_eq!(4, device.register_c);
        device.reset();
        device.program[1] = 2.into();
        device.register_a = 9;
        device.step();
        assert_eq!(9, device.register_a);
        assert_eq!(2, device.register_c);
        Ok(())
    }

    #[test]
    fn test_one() -> miette::Result<()> {
        let mut device = Device {
            register_c: 9,
            program: vec![2.into(), 6.into()],
            ..Default::default()
        };
        device.run();
        assert_eq!(device.register_b, 1);
        Ok(())
    }
    #[test]
    fn test_two() -> miette::Result<()> {
        let mut device = Device {
            register_a: 10,
            program: vec![5, 0, 5, 1, 5, 4]
                .iter()
                .map(|&e| e.into())
                .collect::<Vec<_>>(),
            ..Default::default()
        };
        device.run();
        assert_eq!(device.out, vec![0, 1, 2]);
        Ok(())
    }
}
