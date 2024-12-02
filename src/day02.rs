use crate::day02::State::Unsafe;
use anyhow::{Context, Result};
use State::{Decreasing, Increasing};

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Increasing,
    Decreasing,
    Unsafe,
}

fn load_data(data: &str) -> Result<Vec<Vec<i8>>> {
    data.lines().map(|line| line.split_ascii_whitespace().map(|value| value.parse().with_context(|| format!("Couldn't parse {}", value))).collect()).collect()
}

fn report_safe(report: &Vec<i8>) -> bool {
    report.iter().zip(report.iter().skip(1)).map(|(a, b)| b - a).map(|diff| {
        if diff.abs() > 3 {
            Unsafe
        } else if diff < 0 {
            Decreasing
        } else if diff > 0 {
            Increasing
        } else {
            Unsafe
        }
    }).reduce(|a, b| match (a, b) {
        (Increasing, Increasing) => Increasing,
        (Decreasing, Decreasing) => Decreasing,
        _ => Unsafe,
    }).unwrap_or(Unsafe) != Unsafe
}

fn report_safe_dampened(report: &Vec<i8>) -> bool {
    if report_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report = report.clone();
        report.remove(i);
        if report_safe(&report) {
            return true;
        }
    }

    false
}

fn count_pure_safe_reports(data: &str) -> Result<usize> {
    Ok(load_data(data)?.iter().filter(|&report| report_safe(report)).count())
}

fn count_dampened_safe_reports(data: &str) -> Result<usize> {
    Ok(load_data(data)?.iter().filter(|&report| report_safe_dampened(report)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(count_pure_safe_reports(EXAMPLE)?, 2);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(count_dampened_safe_reports(EXAMPLE)?, 4);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", count_pure_safe_reports(include_str!("../res/day02.txt"))?);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", count_dampened_safe_reports(include_str!("../res/day02.txt"))?);
        Ok(())
    }
}
