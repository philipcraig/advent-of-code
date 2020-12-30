use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

use std::{error::Error, str::FromStr};

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Line::from_str(l).unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part_1(input: &Vec<Line>) -> usize {
    input.iter().filter(|l| is_valid_part1(*l)).count()
}

#[aoc(day2, part2)]
pub fn part_2(input: &Vec<Line>) -> usize {
    input.iter().filter(|l| is_valid_part2(*l)).count()
}

fn is_valid_part1(l: &Line) -> bool {
    let c = l.password.matches(l.valid).count();

    c >= l.lower && c <= l.upper
}

fn is_valid_part2(l: &Line) -> bool {
    let first = l.valid == l.password.as_bytes()[l.lower - 1] as char;
    let second = l.valid == l.password.as_bytes()[l.upper - 1] as char;

    first ^ second
}

#[derive(Debug)]
pub struct Line {
    lower: usize,
    upper: usize,
    valid: char,
    password: String,
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Line, Box<dyn Error>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                (?P<lower>[0-9]+)
                -
                (?P<upper>[0-9]+)
                \s+
                (?P<valid>[a-z])
                :\s+
                (?P<password>[a-z]+)
            "
            )
            .unwrap();
        }

        let caps = match RE.captures(s) {
            None => return err!("unrecognized line"),
            Some(caps) => caps,
        };
        Ok(Line {
            lower: caps["lower"].parse()?,
            upper: caps["upper"].parse()?,
            valid: caps["valid"].parse()?,
            password: caps["password"].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_part_1() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(part_1(&parse_input(data)), 2)
    }

    #[test]
    fn test_data_part_2() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(part_2(&parse_input(data)), 1)
    }
}
