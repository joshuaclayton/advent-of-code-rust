use std::collections::HashMap;

use crate::parser::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space0, space1},
    combinator::{map, opt, value},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

pub fn solve() {
    let input = include_str!("input-day5");
    println!("Answer: {:?}", run(input));
}

#[derive(Debug)]
struct Crates<'a>(HashMap<usize, Vec<&'a str>>);

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

impl<'a> Crates<'a> {
    fn apply_instruction(&mut self, move_instruction: &MoveInstruction) {
        for _ in times(move_instruction.count) {
            let mut popped = None;
            self.0
                .entry(move_instruction.from)
                .and_modify(|v| popped = Some(v.remove(0)));
            self.0
                .entry(move_instruction.to)
                .and_modify(|v| v.insert(0, popped.unwrap()));
        }
    }

    fn top_crates(&self) -> String {
        self.0
            .iter()
            .sorted_by_key(|v| v.0)
            .map(|(_, v)| v.get(0))
            .filter_map(|v| v.map(|x| x.clone()))
            .collect::<String>()
    }
}

fn run(input: &str) -> Option<String> {
    let (input, mut crates) =
        terminated(preceded(opt(tag("\n")), parse_crates), space1)(input).ok()?;
    let (input, move_instructions) = preceded(
        many1(tag("\n")),
        separated_list1(tag("\n"), parse_move_instruction),
    )(input)
    .ok()?;

    for i in move_instructions {
        crates.apply_instruction(&i);
    }

    Some(crates.top_crates())
}

fn parse_crate_line(input: &str) -> IResult<&str, HashMap<usize, &str>> {
    map(separated_list1(tag(" "), parse_crate), |list| {
        let mut result = HashMap::new();

        for (index, crate_) in list.iter().enumerate() {
            if let Some(crate_) = crate_ {
                result.insert(index, *crate_);
            }
        }

        result
    })(input)
}

#[derive(Debug, Clone)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move_instruction(input: &str) -> IResult<&str, MoveInstruction> {
    let (input, count) = preceded(tag("move "), parse_usize)(input)?;
    let (input, from) = preceded(tag(" from "), parse_usize)(input)?;
    let (input, to) = preceded(tag(" to "), parse_usize)(input)?;

    Ok((input, MoveInstruction { count, from, to }))
}

fn parse_crates(input: &str) -> IResult<&str, Crates> {
    let (input, crates) = parse_temporary_crates(input)?;
    let (input, stacks) = preceded(tag("\n"), parse_stacks)(input)?;

    let mut result: HashMap<usize, Vec<&str>> = HashMap::new();

    for crate_ in crates {
        for (idx, column) in stacks.iter().enumerate() {
            if let Some(val) = crate_.get(&idx) {
                result
                    .entry(*column)
                    .and_modify(|v| v.push(*val))
                    .or_insert(vec![*val]);
            }
        }
    }

    Ok((input, Crates(result)))
}

fn parse_temporary_crates(input: &str) -> IResult<&str, Vec<HashMap<usize, &str>>> {
    separated_list1(tag("\n"), parse_crate_line)(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space0, preceded(space0, parse_usize))(input)
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    alt((
        value(None, tag("   ")),
        map(preceded(tag("["), terminated(alpha1, tag("]"))), Some),
    ))(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "#;
        assert_eq!(super::run(input), Some("CMZ".to_string()))
    }

    #[test]
    fn parse_empty_crate() {
        let map: std::collections::HashMap<usize, _> =
            super::parse_crate_line("    [D]    ").unwrap().1;
        assert_eq!(map.get(&1), Some(&"D"));
        assert_eq!(map.get(&0), None);
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn parse_stack_list() {
        assert_eq!(super::parse_stacks(" 1   2   3 ").unwrap().1, vec![1, 2, 3]);
    }

    #[test]
    fn parse_move_instruction_test() {
        let result = super::parse_move_instruction("move 1 from 2 to 1")
            .unwrap()
            .1;
        assert_eq!(result.count, 1);
        assert_eq!(result.from, 2);
        assert_eq!(result.to, 1);
    }
}
