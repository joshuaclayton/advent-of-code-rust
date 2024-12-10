use crate::parser::*;
use nom::{bytes::complete::tag, combinator::map, multi::separated_list1, IResult};

struct Numbers {
    numbers: Vec<usize>,
}

impl Numbers {
    fn new(numbers: Vec<usize>) -> Self {
        Self { numbers }
    }

    fn answer(&self) -> usize {
        let result;
        let mut position = 0;
        loop {
            match exact_sum(&self.numbers[position..]) {
                Some(ending) => {
                    result = ending;
                    break;
                }
                None => {
                    position += 1;
                }
            }
        }

        result
    }
}

fn exact_sum(numbers: &[usize]) -> Option<usize> {
    let mut total = 0;
    let mut values = vec![];
    let mut result = None;

    for num in numbers {
        values.push(*num);
        total += num;
        if total == 70639851 {
            result = Some(values.iter().min().unwrap() + values.iter().max().unwrap());
            break;
        } else if total > 70639851 {
            break;
        }
    }

    result
}

fn parse_numbers(input: &str) -> IResult<&str, Numbers> {
    map(separated_list1(tag("\n"), parse_usize), Numbers::new)(input)
}

pub fn solve() {
    let input = include_str!("input-day9");

    let (_, numbers) = parse_numbers(input).unwrap();

    println!("Solution: {:?}", numbers.answer());
}
