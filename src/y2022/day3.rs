use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{all_consuming, map},
    multi::separated_list1,
    IResult,
};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve() {
    let input = include_str!("input-day3");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<usize> {
    let (_, rucksacks) =
        all_consuming(separated_list1(tag("\n"), parse_rucksack))(input.trim()).ok()?;
    rucksacks
        .iter()
        .map(|rucksack| priority(rucksack.overlap()))
        .sum::<usize>()
        .into()
}

struct Rucksack(Vec<char>, Vec<char>);

impl Rucksack {
    fn overlap(&self) -> char {
        let left: HashSet<char> = HashSet::from_iter(self.0.clone());
        let right = HashSet::from_iter(self.1.clone());

        *left
            .intersection(&right)
            .map(|v| *v)
            .collect::<Vec<char>>()
            .get(0)
            .unwrap()
    }
}

fn parse_rucksack(input: &str) -> IResult<&str, Rucksack> {
    map(alpha1, |v: &str| {
        let chars = v.chars();
        let result = chars
            .chunks(v.len() / 2)
            .into_iter()
            .map(|v| v.collect::<Vec<char>>())
            .collect::<Vec<_>>();
        Rucksack(
            result.get(0).unwrap().to_vec(),
            result.get(1).unwrap().to_vec(),
        )
    })(input)
}

fn priority(input: char) -> usize {
    let base: usize = (input as u8 - 65).into();

    if input.is_uppercase() {
        base + 27
    } else {
        base - 31
    }
}

mod tests {
    use super::priority;
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        assert_eq!(super::run(input), Some(157))
    }

    #[test]
    fn test_priority() {
        assert_eq!(1, priority('a'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(52, priority('Z'));
    }
}
