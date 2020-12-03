use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Tile {
    Open,
    Tree,
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    let parse_open = map(tag("."), |_| Tile::Open);
    let parse_tree = map(tag("#"), |_| Tile::Tree);
    alt((parse_open, parse_tree))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(parse_tile)(input)
}

type Coordinates = (usize, usize);

fn step((x, y): Coordinates) -> Coordinates {
    (x + 3, y + 1)
}

pub fn solve() {
    let input = include_str!("input-day3");
    let (_, all) = separated_list1(tag("\n"), parse_line)(&input).unwrap();
    let mut start = (0, 0);
    let mut tiles = vec![];
    let width = all[0].len();

    while start.1 < all.len() - 1 {
        start = step(start);
        tiles.push(&all[start.1][start.0.rem_euclid(width)]);
    }

    println!(
        "Solution: {:?}",
        tiles.iter().filter(|v| v == &&&Tile::Tree).count()
    );
}
