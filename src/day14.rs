struct Robot {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64,
}

fn load_data(data: &str) -> Vec<Robot> {
    let mut robots = vec![];
    for line in data.lines() {
        let (pos_x, pos_y) = line[2..line.find(" ").unwrap()].split_once(",").unwrap();
        let pos_x = pos_x.parse::<i64>().unwrap();
        let pos_y = pos_y.parse::<i64>().unwrap();
        let (vel_x, vel_y) = line[(line.find("v=").unwrap() + 2)..].split_once(",").unwrap();
        let vel_x = vel_x.parse::<i64>().unwrap();
        let vel_y = vel_y.parse::<i64>().unwrap();
        robots.push(Robot { pos_x, pos_y, vel_x, vel_y });
    }
    robots
}

fn move_robots(robots: &mut Vec<Robot>, len_x: i64, len_y: i64) {
    for robot in robots {
        robot.pos_x += robot.vel_x;
        if robot.pos_x < 0 {
            robot.pos_x += len_x;
        } else if robot.pos_x >= len_x {
            robot.pos_x -= len_x;
        }
        robot.pos_y += robot.vel_y;
        if robot.pos_y < 0 {
            robot.pos_y += len_y;
        } else if robot.pos_y >= len_y {
            robot.pos_y -= len_y;
        }
    }
}

fn robots_in_area(robots: &Vec<Robot>, start_x: i64, end_x: i64, start_y: i64, end_y: i64) -> usize {
    robots.iter().filter(|robot| robot.pos_x >= start_x && robot.pos_x <= end_x && robot.pos_y >= start_y && robot.pos_y <= end_y).count()
}

fn plot_map(robots: &Vec<Robot>, len_x: i64, len_y: i64) -> Vec<Vec<usize>> {
    let mut map = vec![];
    for y in 0..(len_y as usize) {
        map.push(vec![]);
        for _ in 0..(len_x as usize) {
            map[y].push(0);
        }
    }
    for robot in robots {
        map[robot.pos_y as usize][robot.pos_x as usize] += 1;
    }
    map
}

fn print_map(map: &Vec<Vec<usize>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                print!(".", );
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!();
    }
}

pub fn safety_factor(data: &str, len_x: i64, len_y: i64, seconds: usize) -> usize {
    let mut robots = load_data(data);
    for _ in 0..seconds {
        move_robots(&mut robots, len_x, len_y);
    }
    robots_in_area(&robots, 0, len_x / 2 - 1, 0, len_y / 2 - 1)
        * robots_in_area(&robots, len_x / 2 + 1, len_x - 1, 0, len_y / 2 - 1)
        * robots_in_area(&robots, 0, len_x / 2 - 1, len_y / 2 + 1, len_y - 1)
        * robots_in_area(&robots, len_x / 2 + 1, len_x - 1, len_y / 2 + 1, len_y - 1)
}

pub fn find_picture(data: &str, len_x: i64, len_y: i64) -> usize {
    let mut robots = load_data(data);
    let mut seconds = 0;
    loop {
        seconds += 1;
        move_robots(&mut robots, len_x, len_y);
        let map = plot_map(&robots, len_x, len_y);
        for y in 0..(len_y as usize) {
            let mut in_a_row = 0;
            for x in 0..(len_x as usize) {
                if map[y][x] != 0 {
                    in_a_row += 1;
                } else {
                    in_a_row = 0;
                }
                if in_a_row > 20 {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    print_map(&map);
                    println!("{} seconds elapsed", seconds);
                    return seconds;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP_X: i64 = 101;
    const MAP_Y: i64 = 103;
    const EXAMPLE_X: i64 = 11;
    const EXAMPLE_Y: i64 = 7;
    const EXAMPLE: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn example_1() {
        assert_eq!(safety_factor(EXAMPLE, EXAMPLE_X, EXAMPLE_Y, 100), 12);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", safety_factor(include_str!("../res/day14.txt"), MAP_X, MAP_Y, 100));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", find_picture(include_str!("../res/day14.txt"), MAP_X, MAP_Y));
    }
}
