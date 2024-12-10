use crate::parser::*;
use nom::{bytes::complete::tag, multi::separated_list1};
use num_bigint::BigUint;

fn previous_scores_to_sum(results: &[Option<usize>]) -> usize {
    if results.len() == 3 {
        let val = (results[0], results[1], results[2]);
        match val {
            (None, Some(_), None) => 1,
            (Some(_), Some(_), _) => 1,
            (Some(_), None, _) => 2,
            (None, Some(a), Some(b)) => {
                if b - a == 1 {
                    3
                } else {
                    1
                }
            }
            (None, None, Some(_)) => 3,
            _ => panic!("failed to handle case"),
        }
    } else {
        if results[0].is_none() {
            3
        } else if results[1].is_none() {
            2
        } else {
            1
        }
    }
}

pub fn solve() {
    let input = include_str!("input-day10");

    let (_, mut adapter_list) = separated_list1(tag("\n"), parse_usize)(input).unwrap();
    adapter_list.push(0);
    adapter_list.sort();
    let mut results: Vec<usize> = vec![0];
    // (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
    // (0), 1, 4,       7, 10,     12, 15, 16, 19, (22)
    //
    // 0       3, 4, 7,       10, 11, 14, 17,         20,  23,       25,  28,  31,          34,   35, 38, 39, 42, 45,           48,   49 (20)
    // 0 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20,  23,  24,  25,  28,  31, 32, 33,  34,   35, 38, 39, 42, 45, 46, 47,   48,   49 (31)
    // 1 1  2  4  7  7  7  14 28  49  49  49  49  98  196 196  196  392  392  392 392 784 1568  2744                      5488 10976 19208

    shortest_solution(0, &mut results, &adapter_list);

    println!("{:?}", results);
    println!("{:?}", adapter_list);
    let mut scores: Vec<BigUint> = vec![];
    let mut window = vec![];

    for v in adapter_list {
        if results.contains(&v) {
            window.push(Some(v));
        } else {
            window.push(None);
        }
        window = window.into_iter().rev().take(3).rev().collect::<Vec<_>>();

        if scores.is_empty() {
            scores.push(num_traits::One::one());
        } else {
            let current_score = scores
                .iter()
                .rev()
                .take(previous_scores_to_sum(&window))
                .sum();

            scores.push(current_score)
        }
        scores = scores.into_iter().rev().take(4).rev().collect::<Vec<_>>();
        println!("{} {:?}", v, scores);
    }

    println!("Solution: {:?}", scores.iter().last());
}

fn shortest_solution(starting: usize, results: &mut Vec<usize>, full_list: &[usize]) {
    let four_available = full_list.contains(&(starting + 3));
    let three_available = full_list.contains(&(starting + 2));
    let two_available = full_list.contains(&(starting + 1));

    if four_available {
        results.push(starting + 3);
        shortest_solution(starting + 3, results, full_list);
    } else if three_available {
        results.push(starting + 2);
        shortest_solution(starting + 2, results, full_list);
    } else if two_available {
        results.push(starting + 1);
        shortest_solution(starting + 1, results, full_list);
    }
}
