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

fn step((plus_x, plus_y): (usize, usize)) -> Box<dyn Fn(Coordinates) -> Coordinates> {
    Box::new(move |(x, y)| (x + plus_x, y + plus_y))
}

pub fn solve() {
    let input = include_str!("input-day3");
    let (_, all) = separated_list1(tag("\n"), parse_line)(&input).unwrap();
    let width = all[0].len();
    let mut counts = vec![];

    for modifier in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let mut tiles = vec![];
        let mut start = (0, 0);
        while start.1 < all.len() - 1 {
            start = step(*modifier)(start);
            tiles.push(&all[start.1][start.0.rem_euclid(width)]);
        }
        counts.push(tiles.iter().filter(|v| v == &&&Tile::Tree).count());
    }

    println!("Solution: {:?}", counts.iter().fold(1, |acc, v| acc * v));
}
