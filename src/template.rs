#[allow(dead_code, unused_imports)]

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"";

    #[test]
    fn example_1() {
        assert_eq!(EXAMPLE.len(), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(EXAMPLE.len(), 0);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", include_str!("../res/day01.txt").len());
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", include_str!("../res/day01.txt").len());
    }
}
