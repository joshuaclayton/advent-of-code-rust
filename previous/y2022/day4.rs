use crate::parser::*;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::ops::Range;

#[derive(Debug)]
struct Elf(Range<usize>);

fn partially_contained(left: &Elf, right: &Elf) -> bool {
    left.0.contains(&right.0.start) || left.0.contains(&(right.0.end - 1))
}

pub fn solve() {
    let input = include_str!("input-day4");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<usize> {
    let (_, mut elf_pairs) =
        all_consuming(separated_list1(tag("\n"), parse_elf_pair))(input.trim()).ok()?;
    elf_pairs.retain(|(left, right)| {
        partially_contained(left, right) || partially_contained(right, left)
    });
    elf_pairs.len().into()
}

fn parse_elf_pair(input: &str) -> IResult<&str, (Elf, Elf)> {
    map(
        separated_pair(parse_range, tag(","), parse_range),
        |(left, right)| (Elf(left), Elf(right)),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, Range<usize>> {
    map(
        separated_pair(parse_usize, tag("-"), parse_usize),
        |(start, end)| Range {
            start,
            end: end + 1,
        },
    )(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "#;
        assert_eq!(super::run(input), Some(4))
    }
}
