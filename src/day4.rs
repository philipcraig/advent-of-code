use aoc_runner_derive::aoc;

use std::ops::RangeInclusive;

use regex::Regex;

#[aoc(day4, part1)]
pub fn part_01(input: &str) -> usize {
    let passports: Vec<_> = input.split("\n\n").collect();
    passports
        .iter()
        .map(|&passport| has_required_fields(passport) as usize)
        .sum()
}

#[aoc(day4, part2)]
pub fn part_02(input: &str) -> usize {
    let passports: Vec<_> = input.split("\n\n").collect();

    passports
        .iter()
        .filter(|&passport| has_required_fields(passport))
        .map(|&passport| has_valid_fields(passport) as usize)
        .sum()
}

fn has_required_fields(passport: &str) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|&required| passport.contains(required))
}

fn has_valid_fields(passport: &str) -> bool {
    let fields_separator = Regex::new(r"\s\n|\s|\n").unwrap();

    let fields: Vec<&str> = fields_separator
        .split(passport)
        .filter(|field| !field.is_empty())
        .collect();

    fields.iter().all(|field| is_valid_field(field))
}

fn is_valid_field(field: &str) -> bool {
    let field_extractor = Regex::new(r":").unwrap();
    let split_field: Vec<&str> = field_extractor.split(field).collect();
    let (key, value) = (split_field[0], split_field[1]);

    match key {
        "byr" => is_in_range(value, 1920..=2002),
        "iyr" => is_in_range(value, 2010..=2020),
        "eyr" => is_in_range(value, 2020..=2030),
        "hgt" => {
            is_in_range_suffix(value, 150..=193, "cm") || is_in_range_suffix(value, 59..=76, "in")
        }
        "hcl" => value
            .strip_prefix("#")
            .map_or(false, |color| color.chars().all(|c| c.is_ascii_hexdigit())),
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => false,
    }
}

fn is_in_range(value: &str, range: RangeInclusive<usize>) -> bool {
    value
        .parse()
        .map_or(false, |num: usize| range.contains(&num))
}

fn is_in_range_suffix(value: &str, range: RangeInclusive<usize>, suffix: &str) -> bool {
    value
        .strip_suffix(suffix)
        .map_or(false, |num| is_in_range(num, range))
}

const REQUIRED_FIELDS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
