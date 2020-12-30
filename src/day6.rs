use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[aoc_generator(day6)]
pub fn parse_input(buf: &str) -> Vec<String> {
    buf.split("\n\n").map(|s| s.to_string()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).unique().count())
        .sum()
}

#[aoc(day6, part2)]
pub fn part_2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|s| {
            let sets = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();

            let mut inter = sets[0].clone();
            for i in 1..sets.len() {
                inter = intersect(&inter, &sets[i]);
            }

            inter.len()
        })
        .sum()
}

pub fn intersect(a: &String, b: &String) -> String {
    a.chars().filter(|&c| b.contains(c)).collect()
}
