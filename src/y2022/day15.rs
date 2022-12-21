use crate::parser::*;
use nom::{
    bytes::complete::tag,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};
use std::{collections::HashSet, ops::Range};

pub fn solve() {
    let input = include_str!("input-day15");
    println!("Answer: {:?}", run(input, 2000000));
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
    fn manhattan_distances_unavailable(&self, y: isize) -> HashSet<Point> {
        let y_offset = (self.point.1 - y).abs();
        let width = self.manhattan_distance_to(&self.closest_beacon) as isize - y_offset;

        let mut results = HashSet::new();

        for width_offset in (Range {
            start: width * -1,
            end: width + 1,
        }) {
            results.insert(Point(self.point.0 + width_offset, y));
        }

        results
    }
}

fn run(input: &str, row: isize) -> Option<usize> {
    let (_, sensors) =
        all_consuming(separated_list1(tag("\n"), parse_sensor))(input.trim()).ok()?;
    let points = sensors.iter().fold(HashSet::new(), |acc, s| {
        s.manhattan_distances_unavailable(row)
            .union(&acc)
            .cloned()
            .collect()
    });

    let taken_points = sensors
        .iter()
        .flat_map(|s| vec![s.point, s.closest_beacon.0])
        .collect::<HashSet<Point>>();

    Some(points.difference(&taken_points).count())
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
}
