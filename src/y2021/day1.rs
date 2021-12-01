use crate::parser::*;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

pub fn solve() {
    let input = include_str!("input-day1");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<usize> {
    let (_, depths) = parse_depths(input.trim()).ok()?;
    let mut last = 0;
    let mut increases = 0;
    for v in depths {
        if v > last {
            increases += 1;
        }
        last = v;
    }

    Some(increases - 1)
}

fn parse_depths(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag("\n"), parse_usize)(input)
}

mod tests {
    use super::run;

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
        assert_eq!(run(input), Some(7))
    }
}
