struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    p_x: i64,
    p_y: i64,
}

fn load_data(data: &str, prize_offset: i64) -> Vec<Machine> {
    let mut machines = vec![];
    for machine in data.split("\n\n") {
        let mut lines = machine.splitn(3, "\n");
        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        machines.push(Machine {
            a_x: button_a["Button A: X+".len()..button_a.find(",").unwrap()].parse().unwrap(),
            a_y: button_a[button_a.rfind("+").unwrap()+1..].parse().unwrap(),
            b_x: button_b["Button B: X+".len()..button_b.find(",").unwrap()].parse().unwrap(),
            b_y: button_b[button_b.rfind("+").unwrap()+1..].parse().unwrap(),
            p_x: prize["Prize: X=".len()..prize.find(",").unwrap()].parse::<i64>().unwrap() + prize_offset,
            p_y: prize[prize.rfind("=").unwrap()+1..].parse::<i64>().unwrap() + prize_offset,
        });
    }
    machines
}

pub fn min_spend_for_most_prizes(data: &str, prize_offset: i64) -> i64 {
    let machines = load_data(data, prize_offset);
    let mut wins = vec![];
    for machine in machines {
        let b = (machine.a_x * machine.p_y - machine.a_y * machine.p_x) / (machine.a_x * machine.b_y - machine.a_y * machine.b_x);
        let a = (machine.p_x - machine.b_x * b) / machine.a_x;
        if machine.a_x * a + machine.b_x * b == machine.p_x {
            if machine.a_y * a + machine.b_y * b == machine.p_y {
                wins.push(a * 3 + b);
            }
        }
    }
    wins.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOCATION_OFFSET: i64 = 10000000000000;
    const EXAMPLE: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn example_1() {
        assert_eq!(min_spend_for_most_prizes(EXAMPLE, 0), 480);
    }

    #[test]
    fn example_2() {
        assert_eq!(min_spend_for_most_prizes(EXAMPLE, LOCATION_OFFSET), 875318608908);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", min_spend_for_most_prizes(include_str!("../res/day13.txt"), 0));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", min_spend_for_most_prizes(include_str!("../res/day13.txt"), LOCATION_OFFSET));
    }
}
