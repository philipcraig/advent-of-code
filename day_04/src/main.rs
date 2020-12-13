use std::{ops::RangeInclusive, time::Instant};

use regex::Regex;

fn main() {
    println!("Results for Day 4");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let passports: Vec<_> = INPUT.split("\n\n").collect();
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let part_01: usize = passports
        .iter()
        .map(|&passport| has_required_fields(passport, &required_fields) as usize)
        .sum();

    println!("Part 1: {:?} ({:.2?})", part_01, now.elapsed());

    // Part 2
    let now = Instant::now();

    let part_02: usize = passports
        .iter()
        .filter(|&passport| has_required_fields(passport, &required_fields))
        .map(|&passport| has_valid_fields(passport) as usize)
        .sum();

    println!("Part 2: {:?} ({:.2?})", part_02, now.elapsed());
}

fn has_required_fields(passport: &str, required_fields: &Vec<&str>) -> bool {
    required_fields
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

const INPUT: &str = include_str!("../input");
