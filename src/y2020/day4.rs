use crate::parser::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::{take_till, take_while_m_n},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug)]
enum BirthYear<'a> {
    Valid(usize),
    Invalid(&'a str),
}

#[derive(Debug)]
enum IssueYear<'a> {
    Valid(usize),
    Invalid(&'a str),
}

#[derive(Debug)]
enum ExpirationYear<'a> {
    Valid(usize),
    Invalid(&'a str),
}

#[derive(Debug)]
enum Height<'a> {
    Cm(usize),
    In(usize),
    Invalid(&'a str),
}

#[derive(Debug)]
enum HairColor<'a> {
    Valid(&'a str),
    Invalid(&'a str),
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
    Invalid,
}

#[derive(Debug)]
enum PassportId<'a> {
    Valid(&'a str),
    Invalid(&'a str),
}

#[derive(Debug)]
enum CredentialAttribute<'a> {
    BirthYear(BirthYear<'a>),
    IssueYear(IssueYear<'a>),
    ExpirationYear(ExpirationYear<'a>),
    Height(Height<'a>),
    HairColor(HairColor<'a>),
    EyeColor(EyeColor),
    PassportId(PassportId<'a>),
    CountryId(&'a str),
}

impl<'a> CredentialAttribute<'a> {
    fn is_valid(&self) -> bool {
        match self {
            CredentialAttribute::BirthYear(BirthYear::Invalid(_)) => false,
            CredentialAttribute::IssueYear(IssueYear::Invalid(_)) => false,
            CredentialAttribute::ExpirationYear(ExpirationYear::Invalid(_)) => false,
            CredentialAttribute::Height(Height::Invalid(_)) => false,
            CredentialAttribute::HairColor(HairColor::Invalid(_)) => false,
            CredentialAttribute::EyeColor(EyeColor::Invalid) => false,
            CredentialAttribute::PassportId(PassportId::Invalid(_)) => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
struct CredentialAttributes<'a>(Vec<CredentialAttribute<'a>>);

impl<'a> CredentialAttributes<'a> {
    fn is_valid(&self) -> bool {
        self.0
            .iter()
            .filter(|v| match v {
                CredentialAttribute::CountryId(_) => false,
                _ => v.is_valid(),
            })
            .count()
            == 7
    }
}

fn parse_birth_year(input: &str) -> IResult<&str, BirthYear> {
    let (input, _) = tag("byr:")(input)?;

    alt((
        map(parse_usize_in_range(1920..=2002), BirthYear::Valid),
        map(to_end_of_tag, BirthYear::Invalid),
    ))(input)
}

fn parse_issue_year(input: &str) -> IResult<&str, IssueYear> {
    let (input, _) = tag("iyr:")(input)?;

    alt((
        map(parse_usize_in_range(2010..=2020), IssueYear::Valid),
        map(to_end_of_tag, IssueYear::Invalid),
    ))(input)
}

fn parse_expiration_year(input: &str) -> IResult<&str, ExpirationYear> {
    let (input, _) = tag("eyr:")(input)?;

    alt((
        map(parse_usize_in_range(2020..=2030), ExpirationYear::Valid),
        map(to_end_of_tag, ExpirationYear::Invalid),
    ))(input)
}

fn parse_height(input: &str) -> IResult<&str, Height> {
    let (input, _) = tag("hgt:")(input)?;

    alt((
        map(
            terminated(parse_usize_in_range(59..=76), tag("in")),
            Height::In,
        ),
        map(
            terminated(parse_usize_in_range(150..=193), tag("cm")),
            Height::Cm,
        ),
        map(to_end_of_tag, Height::Invalid),
    ))(input)
}

fn parse_hair_color(input: &str) -> IResult<&str, HairColor> {
    let (input, _) = tag("hcl:")(input)?;

    alt((
        map(
            preceded(
                tag("#"),
                take_while_m_n(6, 6, |v: char| v.is_ascii_hexdigit()),
            ),
            HairColor::Valid,
        ),
        map(to_end_of_tag, HairColor::Invalid),
    ))(input)
}

fn parse_eye_color(input: &str) -> IResult<&str, EyeColor> {
    let (input, _) = tag("ecl:")(input)?;

    alt((
        map(tag("amb"), |_| EyeColor::Amber),
        map(tag("blu"), |_| EyeColor::Blue),
        map(tag("brn"), |_| EyeColor::Brown),
        map(tag("gry"), |_| EyeColor::Gray),
        map(tag("grn"), |_| EyeColor::Green),
        map(tag("hzl"), |_| EyeColor::Hazel),
        map(tag("oth"), |_| EyeColor::Other),
        map(to_end_of_tag, |_| EyeColor::Invalid),
    ))(input)
}

fn parse_passport_id(input: &str) -> IResult<&str, PassportId> {
    let (input, _) = tag("pid:")(input)?;

    alt((
        map_res(to_end_of_tag, |v: &str| {
            if v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()) {
                Ok(PassportId::Valid(v))
            } else {
                Err("invalid length")
            }
        }),
        map(to_end_of_tag, PassportId::Invalid),
    ))(input)
}

fn to_end_of_tag(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == ' ' || c == '\n')(input)
}

fn parse_credential_attribute<'a>(input: &'a str) -> IResult<&'a str, CredentialAttribute<'a>> {
    alt((
        map(parse_birth_year, CredentialAttribute::BirthYear),
        map(parse_issue_year, CredentialAttribute::IssueYear),
        map(parse_expiration_year, CredentialAttribute::ExpirationYear),
        map(parse_height, CredentialAttribute::Height),
        map(parse_hair_color, CredentialAttribute::HairColor),
        map(parse_eye_color, CredentialAttribute::EyeColor),
        map(parse_passport_id, CredentialAttribute::PassportId),
        map(
            preceded(tag("cid:"), to_end_of_tag),
            CredentialAttribute::CountryId,
        ),
    ))(input)
}

fn parse_credentials<'a>(input: &'a str) -> IResult<&'a str, CredentialAttributes<'a>> {
    let (input, credentials_attributes) =
        separated_list1(alt((tag(" "), tag("\n"))), parse_credential_attribute)(input)?;

    Ok((input, CredentialAttributes(credentials_attributes)))
}

pub fn solve() {
    let input = include_str!("input-day4");
    let (_, all) = separated_list1(tag("\n\n"), parse_credentials)(&input).unwrap();
    println!(
        "Solution: {:?}",
        all.iter().filter(|v| v.is_valid()).count()
    );
}
