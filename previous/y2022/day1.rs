use crate::parser::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    IResult,
};

pub fn solve() {
    let input = include_str!("input-day1");
    println!("Answer: {:?}", run(input));
}

#[derive(Debug, PartialEq)]
struct Food {
    calories: usize,
}

impl Food {
    fn new(calories: usize) -> Self {
        Food { calories }
    }
}

#[derive(Debug, PartialEq)]
struct Elf {
    food: Vec<Food>,
}

impl Elf {
    fn new(food: Vec<Food>) -> Self {
        Elf { food }
    }

    fn calories_carried(&self) -> usize {
        self.food.iter().map(|f| f.calories).sum()
    }
}

fn run(input: &str) -> Option<usize> {
    let (_, elves) = all_consuming(separated_list1(tag("\n\n"), parse_elf))(input.trim()).ok()?;

    elves
        .iter()
        .map(|e| e.calories_carried())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum::<usize>()
        .into()
}

fn parse_food(input: &str) -> IResult<&str, Food> {
    map(parse_usize, Food::new)(input)
}

fn parse_elf(input: &str) -> IResult<&str, Elf> {
    map(separated_list1(tag("\n"), parse_food), Elf::new)(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "#;
        assert_eq!(super::run(input), Some(45000))
    }
}
