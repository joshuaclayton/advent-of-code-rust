use crate::parser::*;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::preceded,
    IResult,
};

struct Computer {
    acc: isize,
    instructions: Vec<Instruction>,
    current_position: usize,
    previous_positions: Vec<usize>,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            instructions,
            current_position: 0,
            previous_positions: vec![],
        }
    }
}

impl Iterator for Computer {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.previous_positions.contains(&self.current_position) {
            None
        } else {
            self.previous_positions.push(self.current_position);
            match self.instructions[self.current_position] {
                Instruction::NoOp => {
                    self.current_position += 1;
                }
                Instruction::JumpToRelative(v) => {
                    if v < 0 {
                        self.current_position = self.current_position - v.abs() as usize;
                    } else {
                        self.current_position = self.current_position + v as usize;
                    }
                }
                Instruction::IncrementAccumulator(v) => {
                    self.current_position += 1;
                    self.acc += v;
                }
            }

            Some(self.acc)
        }
    }
}

enum Instruction {
    IncrementAccumulator(isize),
    JumpToRelative(isize),
    NoOp,
}

fn parse_signed_isize(input: &str) -> IResult<&str, isize> {
    let (input, multiplier) = alt((map(tag("-"), |_| -1), map(tag("+"), |_| 1)))(input)?;
    let (input, result) = map(parse_usize, |v| multiplier * (v as isize))(input)?;

    Ok((input, result))
}

fn parse_nop(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("nop "), parse_signed_isize), |_| {
        Instruction::NoOp
    })(input)
}

fn parse_jmp(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("jmp "), parse_signed_isize), |v| {
        Instruction::JumpToRelative(v)
    })(input)
}

fn parse_acc(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("acc "), parse_signed_isize), |v| {
        Instruction::IncrementAccumulator(v)
    })(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_nop, parse_jmp, parse_acc))(input)
}

pub fn solve() {
    let input = include_str!("input-day8");
    let (_, instructions) = separated_list1(tag("\n"), parse_instruction)(input).unwrap();

    println!("Solution: {:?}", Computer::new(instructions).last());
}
