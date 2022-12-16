use std::collections::{BTreeMap, HashMap};

use crate::parser::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space0, space1},
    combinator::{map, opt, value},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

pub fn solve() {
    let input = include_str!("input-day5");
    println!("Answer: {:?}", run(input));
}

struct Point(usize, usize);

enum Position {
    Starting(Point),
    Point(Point, usize),
    Ending(Point),
}

fn run(input: &str) -> Option<usize> {
    let matrix = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map: BTreeMap<usize, Position> = BTreeMap::new();
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {}
    }

    None
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;
        assert_eq!(super::run(input), Some(31))
    }
}
