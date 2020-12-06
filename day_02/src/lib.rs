#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

use std::{error::Error, str::FromStr};

pub fn part_01(input: &Vec<Line>) -> usize {
    input.iter().filter(|l| is_valid_part1(*l)).count()
}

pub fn part_02(input: &Vec<Line>) -> usize {
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
