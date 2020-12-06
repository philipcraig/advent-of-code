use std::{str::FromStr, time::Instant};

use day_02::Line;

fn main() {
    let input = parse_input(include_str!("../input"));

    println!("Results for Day 2");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let part_01 = day_02::part_01(&input);

    println!("Part 1: {:?} ({:.2?})", part_01, now.elapsed());

    // Part 2
    let now = Instant::now();
    let part_02 = day_02::part_02(&input);

    println!("Part 2: {:?} ({:.2?})", part_02, now.elapsed());
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Line::from_str(l).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_part_01() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(day_02::part_01(&parse_input(data)), 2)
    }

    #[test]
    fn test_data_part_02() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(day_02::part_02(&parse_input(data)), 1)
    }
}
