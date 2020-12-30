use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<usize> {
    get_seat_ids(input.lines().collect::<Vec<&str>>())
}

#[aoc(day5, part1)]
pub fn part_01(input: &Vec<usize>) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part_02(input: &Vec<usize>) -> usize {
    2
}

fn get_seat_ids(seats: Vec<&str>) -> Vec<usize> {
    seats
        .iter()
        .map(|line| get_seat_id(line))
        .collect::<Vec<usize>>()
}

fn get_seat_id(line: &str) -> usize {
    let binary = line
        .to_string()
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    usize::from_str_radix(&binary, 2).unwrap()
}
