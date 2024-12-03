use anyhow::{Context, Result};
use regex::Regex;

pub fn add_all_muls(data: &str) -> Result<u64> {
    let mut sum = 0u64;
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)")?;
    for line in data.lines() {
        for x in re.find_iter(line) {
            let x = x.as_str();
            let (a, b) = x[4..(x.len() - 1)].split_once(",").with_context(|| format!("Couldn't split mul: {}", x))?;
            let a = a.parse::<u64>()?;
            let b = b.parse::<u64>()?;
            sum += a * b;
        }
    }
    Ok(sum)
}

pub fn clean_do_donts(data: &str) -> Result<String> {
    let string = match data.find("don't()") {
        None => data.to_string(),
        Some(start) => {
            let remaining = &data[(start + 7)..];
            match remaining.find("do()") {
                None => data[..start].to_string(),
                Some(end) => format!("{}{}", &data[..start], clean_do_donts(&remaining[(end + 4)..])?),
            }
        }
    };
    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(add_all_muls(EXAMPLE_1)?, 161);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(add_all_muls(&clean_do_donts(EXAMPLE_2)?)?, 48);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", add_all_muls(include_str!("../res/day03.txt"))?);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", add_all_muls(&clean_do_donts(include_str!("../res/day03.txt"))?)?);
        Ok(())
    }
}
