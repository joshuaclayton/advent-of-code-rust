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
    fn follow(&mut self, knot: &impl Positioned);
}

trait Positioned {
    fn position(&self) -> Position;

    fn adjacent_to_knot(&self, knot: &impl Positioned) -> bool {
        (knot.position().0 - self.position().0).abs() < 2
            && (knot.position().1 - self.position().1).abs() < 2
    }

    fn direction_to_move(&self, knot: &impl Positioned) -> Option<Direction> {
        if !self.adjacent_to_knot(knot) {
            match (
                knot.position().0.cmp(&self.position().0),
                knot.position().1.cmp(&self.position().1),
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Knot(Position);

#[derive(Debug, Default)]
struct Rope {
    body: [Knot; 9],
    tail: Tail,
}

impl Positioned for Knot {
    fn position(&self) -> Position {
        self.0
    }
}

impl Positioned for Tail {
    fn position(&self) -> Position {
        self.position
    }
}

impl Moveable for Tail {
    fn move_in_direction(&mut self, direction: &Direction) {
        self.position.move_in_direction(direction)
    }

    fn follow(&mut self, knot: &impl Positioned) {
        if let Some(direction) = self.direction_to_move(knot) {
            self.move_in_direction(&direction);
            self.visited.push(self.position);
        }
    }
}

impl Moveable for Knot {
    fn move_in_direction(&mut self, direction: &Direction) {
        self.0.move_in_direction(direction)
    }

    fn follow(&mut self, knot: &impl Positioned) {
        if let Some(direction) = self.direction_to_move(knot) {
            self.move_in_direction(&direction);
        }
    }
}

impl Moveable for Rope {
    fn move_in_direction(&mut self, direction: &Direction) {
        let mut prev: Option<Knot> = None;

        for knot in self.body.iter_mut() {
            match prev {
                None => {
                    knot.move_in_direction(direction);
                    prev = Some(*knot);
                }
                Some(previous) => {
                    knot.follow(&previous);
                    prev = Some(*knot);
                }
            }
        }

        if let Some(previous) = prev {
            self.tail.follow(&previous);
        }
    }

    fn follow(&mut self, _: &impl Positioned) {}
}

impl Default for Knot {
    fn default() -> Self {
        Knot(Position(0, 0))
    }
}

#[derive(Debug)]
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

fn run(input: &str) -> Option<usize> {
    let (_, instructions) =
        all_consuming(separated_list1(tag("\n"), parse_instruction))(input.trim()).ok()?;

    let mut rope = Rope::default();

    for instruction in &instructions {
        for _ in times(instruction.1) {
            rope.move_in_direction(&instruction.0);
        }
    }

    Some(
        rope.tail
            .visited
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
        assert_eq!(run(input), Some(1))
    }

    #[test]
    fn solve_returns_the_correct_value_bigger() {
        let input = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
            "#;
        assert_eq!(run(input), Some(36))
    }

    #[test]
    fn adjacency() {
        assert!(Tail {
            position: Position(0, 0),
            visited: vec![]
        }
        .adjacent_to_knot(&Knot::default()));

        assert!(Tail {
            position: Position(1, 0),
            visited: vec![]
        }
        .adjacent_to_knot(&Knot::default()));

        assert!(Tail {
            position: Position(-1, -1),
            visited: vec![]
        }
        .adjacent_to_knot(&Knot::default()));

        assert!(!Tail {
            position: Position(2, 0),
            visited: vec![]
        }
        .adjacent_to_knot(&Knot::default()));
    }

    #[test]
    fn following() {
        let mut tail = Tail {
            position: Position(0, 0),
            visited: vec![],
        };

        tail.follow(&Knot(Position(-3, 0)));
        assert_eq!(tail.position, Position(-1, 0));

        tail.follow(&Knot(Position(-3, 0)));
        assert_eq!(tail.position, Position(-2, 0));

        tail.follow(&Knot(Position(-3, 0)));
        assert_eq!(tail.position, Position(-2, 0));

        tail.follow(&Knot(Position(-3, 1)));
        assert_eq!(tail.position, Position(-2, 0));

        tail.follow(&Knot(Position(-3, 2)));
        assert_eq!(tail.position, Position(-3, 1));
    }

    #[test]
    fn direction() {
        let tail = Tail {
            position: Position(0, 0),
            visited: vec![],
        };

        assert_eq!(
            tail.direction_to_move(&Knot(Position(-3, 0))).unwrap(),
            Direction::West
        );

        assert_eq!(
            tail.direction_to_move(&Knot(Position(0, 2))).unwrap(),
            Direction::North
        );

        assert_eq!(
            tail.direction_to_move(&Knot(Position(-3, 2))).unwrap(),
            Direction::NorthWest
        );
    }
}
