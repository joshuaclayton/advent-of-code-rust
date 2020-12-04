use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct RequiredCredentials {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
}

#[derive(Debug, PartialEq)]
enum Credentials {
    NorthPoleCredentials {
        creds: RequiredCredentials,
    },
    Passport {
        creds: RequiredCredentials,
        country_id: String,
    },
}

fn parse_field<'a>(
    name: &'static str,
) -> Box<dyn Fn(&'a str) -> IResult<&'a str, (&'a str, &'a str)>> {
    Box::new(move |input| {
        let to_break = take_till(|c| c == ' ' || c == '\n');
        let (input, val) = preceded(tag(name), preceded(tag(":"), to_break))(input)?;
        Ok((input, (name, val)))
    })
}

fn parse_passport<'a>(input: &'a str) -> IResult<&str, HashMap<&'a str, &'a str>> {
    let key_value_pair = alt((
        parse_field("byr"),
        parse_field("iyr"),
        parse_field("eyr"),
        parse_field("hgt"),
        parse_field("hcl"),
        parse_field("ecl"),
        parse_field("pid"),
        parse_field("cid"),
    ));
    let (input, pairs) = separated_list1(alt((tag(" "), tag("\n"))), key_value_pair)(input)?;

    Ok((input, pairs.into_iter().collect::<HashMap<_, _>>()))
}

#[derive(Debug, PartialEq)]
enum InvalidBaseCredentials {
    MissingBirthYear,
    MissingIssueYear,
    MissingExpirationYear,
    MissingHeight,
    MissingHairColor,
    MissingEyeColor,
    MissingPassportId,
}

impl TryFrom<&HashMap<&str, &str>> for Credentials {
    type Error = Vec<InvalidBaseCredentials>;

    fn try_from(value: &HashMap<&str, &str>) -> Result<Self, Self::Error> {
        let possible_country_id = value.get("cid");
        let creds: RequiredCredentials = value.try_into()?;

        if let Some(country_id) = possible_country_id {
            Ok(Credentials::Passport {
                creds,
                country_id: country_id.to_string(),
            })
        } else {
            Ok(Credentials::NorthPoleCredentials { creds })
        }
    }
}

impl TryFrom<&HashMap<&str, &str>> for RequiredCredentials {
    type Error = Vec<InvalidBaseCredentials>;

    fn try_from(value: &HashMap<&str, &str>) -> Result<Self, Self::Error> {
        let mut errors = vec![];

        if let None = value.get("byr") {
            errors.push(InvalidBaseCredentials::MissingBirthYear)
        }

        if let None = value.get("iyr") {
            errors.push(InvalidBaseCredentials::MissingIssueYear)
        }

        if let None = value.get("eyr") {
            errors.push(InvalidBaseCredentials::MissingExpirationYear)
        }

        if let None = value.get("hgt") {
            errors.push(InvalidBaseCredentials::MissingHeight)
        }

        if let None = value.get("hcl") {
            errors.push(InvalidBaseCredentials::MissingHairColor)
        }

        if let None = value.get("ecl") {
            errors.push(InvalidBaseCredentials::MissingEyeColor)
        }

        if let None = value.get("pid") {
            errors.push(InvalidBaseCredentials::MissingPassportId)
        }

        if errors.is_empty() {
            Ok(Self {
                birth_year: value.get("byr").unwrap().to_string(),
                issue_year: value.get("iyr").unwrap().to_string(),
                expiration_year: value.get("eyr").unwrap().to_string(),
                height: value.get("hgt").unwrap().to_string(),
                hair_color: value.get("hcl").unwrap().to_string(),
                eye_color: value.get("ecl").unwrap().to_string(),
                passport_id: value.get("pid").unwrap().to_string(),
            })
        } else {
            Err(errors)
        }
    }
}

pub fn solve() {
    let input = include_str!("input-day4");
    let (_, all) = separated_list1(tag("\n\n"), parse_passport)(&input).unwrap();
    let valid_credentials = all.iter().map(|creds| Credentials::try_from(creds));
    println!(
        "Solution: {:?}",
        valid_credentials.filter_map(|v| v.ok()).count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_passport_handles_basic() {
        let input =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
        let (_, result) = parse_passport(input).unwrap();
        assert!(Credentials::try_from(&result).is_ok());
    }

    #[test]
    fn parse_passport_reports_errors() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd";
        let (_, result) = parse_passport(input).unwrap();

        assert_eq!(
            Err(vec![
                InvalidBaseCredentials::MissingBirthYear,
                InvalidBaseCredentials::MissingIssueYear,
                InvalidBaseCredentials::MissingHeight,
            ]),
            Credentials::try_from(&result)
        );
    }
}
