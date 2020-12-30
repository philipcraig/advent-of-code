use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part_01(input: &str) -> usize {
    count_trees(&input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part_02(input: &str) -> usize {
    SLOPES
        .iter()
        .map(|slopes| count_trees(&input, slopes[0], slopes[1]))
        .product()
}

fn count_trees(trees: &str, x_slope: usize, y_slope: usize) -> usize {
    trees
        .lines()
        .step_by(y_slope)
        .enumerate()
        .filter(|(step, line)| line.chars().nth(step * x_slope % line.len()).unwrap() == '#')
        .count()
}

const SLOPES: [[usize; 2]; 5] = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
