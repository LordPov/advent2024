use anyhow::{Context, Result};

fn load_data(data: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    let mut a = vec![];
    let mut b = vec![];
    for line in data.lines() {
        let mut split = line.split_whitespace();
        a.push(split.next().with_context(|| "Couldn't get first value")?.parse().with_context(|| "Couldn't parse first value")?);
        b.push(split.next().with_context(|| "Couldn't get second value")?.parse().with_context(|| "Couldn't parse second value")?);
    }
    Ok((a, b))
}

pub fn sum_differences(data: &str) -> Result<u64> {
    let (mut a, mut b) = load_data(data)?;
    a.sort();
    b.sort();
    Ok(a.into_iter().zip(b.into_iter()).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn similarity_score(data: &str) -> Result<u64> {
    let (a, b) = load_data(data)?;
    Ok(a.into_iter().map(|a| a * b.iter().filter(|&&b| a == b).count() as u64).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(sum_differences(EXAMPLE)?, 11);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(similarity_score(EXAMPLE)?, 31);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", sum_differences(include_str!("../res/day01.txt"))?);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", similarity_score(include_str!("../res/day01.txt"))?);
        Ok(())
    }
}
