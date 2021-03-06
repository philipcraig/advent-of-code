use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> HashSet<u64> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &HashSet<u64>) -> u64 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

#[aoc(day1, part2)]
pub fn part_2(input: &HashSet<u64>) -> u64 {
    for x in input {
        for y in input {
            if x + y > 2020 {
                continue;
            }

            let z = 2020 - x - y;

            if input.contains(&z) {
                return x * y * z;
            }
        }
    }

    0
}
