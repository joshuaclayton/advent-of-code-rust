const TARGET: usize = 2020;

pub fn solve() {
    let input = include_str!("input-day1");
    let numbers: Vec<usize> = input
        .lines()
        .map(|v| v.parse())
        .filter_map(|v| v.ok())
        .collect::<Vec<_>>();

    let (higher, lower): (Vec<usize>, Vec<usize>) =
        numbers.iter().partition(|&v| v >= &(TARGET / 2));

    let result = higher.iter().find(|&v| lower.contains(&(TARGET - v)));

    if let Some(found) = result {
        println!("Solution: {:?}", found * (TARGET - found));
    }
}
