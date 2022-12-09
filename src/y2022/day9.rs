use crate::parser::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{cmp::Ordering, collections::HashSet};

pub fn solve() {
    let input = include_str!("input-day9");
    println!("Answer: {:?}", run(input));
}

trait Moveable {
    fn move_in_direction(&mut self, direction: &Direction);
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position(isize, isize);

impl Position {
    fn move_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.1 += 1,
            Direction::South => self.1 -= 1,
            Direction::East => self.0 += 1,
            Direction::West => self.0 -= 1,
            Direction::NorthEast => {
                self.move_in_direction(&Direction::North);
                self.move_in_direction(&Direction::East);
            }
            Direction::NorthWest => {
                self.move_in_direction(&Direction::North);
                self.move_in_direction(&Direction::West);
            }
            Direction::SouthEast => {
                self.move_in_direction(&Direction::South);
                self.move_in_direction(&Direction::East);
            }
            Direction::SouthWest => {
                self.move_in_direction(&Direction::South);
                self.move_in_direction(&Direction::West);
            }
        }
    }
}

struct Head(Position);

impl Moveable for Tail {
    fn move_in_direction(&mut self, direction: &Direction) {
        self.position.move_in_direction(direction)
    }
}

impl Moveable for Head {
    fn move_in_direction(&mut self, direction: &Direction) {
        self.0.move_in_direction(direction)
    }
}

impl Default for Head {
    fn default() -> Self {
        Head(Position(0, 0))
    }
}

struct Tail {
    position: Position,
    visited: Vec<Position>,
}

impl Default for Tail {
    fn default() -> Self {
        Tail {
            position: Position(0, 0),
            visited: vec![Position(0, 0)],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Tail {
    fn follow(&mut self, head: &Head) {
        if let Some(direction) = self.direction_to_move(head) {
            self.move_in_direction(&direction);
            self.visited.push(self.position);
        }
    }

    fn adjacent_to_head(&self, head: &Head) -> bool {
        (head.0 .0 - self.position.0).abs() < 2 && (head.0 .1 - self.position.1).abs() < 2
    }

    fn direction_to_move(&self, head: &Head) -> Option<Direction> {
        if !self.adjacent_to_head(head) {
            match (
                head.0 .0.cmp(&self.position.0),
                head.0 .1.cmp(&self.position.1),
            ) {
                (Ordering::Equal, Ordering::Greater) => Some(Direction::North),
                (Ordering::Equal, Ordering::Less) => Some(Direction::South),
                (Ordering::Greater, Ordering::Equal) => Some(Direction::East),
                (Ordering::Less, Ordering::Equal) => Some(Direction::West),
                (Ordering::Greater, Ordering::Greater) => Some(Direction::NorthEast),
                (Ordering::Greater, Ordering::Less) => Some(Direction::SouthEast),
                (Ordering::Less, Ordering::Greater) => Some(Direction::NorthWest),
                (Ordering::Less, Ordering::Less) => Some(Direction::SouthWest),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn run(input: &str) -> Option<usize> {
    let (_, instructions) =
        all_consuming(separated_list1(tag("\n"), parse_instruction))(input.trim()).ok()?;

    let mut head = Head::default();
    let mut tail = Tail::default();

    for instruction in &instructions {
        for _ in times(instruction.1) {
            head.move_in_direction(&instruction.0);
            tail.follow(&head);
        }
    }

    Some(
        tail.visited
            .into_iter()
            .collect::<HashSet<Position>>()
            .len(),
    )
}

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

#[derive(Debug)]
struct Instruction(Direction, usize);

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        separated_pair(parse_direction, tag(" "), parse_usize),
        |(direction, count)| Instruction(direction, count),
    )(input)
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::North, tag("U")),
        value(Direction::South, tag("D")),
        value(Direction::East, tag("R")),
        value(Direction::West, tag("L")),
    ))(input)
}

mod tests {
    use super::*;

    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
        "#;
        assert_eq!(run(input), Some(13))
    }

    #[test]
    fn adjacency() {
        assert!(Tail {
            position: Position(0, 0),
            visited: vec![]
        }
        .adjacent_to_head(&Head::default()));

        assert!(Tail {
            position: Position(1, 0),
            visited: vec![]
        }
        .adjacent_to_head(&Head::default()));

        assert!(Tail {
            position: Position(-1, -1),
            visited: vec![]
        }
        .adjacent_to_head(&Head::default()));

        assert!(!Tail {
            position: Position(2, 0),
            visited: vec![]
        }
        .adjacent_to_head(&Head::default()));
    }

    #[test]
    fn following() {
        let mut tail = Tail {
            position: Position(0, 0),
            visited: vec![],
        };

        tail.follow(&Head(Position(-3, 0)));
        assert_eq!(tail.position, Position(-1, 0));

        tail.follow(&Head(Position(-3, 0)));
        assert_eq!(tail.position, Position(-2, 0));

        tail.follow(&Head(Position(-3, 0)));
        assert_eq!(tail.position, Position(-2, 0));

        tail.follow(&Head(Position(-3, 1)));
        assert_eq!(tail.position, Position(-2, 0));

        tail.follow(&Head(Position(-3, 2)));
        assert_eq!(tail.position, Position(-3, 1));
    }

    #[test]
    fn direction() {
        let tail = Tail {
            position: Position(0, 0),
            visited: vec![],
        };

        assert_eq!(
            tail.direction_to_move(&Head(Position(-3, 0))).unwrap(),
            Direction::West
        );

        assert_eq!(
            tail.direction_to_move(&Head(Position(0, 2))).unwrap(),
            Direction::North
        );

        assert_eq!(
            tail.direction_to_move(&Head(Position(-3, 2))).unwrap(),
            Direction::NorthWest
        );
    }
}
