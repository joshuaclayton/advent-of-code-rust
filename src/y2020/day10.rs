use crate::parser::*;
use nom::{bytes::complete::tag, multi::separated_list1};

pub fn solve() {
    let input = include_str!("input-day10");

    let (_, mut adapter_list) = separated_list1(tag("\n"), parse_usize)(input).unwrap();
    adapter_list.sort();

    let mut previous_value = 0;
    let mut add_one = 0;
    let mut add_three = 0;

    for v in adapter_list {
        match v - previous_value {
            1 => {
                add_one += 1;
            }
            3 => {
                add_three += 1;
            }
            _ => (),
        }

        previous_value = v;
    }

    println!("Solution: {:?}", add_one * (add_three + 1));
}
