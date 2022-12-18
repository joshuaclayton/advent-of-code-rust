use crate::parser::*;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;

pub fn solve() {
    let input = include_str!("input-day14");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<usize> {
    let mut walls = vec![];

    let (_, all_points) =
        all_consuming(separated_list1(tag("\n"), parse_line))(input.trim()).ok()?;

    for points in all_points {
        for pair in points.as_slice().windows(2) {
            walls.extend(pair[0].to_position(pair[1]));
        }
    }

    let mut board = Board {
        walls,
        sand: vec![],
        running: true,
    };

    let mut dropped_sand = 0;

    while board.running {
        let sand = Position::default();

        board.fall(sand);
        dropped_sand += 1;
    }

    Some(dropped_sand - 1)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Position>> {
    separated_list1(tag(" -> "), parse_position)(input)
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    map(
        separated_pair(parse_usize, tag(","), parse_usize),
        |(x, y)| Position(x, y),
    )(input)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position(usize, usize);

impl Default for Position {
    fn default() -> Self {
        Position(500, 0)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Position {
    fn down(&self) -> Position {
        Position(self.0, self.1 + 1)
    }

    fn up(&self) -> Position {
        Position(self.0, self.1 - 1)
    }

    fn right(&self) -> Position {
        Position(self.0 + 1, self.1)
    }

    fn left(&self) -> Position {
        Position(self.0 - 1, self.1)
    }

    fn move_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Down => {
                *self = self.down();
            }
            Direction::Up => {
                *self = self.up();
            }
            Direction::Right => {
                *self = self.right();
            }
            Direction::Left => {
                *self = self.left();
            }
        }
    }

    fn direction_to(&self, Position(target_x, target_y): Position) -> Option<Direction> {
        match (self.0.cmp(&target_x), self.1.cmp(&target_y)) {
            (Ordering::Equal, Ordering::Less) => Some(Direction::Down),
            (Ordering::Equal, Ordering::Greater) => Some(Direction::Up),
            (Ordering::Less, Ordering::Equal) => Some(Direction::Right),
            (Ordering::Greater, Ordering::Equal) => Some(Direction::Left),
            _ => None,
        }
    }

    fn to_position(&self, position: Position) -> Vec<Position> {
        let mut results = vec![];
        let mut current_position = self.clone();

        let direction = current_position.direction_to(position).unwrap();

        while current_position != position {
            results.push(current_position);
            current_position.move_in_direction(&direction);
        }

        results.push(position);

        results
    }
}

type Walls = Vec<Position>;
type Sand = Vec<Position>;

struct Board {
    walls: Walls,
    sand: Sand,
    running: bool,
}

impl Board {
    fn fall(&mut self, mut sand: Position) {
        let mut moving = true;

        let mut iter_count = 0;

        while moving && self.running {
            iter_count += 1;

            match (
                self.blocked(sand.down().left()),
                self.blocked(sand.down()),
                self.blocked(sand.down().right()),
            ) {
                (None, Some(_), _) => {
                    sand = sand.down().left();
                }
                (Some(_), Some(_), None) => {
                    sand = sand.down().right();
                }
                (_, None, _) => {
                    sand = sand.down();
                }
                (Some(_), Some(_), Some(_)) => {
                    moving = false;
                    self.sand.push(sand);
                }
            }

            if iter_count > 200 {
                self.running = false;
            }
        }
    }

    fn blocked(&self, position: Position) -> Option<Position> {
        self.walls
            .iter()
            .find(|&v| v == &position)
            .or(self.sand.iter().find(|&v| v == &position))
            .copied()
    }
}

mod tests {
    use super::*;

    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
        "#;
        assert_eq!(super::run(input), Some(24))
    }

    #[test]
    fn direction_to_other_point() {
        assert_eq!(
            Position(1, 1).direction_to(Position(3, 1)),
            Some(Direction::Right)
        );

        assert_eq!(
            Position(3, 0).direction_to(Position(3, 1)),
            Some(Direction::Down)
        );

        assert_eq!(
            Position(3, 1).direction_to(Position(3, 0)),
            Some(Direction::Up)
        );

        assert_eq!(
            Position(3, 1).direction_to(Position(1, 1)),
            Some(Direction::Left)
        );
    }
}
