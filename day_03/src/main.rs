use std::time::Instant;

fn main() {
    println!("Results for Day 3");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let part_01 = count_trees(&INPUT, 3, 1);

    println!("Part 1: {:?} ({:.2?})", part_01, now.elapsed());

    // Part 2
    let now = Instant::now();
    let part_02: usize = SLOPES
        .iter()
        .map(|slopes| count_trees(&INPUT, slopes[0], slopes[1]))
        .product();

    println!("Part 2: {:?} ({:.2?})", part_02, now.elapsed());
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
const INPUT: &str = include_str!("../input");
