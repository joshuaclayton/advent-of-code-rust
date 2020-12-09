use crate::parser::*;
use itertools::*;
use nom::{bytes::complete::tag, combinator::map, multi::separated_list1, IResult};

struct Numbers {
    window: usize,
    position: usize,
    numbers: Vec<usize>,
}

impl Numbers {
    fn new(numbers: Vec<usize>) -> Self {
        Self {
            window: 25,
            position: 0,
            numbers,
        }
    }
}

impl Iterator for Numbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if check_validity(
            &self.numbers[self.position..(self.position + self.window)],
            self.numbers[self.position + self.window],
        ) {
            self.position += 1;
            Some(self.numbers[self.position + self.window])
        } else {
            None
        }
    }
}

fn check_validity(numbers: &[usize], value: usize) -> bool {
    numbers
        .iter()
        .combinations(2)
        .map(|v| v[0] + v[1])
        .collect::<Vec<_>>()
        .contains(&value)
}

fn parse_numbers(input: &str) -> IResult<&str, Numbers> {
    map(separated_list1(tag("\n"), parse_usize), Numbers::new)(input)
}

pub fn solve() {
    let input = include_str!("input-day9");

    let (_, numbers) = parse_numbers(input).unwrap();

    println!("Solution: {:?}", numbers.last());
}
