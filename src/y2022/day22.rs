use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

use crate::parser::parse_digits;

pub fn solve() {
    let input = include_str!("input-day22");
    println!("Answer: {:?}", run(input));
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy)]
struct Position(usize, usize);

trait Positioned {
    fn position(&self) -> Position;
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&mut self, rotation: Rotate) {
        match (&self, rotation) {
            (Direction::Up, Rotate::Left) => *self = Direction::Left,
            (Direction::Up, Rotate::Right) => *self = Direction::Right,
            (Direction::Down, Rotate::Left) => *self = Direction::Right,
            (Direction::Down, Rotate::Right) => *self = Direction::Left,
            (Direction::Left, Rotate::Left) => *self = Direction::Down,
            (Direction::Left, Rotate::Right) => *self = Direction::Up,
            (Direction::Right, Rotate::Left) => *self = Direction::Up,
            (Direction::Right, Rotate::Right) => *self = Direction::Down,
        }
    }
}

impl Positioned for Slot {
    fn position(&self) -> Position {
        match self {
            Slot::Space(p) => *p,
            Slot::Wall(p) => *p,
        }
    }
}

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

#[derive(Debug, Clone, Copy)]
enum Rotate {
    Left,
    Right,
}

#[derive(Clone, Debug)]
enum Movement {
    Move(usize),
    Rotate(Rotate),
}

fn run(input: &str) -> Option<usize> {
    let (_, (mut board, instructions)) = all_consuming(parse)(input).unwrap();

    for movement in instructions {
        board.apply_movement(movement)
    }

    let direction_score = match board.character.direction {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Right => 0,
        Direction::Left => 2,
    };

    Some(board.character.position().1 * 1000 + board.character.position().0 * 4 + direction_score)
}

#[derive(Debug, Clone)]
enum ParsedSlot {
    Space,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Slot {
    Space(Position),
    Wall(Position),
}

#[derive(Debug, Copy, Clone)]
struct Character {
    position: Position,
    direction: Direction,
}

impl Positioned for Character {
    fn position(&self) -> Position {
        self.position
    }
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Slot>>,
    tiles_map: HashMap<Position, Slot>,
    character: Character,
}

impl Board {
    fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Move(count) => {
                for _ in times(count) {
                    self.move_character(self.character.direction)
                }
            }
            Movement::Rotate(rotate) => {
                self.character.direction.rotate(rotate);
            }
        }
    }

    fn move_character(&mut self, direction: Direction) {
        match self.slot_in_direction(direction) {
            Slot::Space(new_position) => self.character.position = new_position,
            Slot::Wall(_) => (),
        }
    }

    fn right_most_in_row(&self, row: usize) -> usize {
        self.tiles
            .iter()
            .flatten()
            .map(|tile| tile.position())
            .filter(|p| p.1 == row)
            .map(|p| p.0)
            .max()
            .unwrap()
    }

    fn left_most_in_row(&self, row: usize) -> usize {
        self.tiles
            .iter()
            .flatten()
            .map(|tile| tile.position())
            .filter(|p| p.1 == row)
            .map(|p| p.0)
            .min()
            .unwrap()
    }

    fn bottom_most_in_column(&self, column: usize) -> usize {
        self.tiles
            .iter()
            .flatten()
            .map(|tile| tile.position())
            .filter(|p| p.0 == column)
            .map(|p| p.1)
            .max()
            .unwrap()
    }

    fn top_most_in_column(&self, column: usize) -> usize {
        self.tiles
            .iter()
            .flatten()
            .map(|tile| tile.position())
            .filter(|p| p.0 == column)
            .map(|p| p.1)
            .min()
            .unwrap()
    }

    fn slot_in_direction(&self, direction: Direction) -> Slot {
        let target_position = match direction {
            Direction::Up => Position(self.character.position().0, self.character.position().1 - 1),
            Direction::Down => {
                Position(self.character.position().0, self.character.position().1 + 1)
            }
            Direction::Left => {
                Position(self.character.position().0 - 1, self.character.position().1)
            }
            Direction::Right => {
                Position(self.character.position().0 + 1, self.character.position().1)
            }
        };

        if let Some(&found_slot) = self.tiles_map.get(&target_position) {
            found_slot
        } else {
            match direction {
                Direction::Up => *self
                    .tiles_map
                    .get(&Position(
                        self.character.position().0,
                        self.bottom_most_in_column(self.character.position().0),
                    ))
                    .unwrap(),
                Direction::Down => *self
                    .tiles_map
                    .get(&Position(
                        self.character.position().0,
                        self.top_most_in_column(self.character.position().0),
                    ))
                    .unwrap(),
                Direction::Left => *self
                    .tiles_map
                    .get(&Position(
                        self.right_most_in_row(self.character.position().1),
                        self.character.position().1,
                    ))
                    .unwrap(),
                Direction::Right => *self
                    .tiles_map
                    .get(&Position(
                        self.left_most_in_row(self.character.position().1),
                        self.character.position().1,
                    ))
                    .unwrap(),
            }
        }
    }
}

fn parse(input: &str) -> IResult<&str, (Board, Vec<Movement>)> {
    separated_pair(parse_board, tag("\n\n"), parse_movement)(input.trim_end())
}

fn parse_board(input: &str) -> IResult<&str, Board> {
    map(separated_list1(tag("\n"), parse_row), |rows| {
        let mut tiles = vec![];
        let mut tiles_map = HashMap::new();

        for (y, row) in rows.iter().enumerate() {
            let mut row_results = vec![];

            for (x, col) in row.iter().enumerate() {
                let position = Position(x + 1, y + 1);

                match col {
                    None => (),
                    Some(ParsedSlot::Space) => {
                        row_results.push(Slot::Space(position));
                        tiles_map.insert(position, Slot::Space(position));
                    }
                    Some(ParsedSlot::Wall) => {
                        row_results.push(Slot::Wall(position));
                        tiles_map.insert(position, Slot::Wall(position));
                    }
                }
            }

            tiles.push(row_results);
        }
        let character = Character {
            position: tiles[0][0].position(),
            direction: Direction::Right,
        };

        Board {
            tiles,
            tiles_map,
            character,
        }
    })(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Option<ParsedSlot>>> {
    many1(parse_slot)(input)
}

fn parse_slot(input: &str) -> IResult<&str, Option<ParsedSlot>> {
    alt((
        value(None, tag(" ")),
        value(Some(ParsedSlot::Space), tag(".")),
        value(Some(ParsedSlot::Wall), tag("#")),
    ))(input)
}

fn parse_movement(input: &str) -> IResult<&str, Vec<Movement>> {
    many1(alt((
        map(parse_digits, Movement::Move),
        value(Movement::Rotate(Rotate::Left), tag("L")),
        value(Movement::Rotate(Rotate::Right), tag("R")),
    )))(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
        assert_eq!(super::run(input), Some(6032))
    }
}
