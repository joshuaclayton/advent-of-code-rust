use crate::parser::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub fn solve() {
    let input = include_str!("input-day10");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<isize> {
    let (_, instructions) =
        all_consuming(separated_list1(tag("\n"), parse_instruction))(input.trim()).ok()?;
    let cpu = Cpu::process(instructions);

    for (index, state) in cpu.cycles.iter().enumerate() {
        let idx = (index as isize) % 40;
        let range = (state.x - 1)..=(state.x + 1);

        if idx == 0 {
            print!("\n");
        }

        if range.contains(&idx) {
            print!("#");
        } else {
            print!(".");
        }
    }

    None
}

#[derive(PartialEq, Clone, Debug, Copy)]
enum Instruction {
    NoOp,
    AddX(isize),
}

#[derive(Debug)]
struct State {
    cycle: usize,
    x: isize,
}

#[derive(Debug)]
struct Cpu {
    cycles: Vec<State>,
}

impl Cpu {
    fn process(instructions: Vec<Instruction>) -> Self {
        let mut cpu = Cpu { cycles: vec![] };

        let mut instructions_with_steps = vec![];

        for instruction in instructions {
            if instruction != Instruction::NoOp {
                instructions_with_steps.push(None);
            }

            instructions_with_steps.push(Some(instruction));
        }

        let mut cycle = 1;
        let mut during_x = 1;

        for instruction in instructions_with_steps {
            let x = during_x;
            cpu.cycles.push(State { cycle, x });

            match instruction {
                Some(Instruction::AddX(x)) => {
                    during_x += x;
                }
                Some(Instruction::NoOp) | None => (),
            }

            cycle += 1;
        }

        cpu
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::NoOp, tag("noop")),
        map(preceded(tag("addx "), parse_digits), Instruction::AddX),
    ))(input)
}

mod tests {
    use super::*;

    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
        "#;
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            super::parse_instruction("addx -11").unwrap().1,
            Instruction::AddX(-11)
        );
    }
}
