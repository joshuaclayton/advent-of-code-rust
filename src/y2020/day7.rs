use crate::parser::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
enum ColorRule<'a> {
    Empty(&'a str),
    NonEmpty(&'a str, Vec<(usize, &'a str)>),
}

fn parse_color(input: &str) -> IResult<&str, &str> {
    terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag"))))(input)
}

fn parse_single_rule(input: &str) -> IResult<&str, (usize, &str)> {
    separated_pair(parse_usize, tag(" "), parse_color)(input)
}

fn parse_contents(input: &str) -> IResult<&str, Vec<(usize, &str)>> {
    separated_list1(tag(", "), parse_single_rule)(input)
}

fn parse_empty(input: &str) -> IResult<&str, ColorRule> {
    map(
        terminated(parse_color, tag(" contain no other bags")),
        ColorRule::Empty,
    )(input)
}

fn parse_non_empty(input: &str) -> IResult<&str, ColorRule> {
    let (input, (bag, rules)) =
        separated_pair(parse_color, tag(" contain "), parse_contents)(input)?;

    Ok((input, ColorRule::NonEmpty(bag, rules)))
}

fn parse_contains(input: &str) -> IResult<&str, ColorRule> {
    terminated(alt((parse_empty, parse_non_empty)), tag("."))(input)
}

pub fn solve() {
    let input = include_str!("input-day7");

    let (_, all) = separated_list1(tag("\n"), parse_contains)(input).unwrap();

    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();

    for contain in all {
        match contain {
            ColorRule::Empty(k) => result.insert(k, vec![]),
            ColorRule::NonEmpty(k, vs) => result.insert(k, vs.iter().map(|x| x.1).collect()),
        };
    }

    println!("Solution: {:?}", ancestors_for("shiny gold", &result).len());
}

fn ancestors_for<'a, 'b>(
    color: &'a str,
    rules: &HashMap<&'b str, Vec<&'b str>>,
) -> HashSet<&'b str> {
    let set = color_matches(color, rules);
    if set.is_empty() {
        set
    } else {
        let result = set.iter().fold(HashSet::new(), |acc, s| {
            acc.union(&ancestors_for(s, rules))
                .cloned()
                .collect::<HashSet<&str>>()
        });

        result.union(&set).cloned().collect()
    }
}

fn color_matches<'a, 'b>(
    color: &'a str,
    rules: &HashMap<&'b str, Vec<&'b str>>,
) -> HashSet<&'b str> {
    rules
        .iter()
        .filter(|(_, v)| v.contains(&color))
        .map(|(&k, _)| k)
        .collect::<HashSet<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_rules() {
        assert_eq!(
            parse_contains(
                "shimmering gold bags contain 1 glittery gold bag, 2 flourescent green bags."
            )
            .unwrap()
            .1,
            ColorRule::NonEmpty(
                "shimmering gold",
                vec![(1, "glittery gold"), (2, "flourescent green")]
            )
        );
    }

    #[test]
    fn singular_color() {
        assert_eq!(
            parse_single_rule("1 glittery gold bag").unwrap().1,
            (1, "glittery gold")
        );
    }

    #[test]
    fn singular_contents() {
        assert_eq!(
            parse_contents("1 glittery gold bag").unwrap().1,
            vec![(1, "glittery gold")]
        );
    }

    #[test]
    fn multiple_contents() {
        assert_eq!(
            parse_contents("1 glittery gold bag, 2 shining golden bags")
                .unwrap()
                .1,
            vec![(1, "glittery gold"), (2, "shining golden")]
        );
    }
}
