use nom::{
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    IResult,
};

pub fn solve() {
    let input = include_str!("input-day8");
    println!("Answer: {:?}", run(input));
}

fn run(input: &str) -> Option<usize> {
    let (_, matrix) = parse_matrix(input.trim()).ok()?;
    let pass_1 = matrix
        .clone()
        .into_iter()
        .map(|row| determine_visibility(&row))
        .collect::<Vec<_>>();
    let pass_2 = transpose(matrix.clone())
        .into_iter()
        .map(|row| determine_visibility(&row))
        .collect::<Vec<_>>();

    let mut count = 0;

    for (first, second) in pass_1.into_iter().zip(transpose(pass_2)) {
        count += first
            .into_iter()
            .zip(second)
            .filter(|&(l, r)| l || r)
            .collect::<Vec<_>>()
            .len();
    }

    Some(count)
}

fn determine_visibility(row: &[usize]) -> Vec<bool> {
    let mut ltr: Vec<bool> = vec![];
    let mut rtl: Vec<bool> = vec![];

    let mut highest = None;

    for v in row {
        match (highest, v) {
            (Some(h), v) if h >= v => {
                ltr.push(false);
            }
            (_, v) => {
                highest = Some(v);
                ltr.push(true);
            }
        }
    }

    highest = None;

    for v in row.iter().rev() {
        match (highest, v) {
            (Some(h), v) if h >= v => {
                rtl.insert(0, false);
            }
            (_, v) => {
                highest = Some(v);
                rtl.insert(0, true);
            }
        }
    }

    ltr.iter().zip(rtl).map(|(l, r)| *l || r).collect()
}

fn transpose<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    (0..len)
        .into_iter()
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn parse_row(input: &str) -> IResult<&str, Vec<usize>> {
    many1(map(satisfy(|c: char| c.is_numeric()), |v| {
        v.to_digit(10).unwrap() as usize
    }))(input)
}

fn parse_matrix(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    all_consuming(separated_list1(tag("\n"), parse_row))(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
30373
25512
65332
33549
35390
        "#;
        assert_eq!(super::run(input.trim()), Some(21))
    }

    #[test]
    fn visibility_in_middle() {
        assert_eq!(
            super::determine_visibility(&[2, 5, 5, 1, 2]),
            vec![true, true, true, false, true]
        );

        assert_eq!(
            super::determine_visibility(&[2, 5, 7, 5, 3, 1, 2]),
            vec![true, true, true, true, true, false, true]
        );
        assert_eq!(
            super::determine_visibility(&[6, 5, 3, 3, 2]),
            vec![true, true, false, true, true]
        );
    }
}
