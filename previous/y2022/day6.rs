use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("input-day6");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> usize {
    let mut final_position = 0;

    for (idx, full_window) in input.as_bytes().windows(14).enumerate() {
        let result: HashSet<_> = full_window.into_iter().collect();

        if result.len() == 14 {
            final_position = idx + 14;
            break;
        }
    }

    final_position
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        assert_eq!(super::run("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(super::run("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(super::run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(super::run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
