use std::str::FromStr;
use anyhow::{Context, Result};

#[derive(Debug)]
struct PageOrdering {
    before: u64,
    after: u64,
}

impl FromStr for PageOrdering {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (before, after) = s.split_once("|").with_context(|| format!("couldn't split page ordering rule: {}", s))?;
        Ok(PageOrdering {
            before: before.parse().with_context(|| format!("couldn't parse before: {}", before))?,
            after: after.parse().with_context(|| format!("couldn't parse after: {}", after))?,
        })
    }
}

fn load_data(data: &str) -> Result<(Vec<PageOrdering>, Vec<Vec<u64>>)> {
    let mut gap = false;
    let mut orderings = vec![];
    let mut updates = vec![];
    for line in data.lines() {
        if line.is_empty() {
            gap = true;
        } else if !gap {
            orderings.push(line.parse()?);
        } else {
            updates.push(line.split(",").map(|page| page.parse::<u64>()).collect::<Result<_, _>>()?);
        }
    }
    Ok((orderings, updates))
}

fn update_valid(update: &Vec<u64>, orderings: &Vec<PageOrdering>) -> bool {
    for (i, &update_page) in update.iter().enumerate() {
        for ordering in orderings.iter().filter(|ordering| ordering.before == update_page) {
            if update.iter().take(i).any(|&prior| prior == ordering.after) {
                return false;
            }
        }
    }
    true
}

fn correct_update(mut update: Vec<u64>, orderings: &Vec<PageOrdering>, mut idx: usize) -> Vec<u64> {
    while idx < update.len() {
        match orderings.iter()
            .filter(|ordering| ordering.before == update[idx])
            .filter_map(|ordering| update.iter().take(idx).position(|&page| page == ordering.after))
            .next() {
            None => {
                idx += 1;
            }
            Some(swap) => {
                update.swap(swap, idx);
                idx = swap;
            }
        }
    }
    update
}

pub fn sum_valid_middle_pages(data: &str) -> Result<u64> {
    let (orderings, updates) = load_data(data)?;
    Ok(updates.iter()
        .filter(|update| update_valid(update, &orderings))
        .map(|update| update[update.len() / 2])
        .sum())
}

pub fn sum_corrected_invalid_middle_pages(data: &str) -> Result<u64> {
    let (orderings, updates) = load_data(data)?;
    Ok(updates.into_iter()
        .filter(|update| !update_valid(update, &orderings))
        .map(|update| correct_update(update, &orderings, 1))
        .map(|update| update[update.len() / 2])
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(sum_valid_middle_pages(EXAMPLE)?, 143);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(sum_corrected_invalid_middle_pages(EXAMPLE)?, 123);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", sum_valid_middle_pages(include_str!("../res/day05.txt"))?);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", sum_corrected_invalid_middle_pages(include_str!("../res/day05.txt"))?);
        Ok(())
    }
}
