use std::collections::BTreeMap;

use crate::parser::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn solve() {
    let input = include_str!("input-day11");
    println!("Answer: {:?}", run(input));
}

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

fn run(input: &str) -> Option<usize> {
    let (_, monkey_list) =
        all_consuming(separated_list1(many1(tag("\n")), parse_monkey))(input.trim()).ok()?;

    let mut monkeys = BTreeMap::new();

    let keys = monkey_list.iter().map(|m| m.id).collect::<Vec<_>>();

    for monkey in monkey_list.into_iter() {
        monkeys.insert(monkey.id, monkey);
    }

    let base: usize = monkeys.values().map(|m| m.test.divisible_by).product();

    for _ in times(10000) {
        for id in &keys {
            let mut throw_at: Vec<(usize, Item)> = vec![];
            let monkey = monkeys.get_mut(&id).unwrap();

            for item in monkey.items.drain(0..) {
                monkey.inspection_count += 1;
                let new_item = Item(monkey.operation.run(item.0) % base);

                throw_at.push(monkey.test.throw(new_item));
            }

            for (id, item) in throw_at {
                monkeys.get_mut(&id).unwrap().items.push(item);
            }
        }
    }

    let (v1, v2) = if let [v1, v2] = monkeys
        .values()
        .sorted_by_key(|m| m.inspection_count)
        .map(|m| m.inspection_count)
        .rev()
        .take(2)
        .collect::<Vec<_>>()[..]
    {
        (v1, v2)
    } else {
        todo!()
    };

    Some(v1 * v2)
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<Item>,
    operation: Operation,
    test: Throw,
    inspection_count: usize,
}

#[derive(Debug)]
struct Item(usize);

#[derive(Debug)]
struct Throw {
    divisible_by: usize,
    when_true: usize,
    when_false: usize,
}

impl Throw {
    fn throw(&self, item: Item) -> (usize, Item) {
        if item.0 % self.divisible_by == 0 {
            (self.when_true, item)
        } else {
            (self.when_false, item)
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Actual(usize),
    Old,
}

impl Value {
    fn resolve(&self, old_value: usize) -> usize {
        match self {
            Value::Actual(v) => *v,
            Value::Old => old_value,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Value, Value),
    Multiply(Value, Value),
}

impl Operation {
    fn run(&self, old_value: usize) -> usize {
        match self {
            Operation::Add(l, r) => l.resolve(old_value) + r.resolve(old_value),
            Operation::Multiply(l, r) => l.resolve(old_value) * r.resolve(old_value),
        }
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = terminated(parse_monkey_id, tag("\n"))(input)?;
    let (input, items) = terminated(parse_starting_items, tag("\n"))(input)?;
    let (input, operation) = terminated(parse_operation, tag("\n"))(input)?;
    let (input, test) = parse_throw(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            operation,
            test,
            inspection_count: 0,
        },
    ))
}

fn parse_monkey_id(input: &str) -> IResult<&str, usize> {
    preceded(tag("Monkey "), terminated(parse_usize, tag(":")))(input)
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<Item>> {
    preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), map(parse_usize, Item)),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    preceded(
        tag("  Operation: new = "),
        alt((
            map(
                separated_pair(parse_value, tag(" + "), parse_value),
                |(l, r)| Operation::Add(l, r),
            ),
            map(
                separated_pair(parse_value, tag(" * "), parse_value),
                |(l, r)| Operation::Multiply(l, r),
            ),
        )),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Old, tag("old")),
        map(parse_usize, Value::Actual),
    ))(input)
}

fn parse_throw(input: &str) -> IResult<&str, Throw> {
    let (input, divisible_by) = preceded(
        tag("  Test: divisible by "),
        terminated(parse_usize, tag("\n")),
    )(input)?;
    let (input, when_true) = preceded(
        tag("    If true: throw to monkey "),
        terminated(parse_usize, tag("\n")),
    )(input)?;
    let (input, when_false) = preceded(tag("    If false: throw to monkey "), parse_usize)(input)?;

    Ok((
        input,
        Throw {
            divisible_by,
            when_true,
            when_false,
        },
    ))
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;
        assert_eq!(super::run(input), Some(2713310158))
    }
}
