use crate::custom_error::AocError;
use std::fmt::Display;
use std::ops::BitXor;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut device = parser::parse(input);
    while !device.step() {
        if let Some(o) = device.last_out.take() {
            print!("{},", o);
        }
    }
    println!("\n");
    Ok("".to_string())
}
pub fn process_limit(input: &str, start: i64, end: i64) -> miette::Result<String, AocError> {
    let mut device = parser::parse(input);
    for i in start..end {
        if i % 10000000 == 0 {
            println!("Trying {}", i);
        }
        if device.is_quine(i) {
            return Ok(i.to_string());
        }
    }
    Ok("Not found".to_string())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl From<i64> for Instruction {
    fn from(value: i64) -> Self {
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
    saved_register_a: i64,
    saved_register_b: i64,
    saved_register_c: i64,
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<Instruction>,
    pointer: usize,
    out: Vec<i64>,
    last_out: Option<i64>,
}

impl Device {
    pub fn find(&mut self, output: Instruction, reg_a: i64) -> Vec<i64> {
        let mut result = vec![];
        println!("looking for {:?}", output);
        for i in reg_a..1000000000 {
            self.reset();
            self.register_a = i;
            while self.program[self.pointer] != Instruction::Jnz {
                self.step();
            }
            if (self.register_a == reg_a) && self.last_out.map(|e| e.into()) == Some(output) {
                result.push(i);
            }
        }
        result
    }
    pub fn is_quine(&mut self, a: i64) -> bool {
        let desired = self.program.clone();
        let mut desired = desired.iter().rev().collect::<Vec<_>>();
        self.reset();
        self.register_a = a;
        while !self.step() {
            if let Some(o) = self.last_out {
                if Some(&o.into()) != desired.pop() {
                    return false;
                }
                self.last_out = None;
            }
        }
        desired.is_empty()
    }
    pub fn reset(&mut self) {
        self.register_a = self.saved_register_a;
        self.register_b = self.saved_register_b;
        self.register_c = self.saved_register_c;
        self.out = vec![];
        self.pointer = 0;
    }
    pub fn run(&mut self) {
        while !self.step() {}
    }

    pub fn step(&mut self) -> bool {
        if self.pointer >= self.program.len() {
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
        self.last_out = out;
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

    pub fn literal(&self, _device: &Device) -> i64 {
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

    pub fn execute(&self, device: &Device) -> (i64, i64, i64, usize, Option<i64>, bool) {
        let op = device.program[device.pointer + 1];
        let mut register_a = device.register_a;
        let mut register_b = device.register_b;
        let mut register_c = device.register_c;
        let mut pointer = device.pointer + 2;
        let mut out = None;
        match self {
            Instruction::Adv => {
                register_a /= 2_i64.pow(op.combo(device));
            }
            Instruction::Bxl => {
                register_b = register_b.bitxor(op.literal(device));
            }
            Instruction::Bst => {
                register_b = op.combo(device) as i64 % 8;
            }
            Instruction::Jnz => {
                if register_a > 0 {
                    pointer = op.literal(device) as usize;
                }
            }
            Instruction::Bxc => {
                register_b = register_b.bitxor(register_c);
            }
            Instruction::Out => out = Some(op.combo(device) as i64 % 8),
            Instruction::Bdv => {
                register_b = dbg!(register_a / (2_i64.pow(op.combo(device))));
            }
            Instruction::Cdv => {
                register_c = register_a / (2_i64.pow(op.combo(device)));
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
        character::complete::{i64, newline},
        multi::separated_list1,
        sequence::{preceded, terminated},
        IResult,
    };

    pub fn parse(input: &str) -> Device {
        let (_input, device) = parse_device(input).unwrap();
        device
    }
    pub fn parse_device(input: &str) -> IResult<&str, Device> {
        let (input, register_a) = terminated(preceded(tag("Register A: "), i64), newline)(input)?;
        let (input, register_b) = terminated(preceded(tag("Register B: "), i64), newline)(input)?;
        let (input, register_c) = terminated(preceded(tag("Register C: "), i64), newline)(input)?;
        let (input, _) = newline(input)?;
        let (input, program) = preceded(tag("Program: "), separated_list1(tag(","), i64))(input)?;
        let program = program.iter().map(|&e| e.into()).collect::<Vec<_>>();
        let device = Device {
            register_a,
            register_b,
            register_c,
            saved_register_a: register_a,
            saved_register_b: register_b,
            saved_register_c: register_c,
            program,
            ..Default::default()
        };
        Ok((input, device))
    }
}

pub fn reverse(a: i64) -> (i64, i64) {
    let b = (a % 8).bitxor(5);
    let c = a / (2_i64.pow(b as u32));
    (b.bitxor(6).bitxor(c) % 8, a / 8)
}
