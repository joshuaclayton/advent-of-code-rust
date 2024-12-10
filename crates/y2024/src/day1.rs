use nom::{
    bytes::complete::tag,
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("input-day1");
    let (_, pairs) = parse_input(input)?;

    let mut lefts = pairs.iter().map(|p| p.0).collect::<Vec<_>>();
    let mut rights = pairs.iter().map(|p| p.1).collect::<Vec<_>>();

    lefts.sort();
    rights.sort();

    let mut distance = 0;

    lefts.iter().zip(rights).for_each(|(l, r)| {
        distance += (r - l).abs();
        println!("{} {}", l, r);
    });

    println!("distance: {}", distance);

    Ok(())
}

#[derive(Debug, Clone)]
struct Pair(isize, isize);

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
