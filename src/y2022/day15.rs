use crate::parser::*;
use nom::{
    bytes::complete::tag,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};
use std::ops::Range;

pub fn solve() {
    let input = include_str!("input-day15");
    for n in 0..4000000 {
        run(input, n);
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point(isize, isize);

trait Locatable {
    fn point(&self) -> Point;
    fn manhattan_distance_to(&self, other: &impl Locatable) -> usize {
        (self.point().0 - other.point().0).abs() as usize
            + (self.point().1 - other.point().1).abs() as usize
    }
}

#[derive(Debug)]
struct Beacon(Point);

#[derive(Debug)]
struct Sensor {
    point: Point,
    closest_beacon: Beacon,
}

impl Locatable for Beacon {
    fn point(&self) -> Point {
        self.0
    }
}

impl Locatable for Sensor {
    fn point(&self) -> Point {
        self.point
    }
}

impl Sensor {
    fn manhattan_distances_unavailable(&self, y: isize) -> Option<Range<isize>> {
        let y_offset = (self.point.1 - y).abs();
        let width = self.manhattan_distance_to(&self.closest_beacon) as isize - y_offset;

        if width < 0 {
            None
        } else {
            Some(Range {
                start: width * -1 + self.point.0,
                end: width + 1 + self.point.0,
            })
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Ranges(Vec<Range<isize>>);

impl Ranges {
    fn add_range(&mut self, range: Range<isize>) {
        let mut push = false;

        for range_mut in self.0.iter_mut() {
            if flatten_range(range_mut, &range) {
                push = true
            }
        }

        if push || self.0.is_empty() {
            self.0.push(range);
        }
    }

    fn flatten(&mut self) {
        let mut res = Ranges::default();

        for r in sort_ranges(self.0.clone()) {
            res.add_range(r);
        }

        *self = res;
    }
}

fn sort_ranges(ranges: Vec<Range<isize>>) -> Vec<Range<isize>> {
    let mut new_ranges = ranges.clone();

    new_ranges.sort_by_key(|r| r.start);

    new_ranges
}

fn flatten_range(left: &mut Range<isize>, right: &Range<isize>) -> bool {
    let mut contained = false;
    let mut push = false;

    match (left.contains(&right.start), left.contains(&right.end)) {
        (true, true) => contained = true,
        (true, false) => {
            left.end = right.end;
        }
        (false, true) => {
            left.start = right.start;
        }
        (false, false) => {
            if right.contains(&left.start) && right.contains(&left.end) {
                left.start = right.start;
                left.end = right.end;
            } else {
                push = true;
            }
        }
    }

    if contained {
        false
    } else {
        if push {
            true
        } else {
            false
        }
    }
}

fn run(input: &str, row: isize) -> Option<usize> {
    let (_, sensors) =
        all_consuming(separated_list1(tag("\n"), parse_sensor))(input.trim()).ok()?;
    let mut ranges = vec![];
    for sensor in &sensors {
        if let Some(range) = sensor.manhattan_distances_unavailable(row) {
            ranges.push(range);
        }
    }

    let mut r = Ranges::default();

    for range in sort_ranges(ranges) {
        r.add_range(range);
    }

    r.flatten();

    if r.0.len() > 1 {
        println!("{:?} {}", r, row);
    }

    None
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, sensor_point) = preceded(tag("Sensor at "), parse_point)(input)?;
    let (input, beacon_point) = preceded(tag(": closest beacon is at "), parse_point)(input)?;

    Ok((
        input,
        Sensor {
            point: sensor_point,
            closest_beacon: Beacon(beacon_point),
        },
    ))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = preceded(tag("x="), terminated(parse_digits, tag(", ")))(input)?;
    let (input, y) = preceded(tag("y="), parse_digits)(input)?;

    Ok((input, Point(x, y)))
}

mod tests {
    use super::*;

    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;
        assert_eq!(super::run(input, 10), Some(26))
    }

    #[test]
    fn test_adding_ranges() {
        let mut ranges = Ranges::default();

        ranges.add_range(Range { start: -5, end: 5 });
        ranges.add_range(Range { start: -3, end: 2 });
        ranges.add_range(Range {
            start: -10,
            end: 10,
        });
        ranges.add_range(Range { start: 2, end: 19 });

        assert_eq!(
            ranges.0,
            vec![Range {
                start: -10,
                end: 19
            },]
        );
    }
}
