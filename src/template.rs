#[allow(dead_code, unused_imports)]
use anyhow::{Context, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", include_str!("../res/day01.txt").len());
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", include_str!("../res/day01.txt").len());
        Ok(())
    }
}
