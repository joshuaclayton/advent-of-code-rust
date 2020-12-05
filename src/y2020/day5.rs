use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::HashMap;

#[derive(PartialEq)]
enum FB {
    F,
    B,
}

#[derive(PartialEq)]
enum LR {
    L,
    R,
}

fn parse_fb(input: &str) -> IResult<&str, FB> {
    alt((map(tag("F"), |_| FB::F), map(tag("B"), |_| FB::B)))(input)
}

fn parse_lr(input: &str) -> IResult<&str, LR> {
    alt((map(tag("L"), |_| LR::L), map(tag("R"), |_| LR::R)))(input)
}

fn parse_seat(input: &str) -> IResult<&str, Seat> {
    let (input, fbs) = many1(parse_fb)(input)?;
    let (input, lrs) = many1(parse_lr)(input)?;

    Ok((input, Seat::from((fbs, lrs))))
}

fn fb_set(position: usize) -> u8 {
    match position {
        0 => 0b1000_000,
        1 => 0b0100_000,
        2 => 0b0010_000,
        3 => 0b0001_000,
        4 => 0b0000_100,
        5 => 0b0000_010,
        6 => 0b0000_001,
        _ => panic!("this shouldn't happen"),
    }
}

fn lr_set(position: usize) -> u8 {
    match position {
        0 => 0b100,
        1 => 0b010,
        2 => 0b001,
        _ => panic!("this shouldn't happen"),
    }
}

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl From<(Vec<FB>, Vec<LR>)> for Seat {
    fn from((fbs, lrs): (Vec<FB>, Vec<LR>)) -> Self {
        let mut frs_byte = 0b0000_000;
        let mut lrs_byte = 0b000;
        for (i, fb) in fbs.iter().enumerate() {
            if fb == &FB::B {
                frs_byte |= fb_set(i);
            }
        }

        for (i, lr) in lrs.iter().enumerate() {
            if lr == &LR::R {
                lrs_byte |= lr_set(i);
            }
        }

        let row = usize::from(frs_byte);
        let column = usize::from(lrs_byte);
        Seat { row, column }
    }
}

fn gap(mut input: Vec<usize>) -> Option<usize> {
    let mut missing: Option<usize> = None;
    input.sort();
    let mut last = input.remove(0);

    for v in input {
        if v == last + 1 {
        } else {
            missing = Some(v - 1)
        }

        last = v
    }
    missing
}

pub fn solve() {
    let input = include_str!("input-day5");
    let (_, mut seats) = separated_list1(tag("\n"), parse_seat)(&input).unwrap();
    let mut rowed_seats: HashMap<usize, Vec<Seat>> = HashMap::new();
    let min_row: usize = seats.iter().min_by_key(|v| v.row).unwrap().row;
    let max_row: usize = seats.iter().max_by_key(|v| v.row).unwrap().row;

    for row in min_row..=max_row {
        rowed_seats.insert(
            row,
            seats.drain_filter(|s| s.row == row).collect::<Vec<_>>(),
        );
    }

    println!(
        "Solution: {:?}",
        rowed_seats
            .values()
            .filter_map(|v| gap(v.iter().map(|x| x.seat_id()).collect::<Vec<_>>()))
            .collect::<Vec<_>>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_example() {
        let seat = parse_seat("BFFFBBFRRR").unwrap().1;
        assert_eq!(seat.row, 70);
        assert_eq!(seat.column, 7);
        assert_eq!(seat.seat_id(), 567);
    }

    #[test]
    fn gap_works() {
        assert_eq!(gap(vec![1, 2, 3, 4]), None);
        assert_eq!(gap(vec![1, 2, 3, 5]), Some(4));
        assert_eq!(gap(vec![2, 3]), None);
    }
}
