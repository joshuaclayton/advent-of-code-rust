use std::collections::HashSet;

use itertools::Itertools;

pub fn solve() {
    let input = include_str!("input-day6");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> usize {
    let mut final_position = 0;

    for (idx, (first, second, third, fourth)) in input.chars().tuple_windows().enumerate() {
        let result: HashSet<char> = vec![first, second, third, fourth].into_iter().collect();

        if result.len() == 4 {
            final_position = idx + 4;
            break;
        }
    }

    final_position
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        assert_eq!(super::run("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(super::run("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(super::run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(super::run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
