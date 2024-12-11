use std::collections::HashMap;

fn load_data(data: &str) -> Vec<u64> {
    data.split_whitespace().map(|number| number.parse().unwrap()).collect()
}

fn count_digits(number: u64) -> u32 {
    number.ilog10() + 1
}

fn count_stone(stone: u64, iterations: usize, seen: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(count) = seen.get(&(stone, iterations)) {
        *count
    } else {
        let count = if iterations == 0 {
            1
        } else if stone == 0 {
            count_stone(1, iterations - 1, seen)
        } else {
            let digits = count_digits(stone);
            if digits % 2 == 0 {
                let scale = 10u64.pow(digits / 2);
                let front = stone / scale;
                let back = stone - (front * scale);
                count_stone(front, iterations - 1, seen) + count_stone(back, iterations - 1, seen)
            } else {
                count_stone(stone * 2024, iterations - 1, seen)
            }
        };
        seen.insert((stone, iterations), count);
        count
    }
}

pub fn count_stones(data: &str, iterations: usize) -> usize {
    let mut seen = HashMap::new();
    load_data(data).into_iter().map(|stone| count_stone(stone, iterations, &mut seen)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"125 17";

    #[test]
    fn example_1() {
        assert_eq!(count_stones(EXAMPLE, 25), 55312);
    }

    #[test]
    fn example_2() {
        assert_eq!(count_stones(EXAMPLE, 75), 65601038650482);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", count_stones(include_str!("../res/day11.txt"), 25));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", count_stones(include_str!("../res/day11.txt"), 75));
    }
}
