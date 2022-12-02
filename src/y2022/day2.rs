use crate::parser::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn solve() {
    let input = include_str!("input-day2");
    println!("Answer: {:?}", run(input));
}

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum RoundOutcome {
    Win,
    Loss,
    Draw,
}

impl RoundOutcome {
    fn value(&self) -> usize {
        match self {
            RoundOutcome::Loss => 0,
            RoundOutcome::Draw => 3,
            RoundOutcome::Win => 6,
        }
    }
}

impl Shape {
    fn round_outcome(&self, opponent: &Shape) -> RoundOutcome {
        match (self, opponent) {
            (a, b) if a == b => RoundOutcome::Draw,
            (Shape::Paper, Shape::Rock) => RoundOutcome::Win,
            (Shape::Rock, Shape::Scissors) => RoundOutcome::Win,
            (Shape::Scissors, Shape::Paper) => RoundOutcome::Win,
            _ => RoundOutcome::Loss,
        }
    }

    fn value(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn round_score(you: &Shape, opponent: &Shape) -> usize {
    you.round_outcome(opponent).value() + you.value()
}

fn run(input: &str) -> Option<usize> {
    let (_, scores) =
        all_consuming(separated_list1(tag("\n"), parse_round_score))(input.trim()).ok()?;
    scores.iter().sum::<usize>().into()
}

fn parse_round_score(input: &str) -> IResult<&str, usize> {
    map(
        separated_pair(parse_opponent_shape, tag(" "), parse_your_shape),
        |(opponent, you)| round_score(&you, &opponent),
    )(input)
}

fn parse_opponent_shape(input: &str) -> IResult<&str, Shape> {
    alt((
        value(Shape::Rock, tag("A")),
        value(Shape::Paper, tag("B")),
        value(Shape::Scissors, tag("C")),
    ))(input)
}

fn parse_your_shape(input: &str) -> IResult<&str, Shape> {
    alt((
        value(Shape::Rock, tag("X")),
        value(Shape::Paper, tag("Y")),
        value(Shape::Scissors, tag("Z")),
    ))(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
A Y
B X
C Z
        "#;
        assert_eq!(super::run(input), Some(15))
    }
}
