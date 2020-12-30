use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<usize> {
    get_seat_ids(input.lines().collect::<Vec<&str>>())
}

#[aoc(day5, part1)]
pub fn part_1(input: &Vec<usize>) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part_2(input: &Vec<usize>) -> usize {
    let mut seat_ids = input.clone();
    seat_ids.sort();
    let missing_id = seat_ids.windows(2).find(|window| !(window[0] + 1 == window[1])).unwrap();
    let missing_id = missing_id[1] - 1;
    missing_id}

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
