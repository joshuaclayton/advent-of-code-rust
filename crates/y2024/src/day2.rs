use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use parser::*;

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("input-day2");
    let (_, reports) = parse_input(input)?;

    println!(
        "Safe report count: {}",
        reports.iter().filter(|r| r.is_safe()).count()
    );

    Ok(())
}

struct Level(usize);
impl From<usize> for Level {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

struct Report(Vec<Level>);

impl Report {
    fn is_safe(&self) -> bool {
        let values: Vec<_> = self.0.iter().map(|l| l.0).collect();

        possibilities(&values)
            .into_iter()
            .any(|groups| is_safe(&groups))
    }
}

fn is_safe(values: &[usize]) -> bool {
    (differences(values).iter().all(|&diff| diff < 0)
        || differences(values).iter().all(|&diff| diff > 0))
        && differences(values)
            .iter()
            .all(|&diff| (1..=3).contains(&diff.abs()))
}

fn differences(values: &[usize]) -> Vec<isize> {
    let mut differences = vec![];

    for val in values.windows(2) {
        differences.push((val[1] as isize) - (val[0] as isize));
    }

    differences
}
fn possibilities(values: &[usize]) -> Vec<Vec<usize>> {
    (0..values.len())
        .map(|i| {
            let mut result = vec![];
            result.extend_from_slice(&values[..i]); // Take elements before `i`
            result.extend_from_slice(&values[i + 1..]); // Take elements after `i`
            result
        })
        .collect()
}
fn parse_input(input: &str) -> IResult<&str, Vec<Report>> {
    all_consuming(terminated(
        separated_list1(tag("\n"), parse_report),
        tag("\n"),
    ))(input)
}

fn parse_report(input: &str) -> IResult<&str, Report> {
    map(separated_list1(tag(" "), parse_level), Report)(input)
}

fn parse_level(input: &str) -> IResult<&str, Level> {
    map(parse_usize, |v| v.into())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_differences_variable() {
        let values = vec![2, 4, 3, 3, 2, 1, 0];
        assert_eq!(differences(&values), vec![2, -1, 0, -1, -1, -1]);
    }

    #[test]
    fn test_possibilities() {
        assert_eq!(
            possibilities(&[1, 2, 3, 4]),
            vec![vec![2, 3, 4], vec![1, 3, 4], vec![1, 2, 4], vec![1, 2, 3],]
        );
    }
}
