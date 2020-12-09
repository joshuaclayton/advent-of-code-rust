use crate::parser::*;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::preceded,
    IResult,
};

struct Computer<'a> {
    acc: isize,
    instructions: &'a [Instruction],
    current_position: usize,
    previous_positions: Vec<usize>,
}

impl<'a> Computer<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            acc: 0,
            instructions,
            current_position: 0,
            previous_positions: vec![],
        }
    }
}

impl<'a> Iterator for Computer<'a> {
    type Item = Result<isize, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.previous_positions.contains(&self.current_position) {
            None
        } else if self.current_position >= self.instructions.len() {
            None
        } else {
            let at_last = self.current_position + 1 == self.instructions.len();
            self.previous_positions.push(self.current_position);
            match self.instructions[self.current_position] {
                Instruction::NoOp(_) => {
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

            if at_last {
                Some(Ok(self.acc))
            } else {
                Some(Err("not done"))
            }
        }
    }
}

#[derive(Clone)]
enum Instruction {
    IncrementAccumulator(isize),
    JumpToRelative(isize),
    NoOp(isize),
}

impl Instruction {
    fn swap(&mut self) -> &mut Self {
        match self {
            Instruction::IncrementAccumulator(v) => {
                *self = Instruction::IncrementAccumulator(*v);
            }
            Instruction::JumpToRelative(v) => {
                *self = Instruction::NoOp(*v);
            }
            Instruction::NoOp(v) => {
                *self = Instruction::JumpToRelative(*v);
            }
        }

        self
    }
}

fn parse_signed_isize(input: &str) -> IResult<&str, isize> {
    let (input, multiplier) = alt((map(tag("-"), |_| -1), map(tag("+"), |_| 1)))(input)?;
    let (input, result) = map(parse_usize, |v| multiplier * (v as isize))(input)?;

    Ok((input, result))
}

fn parse_nop(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("nop "), parse_signed_isize), |v| {
        Instruction::NoOp(v)
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

    let result = instructions
        .iter()
        .enumerate()
        .filter_map(|(i, _)| {
            let mut instructions = instructions.clone();
            instructions[i].swap();

            match Computer::new(&instructions).last() {
                Some(Ok(v)) => Some(v),
                _ => None,
            }
        })
        .next();

    println!("Solution: {:?}", result);
}
