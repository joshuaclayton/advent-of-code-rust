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
        (self.differences().iter().all(|&diff| diff < 0)
            || self.differences().iter().all(|&diff| diff > 0))
            && self
                .differences()
                .iter()
                .all(|&diff| (1..=3).contains(&diff.abs()))
    }

    fn differences(&self) -> Vec<isize> {
        let mut differences = vec![];

        for val in self.0.windows(2) {
            differences.push((val[1].0 as isize) - (val[0].0 as isize));
        }

        differences
    }
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
    fn test_differences_ascending() {
        let report = Report(vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()]);
        assert_eq!(report.differences(), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_differences_descending() {
        let report = Report(vec![4.into(), 3.into(), 2.into(), 1.into(), 0.into()]);
        assert_eq!(report.differences(), vec![-1, -1, -1, -1]);
    }

    #[test]
    fn test_differences_variable() {
        let report = Report(vec![
            2.into(),
            4.into(),
            3.into(),
            3.into(),
            2.into(),
            1.into(),
            0.into(),
        ]);
        assert_eq!(report.differences(), vec![2, -1, 0, -1, -1, -1]);
    }
}
