use std::time::Instant;

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

fn parse_input(include_str: &str) -> () {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_part_01() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(day_02::part_01(&parse_input(data)), 514579)
    }

    #[test]
    fn test_data_part_02() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(day_02::part_02(&parse_input(data)), 241861950)
    }
}
