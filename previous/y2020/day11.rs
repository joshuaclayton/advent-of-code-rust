use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Position {
    fn apply_rule(&self, adjacent: &[&Position]) -> Position {
        match self {
            Position::EmptySeat => {
                if adjacent.iter().any(|&v| v == &Position::OccupiedSeat) {
                    Position::EmptySeat
                } else {
                    Position::OccupiedSeat
                }
            }
            Position::OccupiedSeat => {
                if adjacent
                    .iter()
                    .filter(|&&v| v == &Position::OccupiedSeat)
                    .count()
                    >= 4
                {
                    Position::EmptySeat
                } else {
                    Position::OccupiedSeat
                }
            }
            Position::Floor => Position::Floor,
        }
    }
}

fn adjacent<'a>((x, y): (usize, usize), grid: &'a Vec<Vec<Position>>) -> Vec<&'a Position> {
    let mut results = vec![];

    if x > 0 {
        results.push(grid.get(y + 1).and_then(|v| v.get(x - 1)));
        results.push(grid.get(y).and_then(|v| v.get(x - 1)));
    }

    if y > 0 {
        if x > 0 {
            results.push(grid.get(y - 1).and_then(|v| v.get(x - 1)));
        }

        results.push(grid.get(y - 1).and_then(|v| v.get(x)));
        results.push(grid.get(y - 1).and_then(|v| v.get(x + 1)));
    }

    results.extend(vec![
        grid.get(y + 1).and_then(|v| v.get(x)),
        grid.get(y + 1).and_then(|v| v.get(x + 1)),
        grid.get(y).and_then(|v| v.get(x + 1)),
    ]);
    results.into_iter().filter_map(|v| v).collect()
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Position>>> {
    separated_list1(tag("\n"), parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Position>> {
    many1(alt((parse_floor, parse_empty, parse_occupied)))(input)
}

fn parse_floor(input: &str) -> IResult<&str, Position> {
    map(tag("."), |_| Position::Floor)(input)
}

fn parse_empty(input: &str) -> IResult<&str, Position> {
    map(tag("L"), |_| Position::EmptySeat)(input)
}

fn parse_occupied(input: &str) -> IResult<&str, Position> {
    map(tag("#"), |_| Position::OccupiedSeat)(input)
}

fn build_new_grid(old: &Vec<Vec<Position>>) -> Option<Vec<Vec<Position>>> {
    let mut grid = vec![];

    for (y, row) in old.iter().enumerate() {
        let mut new_row: Vec<Position> = vec![];

        for (x, position) in row.iter().enumerate() {
            new_row.push(position.apply_rule(&adjacent((x, y), old)));
        }

        grid.push(new_row);
    }

    if old == &grid {
        None
    } else {
        Some(grid)
    }
}

pub fn solve() {
    let input = include_str!("input-day11");
    let grid = parse_grid(&input).unwrap().1;

    let mut last_grid = Some(grid);

    loop {
        if let Some(last_grid_) = last_grid.as_ref().and_then(|v| build_new_grid(v)) {
            last_grid = Some(last_grid_);
        } else {
            break;
        }
    }

    println!(
        "{:?}",
        last_grid
            .unwrap()
            .iter()
            .flatten()
            .filter(|&v| v == &Position::OccupiedSeat)
            .count()
    );
}
