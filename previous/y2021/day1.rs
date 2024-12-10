use crate::parser::*;
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

pub fn solve() {
    let input = include_str!("input-day1");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<usize> {
    let (_, depths) = parse_depths(input.trim()).ok()?;
    let triplets = depths.iter().tuple_windows::<(_, _, _)>();
    let mut last = 0;
    let mut increases = 0;
    for (first, second, third) in triplets {
        if (first + second + third) > last {
            increases += 1;
        }
        last = first + second + third;
    }

    Some(increases - 1)
}

fn parse_depths(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag("\n"), parse_usize)(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
        "#;
        assert_eq!(super::run(input), Some(5))
    }
}
