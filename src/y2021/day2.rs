use crate::parser::*;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::preceded,
    IResult,
};

enum Operation {
    Forward(usize),
    Down(usize),
    Up(usize),
}

struct Position(usize, usize);

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

impl Position {
    fn change_direction(&self, operation: &Operation) -> Self {
        match operation {
            Operation::Up(v) => Position(self.0 - v, self.1),

            Operation::Down(v) => Position(self.0 + v, self.1),

            Operation::Forward(v) => Position(self.0, self.1 + v),
        }
    }
}

pub fn solve() {
    let input = include_str!("input-day2");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<usize> {
    let (_, operations) = parse_operations(input).ok()?;

    let final_position = operations
        .iter()
        .fold(Position::default(), |position, operation| {
            position.change_direction(operation)
        });

    Some(final_position.0 * final_position.1)
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(tag("\n"), parse_operation)(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((parse_forward, parse_down, parse_up))(input)
}

fn parse_forward(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag("forward "), parse_usize), Operation::Forward)(input)
}

fn parse_down(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag("down "), parse_usize), Operation::Down)(input)
}

fn parse_up(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag("up "), parse_usize), Operation::Up)(input)
}

mod tests {
    #[test]
    fn base_case() {
        let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
        "#;

        assert_eq!(super::run(input.trim()), Some(150));
    }
}
