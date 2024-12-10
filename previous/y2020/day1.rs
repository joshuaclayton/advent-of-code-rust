use std::collections::HashMap;

const TARGET: usize = 2020;

pub fn solve() {
    let input = include_str!("input-day1");
    let numbers: Vec<usize> = input
        .lines()
        .map(|v| v.parse())
        .filter_map(|v| v.ok())
        .collect::<Vec<_>>();

    let mut triple = HashMap::new();

    let (higher, lower): (Vec<usize>, Vec<usize>) =
        numbers.iter().partition(|&v| v >= &(TARGET / 2));

    for high in &higher {
        for low in &lower {
            triple.insert((high, low), high + low);
        }
    }

    let result = triple
        .iter()
        .find(|(_, &v)| v < TARGET && lower.contains(&(&TARGET - v)));

    if let Some(((&k1, &k2), v)) = result {
        println!("Solution: {:?}", k1 * k2 * (TARGET - v));
    }
}
