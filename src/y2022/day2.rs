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

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
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

    fn shape_outcome(&self, opponent: &Shape) -> Shape {
        match (self, opponent) {
            (RoundOutcome::Win, Shape::Paper) => Shape::Scissors,
            (RoundOutcome::Win, Shape::Rock) => Shape::Paper,
            (RoundOutcome::Win, Shape::Scissors) => Shape::Rock,
            (RoundOutcome::Loss, Shape::Paper) => Shape::Rock,
            (RoundOutcome::Loss, Shape::Rock) => Shape::Scissors,
            (RoundOutcome::Loss, Shape::Scissors) => Shape::Paper,
            (RoundOutcome::Draw, &shape) => shape,
        }
    }
}

impl Shape {
    fn value(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn round_score(you: &RoundOutcome, opponent: &Shape) -> usize {
    you.shape_outcome(opponent).value() + you.value()
}

fn run(input: &str) -> Option<usize> {
    let (_, scores) =
        all_consuming(separated_list1(tag("\n"), parse_round_score))(input.trim()).ok()?;
    scores.iter().sum::<usize>().into()
}

fn parse_round_score(input: &str) -> IResult<&str, usize> {
    map(
        separated_pair(parse_opponent_shape, tag(" "), parse_your_outcome),
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

fn parse_your_outcome(input: &str) -> IResult<&str, RoundOutcome> {
    alt((
        value(RoundOutcome::Loss, tag("X")),
        value(RoundOutcome::Draw, tag("Y")),
        value(RoundOutcome::Win, tag("Z")),
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
        assert_eq!(super::run(input), Some(12))
    }
}
