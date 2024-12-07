struct Calibration {
    total: u64,
    operands: Vec<u64>,
}

fn load_data(data: &str) -> Vec<Calibration> {
    let mut calibrations = vec![];
    for line in data.lines() {
        match line.split_once(": ") {
            None => panic!("Couldn't parse input line: {}", line),
            Some((total, operands)) => {
                calibrations.push(Calibration {
                    total: total.parse().unwrap(),
                    operands: operands.split_whitespace().map(|operand| operand.parse().unwrap()).collect(),
                });
            }
        }
    }
    calibrations
}

fn concat_operands(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()  // I could have determined the scale and done cheaper maths, but fuck it
}

fn solveable(target: u64, current: u64, operands: &Vec<u64>, concat: bool) -> bool {
    if current > target {
        false
    } else if operands.is_empty() {
        current == target
    } else {
        let operand = operands[0];
        let operands = operands[1..].to_owned();
        if solveable(target, current + operand, &operands, concat) {
            true
        } else if solveable(target, current * operand, &operands, concat) {
            true
        } else if concat && solveable(target, concat_operands(current, operand), &operands, true) {
            true
        } else {
            false
        }
    }
}

pub fn total_calibration_result(data: &str) -> u64 {
    load_data(data)
        .iter()
        .filter(|calibration| solveable(calibration.total, 0, &calibration.operands, false))
        .map(|calibration| calibration.total)
        .sum()
}

pub fn total_calibration_with_concat_result(data: &str) -> u64 {
    load_data(data)
        .iter()
        .filter(|calibration| solveable(calibration.total, 0, &calibration.operands, true))
        .map(|calibration| calibration.total)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn example_1() {
        assert_eq!(total_calibration_result(EXAMPLE), 3749);
    }

    #[test]
    fn example_2() {
        assert_eq!(total_calibration_with_concat_result(EXAMPLE), 11387);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", total_calibration_result(include_str!("../res/day07.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", total_calibration_with_concat_result(include_str!("../res/day07.txt")));
    }
}
