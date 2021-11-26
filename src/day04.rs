#![allow(dead_code)]

use indoc::indoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm

    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929

    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm

    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in
"};

type Passport = HashMap<String, String>;

fn parse(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .into_iter()
        .map(|record| {
            let mut p = Passport::new();
            for field in record.split_whitespace() {
                let (k, v) = field.split_once(":").unwrap();
                p.insert(k.to_string(), v.to_string());
            }
            p
        })
        .collect()
}

fn load_input(filename: &str) -> Vec<Passport> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

fn between(min: u16, max: u16, value: &str) -> bool {
    let v = value.parse().unwrap();
    min <= v && v <= max
}

fn validate_has_fields(pass: &Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] // "cid" is ignored
        .into_iter()
        .all(|required_field| pass.contains_key(required_field))
}

fn validate_field_contents(pass: &Passport) -> bool {
    lazy_static! {
        static ref HEIGHT: regex::Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
        static ref HAIR_COLOR: regex::Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref EYE_COLOR: regex::Regex =
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID: regex::Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    validate_has_fields(&pass)
        && pass.into_iter().all(|(key, value)| match key.as_str() {
            "byr" => between(1920, 2002, value),
            "iyr" => between(2010, 2020, value),
            "eyr" => between(2020, 2030, value),
            "hgt" => match HEIGHT.captures(value) {
                Some(g) => match &g[2] {
                    "cm" => between(150, 193, &g[1]),
                    "in" => between(59, 76, &g[1]),
                    _ => false,
                },
                None => false,
            },
            "hcl" => HAIR_COLOR.is_match(value),
            "ecl" => EYE_COLOR.is_match(value),
            "pid" => PID.is_match(value),
            "cid" => true,
            _ => false,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let passports = parse(EXAMPLE);
        let valids = passports.into_iter().filter(validate_has_fields);
        assert_eq!(2, valids.count());
    }

    #[test]
    fn example_2() {
        let passports = load_input("input/example04.txt");
        let valids = passports.into_iter().filter(validate_field_contents);
        assert_eq!(4, valids.count());
    }

    #[test]
    fn part_1() {
        let passports = load_input("input/input04.txt");
        let valids = passports.into_iter().filter(validate_has_fields);
        assert_eq!(192, valids.count());
    }

    #[test]
    fn part_2() {
        let passports = load_input("input/input04.txt");
        let valids = passports.into_iter().filter(validate_field_contents);
        assert_eq!(101, valids.count());
    }
}
