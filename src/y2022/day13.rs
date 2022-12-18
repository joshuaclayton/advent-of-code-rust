use crate::parser::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::cmp::Ordering;

pub fn solve() {
    let input = include_str!("input-day13");
    println!("Answer: {:?}", run(input));
}

fn build_divider_packet_with_value(value: usize) -> PacketData {
    PacketData::List(vec![Box::new(PacketData::List(vec![Box::new(
        PacketData::Value(value),
    )]))])
}

fn run(input: &str) -> Option<usize> {
    let (_, packets) = all_consuming(parse_packets)(input.trim()).ok()?;

    let mut all_packets = vec![];
    for (index, (l, r)) in packets.iter().enumerate() {
        all_packets.push(PacketData::List(
            l.clone().into_iter().map(Box::new).collect::<Vec<_>>(),
        ));
        all_packets.push(PacketData::List(
            r.clone().into_iter().map(Box::new).collect::<Vec<_>>(),
        ));
    }

    let v2 = build_divider_packet_with_value(2);
    let v6 = build_divider_packet_with_value(6);
    all_packets.push(v2.clone());
    all_packets.push(v6.clone());

    all_packets.sort_by(|l, r| compare_packets(l, r));
    dbg!(&all_packets);

    Some(
        (all_packets.iter().position(|v| v == &v2).unwrap() + 1)
            * (all_packets.iter().position(|v| v == &v6).unwrap() + 1),
    )
    // Some(comparisons.iter().sum())
}

fn compare_packets<'a>(left: &'a PacketData, right: &'a PacketData) -> Ordering {
    match (left, right) {
        (PacketData::Value(left_val), PacketData::Value(right_val)) => left_val.cmp(right_val),
        (value @ PacketData::Value(_), list) => {
            compare_packets(&PacketData::List(vec![Box::new(value.clone())]), list)
        }
        (list, value @ PacketData::Value(_)) => {
            compare_packets(list, &PacketData::List(vec![Box::new(value.clone())]))
        }
        (PacketData::List(left_list), PacketData::List(right_list)) => {
            let mut current_order = Ordering::Equal;

            for (idx, l) in left_list.iter().enumerate() {
                if current_order == Ordering::Equal {
                    if let Some(r) = right_list.get(idx) {
                        current_order = compare_packets(l, r);
                    } else {
                        current_order = Ordering::Greater;
                    }
                }
            }

            if current_order == Ordering::Equal {
                left_list.len().cmp(&right_list.len())
            } else {
                current_order
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum PacketData {
    Value(usize),
    List(Vec<Box<PacketData>>),
}

fn parse_packets(input: &str) -> IResult<&str, Vec<(Vec<PacketData>, Vec<PacketData>)>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(parse_packet_data_list, tag("\n"), parse_packet_data_list),
    )(input)
}

fn parse_packet_data(input: &str) -> IResult<&str, PacketData> {
    alt((
        parse_packet_data_value,
        map(parse_packet_data_list, |list| {
            PacketData::List(list.into_iter().map(Box::new).collect::<Vec<_>>())
        }),
    ))(input)
}

fn parse_packet_data_value(input: &str) -> IResult<&str, PacketData> {
    map(parse_usize, PacketData::Value)(input)
}

fn parse_packet_data_list(input: &str) -> IResult<&str, Vec<PacketData>> {
    preceded(
        tag("["),
        terminated(separated_list0(tag(","), parse_packet_data), tag("]")),
    )(input)
}

mod tests {
    use super::*;

    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
        "#;
        assert_eq!(run(input), Some(140))
    }
}
