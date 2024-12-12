use nom::{
    bytes::complete::tag,
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use std::collections::HashMap;

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("input-day1");
    let (_, pairs) = parse_input(input)?;

    let lefts = pairs.iter().map(|p| p.0).collect::<Vec<_>>();
    let rights = tally(pairs.iter().map(|p| p.1));

    let mut similarity_score = 0;

    lefts.iter().for_each(|l| {
        similarity_score += (*rights.get(l).unwrap_or(&0) as isize) * *l;
    });

    println!("similarity score: {}", similarity_score);

    Ok(())
}

#[derive(Debug, Clone)]
struct Pair(isize, isize);

fn tally<T, I>(iterable: I) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash,
    I: IntoIterator<Item = T>,
{
    let mut tally = HashMap::new();

    for value in iterable {
        *tally.entry(value).or_insert(0) += 1;
    }

    tally
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pair>> {
    all_consuming(terminated(
        separated_list1(tag("\n"), parse_pair),
        tag("\n"),
    ))(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, left) = parser::parse_digits(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, right) = parser::parse_digits(input)?;

    Ok((input, Pair(left, right)))
}
