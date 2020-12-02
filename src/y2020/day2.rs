use crate::parser::*;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Requirement {
    occurrences: std::ops::Range<usize>,
    character: char,
}

#[derive(Debug, PartialEq)]
struct Password<'a>(&'a str);

impl<'a> Password<'a> {
    fn meets_requirement(&self, requirement: &Requirement) -> bool {
        requirement
            .occurrences
            .contains(&self.0.matches(requirement.character).count())
    }
}

fn parse_requirement(input: &str) -> IResult<&str, Requirement> {
    let (input, (start, end)) = separated_pair(parse_usize, tag("-"), parse_usize)(input)?;
    let (input, character) = terminated(preceded(tag(" "), single_char), tag(":"))(input)?;
    let occurrences = start..end + 1;

    Ok((
        input,
        Requirement {
            occurrences,
            character,
        },
    ))
}

fn parse_password<'a>(input: &'a str) -> IResult<&'a str, Password<'a>> {
    map(to_newline, Password)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Requirement, Password)> {
    separated_pair(parse_requirement, tag(" "), parse_password)(input)
}

pub fn solve() {
    let input = include_str!("input-day2");
    let (_, all) = separated_list1(tag("\n"), parse_line)(&input).unwrap();
    let valid_passwords = all
        .iter()
        .filter(|(requirement, password)| password.meets_requirement(requirement));
    println!("Solution: {:?}", valid_passwords.count());
}
