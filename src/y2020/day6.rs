use crate::parser::*;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::HashSet;

fn parse_responses(input: &str) -> IResult<&str, HashSet<char>> {
    map(many1(single_char), |x| {
        x.into_iter().collect::<HashSet<_>>()
    })(input)
}

fn parse_group(input: &str) -> IResult<&str, HashSet<char>> {
    let (input, group) = separated_list1(tag("\n"), parse_responses)(input)?;

    Ok((
        input,
        group
            .into_iter()
            .fold_first(|acc, v| acc.intersection(&v).cloned().collect())
            .unwrap_or(HashSet::new()),
    ))
}

pub fn solve() {
    let input = include_str!("input-day6");

    let (_, groups) = separated_list1(tag("\n\n"), parse_group)(input).unwrap();

    println!(
        "Solution: {:?}",
        groups.iter().map(|v| v.len()).sum::<usize>()
    );
}
