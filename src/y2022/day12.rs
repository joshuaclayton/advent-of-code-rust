use colored::Colorize;
use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::BTreeMap;

pub fn solve() {
    let input = include_str!("input-day12");
    println!("Answer: {:?}", run(input));
}

#[derive(Clone, Copy, Debug)]
struct Point(usize, usize);

impl Position {
    fn from_char(idx: PositionIdx, input: char, point: Point) -> Self {
        match input {
            'S' => Position::Starting(idx, point),
            'E' => Position::Ending(idx, point),
            v => Position::Point(idx, point, v),
        }
    }

    fn edges_to(&self, other: &Position) -> Vec<(PositionIdx, PositionIdx)> {
        let mut results = vec![];

        if self.height() - other.height() > 1 {
            results.push((other.index(), self.index()));
        }

        if other.height() - self.height() > 1 {
            results.push((self.index(), other.index()));
        }

        if (self.height() - other.height()).abs() <= 1 {
            results.push((other.index(), self.index()));
            results.push((self.index(), other.index()));
        }

        results
    }

    fn to_char(&self) -> char {
        match self {
            Position::Starting(_, _) => 'S',
            Position::Ending(_, _) => 'E',
            Position::Point(_, _, v) => *v,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PositionIdx(usize);

#[derive(Clone, Copy, Debug)]
enum Position {
    Starting(PositionIdx, Point),
    Point(PositionIdx, Point, char),
    Ending(PositionIdx, Point),
}

impl Position {
    fn starting(&self) -> bool {
        match self {
            Position::Starting(_, _) => true,
            _ => false,
        }
    }

    fn ending(&self) -> bool {
        match self {
            Position::Ending(_, _) => true,
            _ => false,
        }
    }

    fn height(&self) -> isize {
        match self {
            Position::Starting(_, _) => 0,
            Position::Ending(_, _) => 25,
            Position::Point(_, _, v) => (*v as u8 - 97) as isize,
        }
    }

    fn index(&self) -> PositionIdx {
        match self {
            Position::Starting(v, _) => *v,
            Position::Ending(v, _) => *v,
            Position::Point(v, _, _) => *v,
        }
    }
}

fn horizontal_edges(positions: Vec<Position>) -> Vec<(PositionIdx, PositionIdx)> {
    let mut results = vec![];

    for pair in positions.as_slice().windows(2) {
        results.extend(pair[0].edges_to(&pair[1]));
    }

    results
}

fn vertical_edges(top: Vec<Position>, bottom: Vec<Position>) -> Vec<(PositionIdx, PositionIdx)> {
    if top.is_empty() {
        return vec![];
    }

    let mut results = vec![];

    for (idx, position) in bottom.iter().enumerate() {
        results.extend(position.edges_to(&top[idx]));
    }

    results
}

fn run(input: &str) -> Option<usize> {
    let matrix = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut idx = 0;
    let mut previous_row = vec![];
    let mut result = BTreeMap::new();
    let mut edges: Vec<(PositionIdx, PositionIdx)> = vec![];
    let mut ending = None;
    let mut grid = vec![];

    for (row_idx, row) in matrix.iter().enumerate() {
        let mut current_row = vec![];

        for (col_idx, col) in row.iter().enumerate() {
            let position = Position::from_char(PositionIdx(idx), *col, Point(row_idx, col_idx));
            current_row.push(position);

            if position.ending() {
                ending = Some(position);
            }
            result.insert(idx, position);

            idx += 1;
        }

        grid.push(current_row.clone());
        edges.extend(horizontal_edges(current_row.clone()));
        edges.extend(vertical_edges(previous_row, current_row.clone()));
        previous_row = current_row;
    }

    let e = edges
        .iter()
        .map(|e| (NodeIndex::new(e.0 .0), NodeIndex::new(e.1 .0)))
        .collect::<Vec<_>>();

    let g = Graph::<i32, ()>::from_edges(&e);
    let node_map = dijkstra(&g, NodeIndex::new(ending.unwrap().index().0), None, |_| 1);

    let mut results = node_map
        .iter()
        .map(|(idx, distance)| {
            let pos = result.get(&idx.index()).unwrap();
            (pos.height(), distance, pos)
        })
        .collect::<Vec<_>>();

    results.sort_by_key(|x| (x.0, x.1));

    let lowest = results.iter().map(|v| v.0).min();
    let binding = results
        .iter()
        .filter(|r| r.0 == lowest.unwrap())
        .collect::<Vec<_>>();

    for row in grid {
        for position in row {
            if node_map.contains_key(&NodeIndex::new(position.index().0)) {
                if binding[0].2.index().0 == position.index().0 {
                    print!("{}", format!("{}", position.to_char()).yellow());
                } else {
                    print!("{}", format!("{}", position.to_char()).green());
                }
            } else {
                print!("{}", format!("{}", position.to_char()).red());
            }
        }
        print!("\n");
    }

    node_map
        .get(&NodeIndex::new(binding[0].2.index().0))
        .copied()
}

mod tests {
    use super::*;

    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;
        assert_eq!(run(input), Some(29))
    }

    #[test]
    fn test_position() {
        assert_eq!(
            Position::from_char(PositionIdx(0), 'a', Point(0, 0)).height(),
            0
        );

        assert_eq!(
            Position::from_char(PositionIdx(0), 'z', Point(0, 0)).height(),
            25
        );

        assert_eq!(
            Position::from_char(PositionIdx(0), 'S', Point(0, 0)).height(),
            0
        );
    }
}
