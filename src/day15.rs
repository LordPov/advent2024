use std::fmt::{Display, Formatter, Write};
use MapItem::Carton;
use crate::day15::Instruction::{Down, Left, Right, Up};
use crate::day15::MapItem::{CartonL, CartonR, Nothing, Robot, Wall};

#[derive(Copy, Clone, PartialEq)]
enum MapItem {
    Robot,
    Wall,
    Carton,
    CartonL,
    CartonR,
    Nothing,
}

impl Display for MapItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Robot => f.write_char('@'),
            Wall => f.write_char('#'),
            Carton => f.write_char('O'),
            CartonL => f.write_char('['),
            CartonR => f.write_char(']'),
            Nothing => f.write_char('.'),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Up => f.write_char('^'),
            Down => f.write_char('v'),
            Left => f.write_char('<'),
            Right => f.write_char('>'),
        }
    }
}

fn load_data(data: &str, wide: bool) -> ((usize, usize), Vec<Vec<MapItem>>, Vec<Instruction>) {
    let (map_data, instruction_data) = data.split_once("\n\n").unwrap();
    let mut robot_loc = None;
    let mut map = vec![];
    for (y, line) in map_data.lines().enumerate() {
        let mut row = vec![];
        for (x, b) in line.bytes().enumerate() {
            let item = match b {
                b'#' => Wall,
                b'@' => Robot,
                b'O' => Carton,
                b'.' => Nothing,
                other => panic!("Unknown item in map: {}", other as char),  
            };
            if wide {
                let (item_l, item_r) = match item {
                    Robot => {
                        robot_loc = Some((x * 2, y));
                        (Robot, Nothing) 
                    }
                    Wall => (Wall, Wall),
                    Carton => (CartonL, CartonR),
                    Nothing => (Nothing, Nothing),
                    _ => panic!("Shouldn't be reachable"),
                };
                row.push(item_l);
                row.push(item_r);
            } else {
                if item == Robot {
                    robot_loc = Some((x, y));
                }
                row.push(item);
            }
        }
        map.push(row);
    }
    let instructions = instruction_data.bytes().filter(|&b| b != b'\n').map(|b| match b {
        b'^' => Up,
        b'>' => Right,
        b'v' => Down,
        b'<' => Left,
        other => panic!("Unknown item in instructions: {}", other as char),
    }).collect();
    (robot_loc.unwrap(), map, instructions)
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<MapItem>>) {
    for row in map {
        for item in row {
            print!("{}", item);
        }
        println!();
    }
}

fn follow_instructions(mut robot_x: usize, mut robot_y: usize, map: &mut Vec<Vec<MapItem>>, instructions: Vec<Instruction>) {
    for instruction in instructions.iter() {
        // println!("{}", instruction);
        match instruction {
            Up => if can_move_item(map, robot_x, robot_y, Up) {
                move_item(map, robot_x, robot_y, Up);
                robot_y -= 1;
            }
            Down => if can_move_item(map, robot_x, robot_y, Down) {
                move_item(map, robot_x, robot_y, Down);
                robot_y += 1;
            }
            Left => if can_move_item(map, robot_x, robot_y, Left) {
                move_item(map, robot_x, robot_y, Left);
                robot_x -= 1;
            }
            Right => if can_move_item(map, robot_x, robot_y, Right) {
                move_item(map, robot_x, robot_y, Right);
                robot_x += 1;
            }
        }
        // print_map(map);
    }
}

fn can_move_item(map: &Vec<Vec<MapItem>>, x: usize, y: usize, direction: Instruction) -> bool {
    let (next_x, next_y) = match direction {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    };
    if map[next_y][next_x] == Nothing {
        true
    } else if map[next_y][next_x] == Carton {
        if can_move_item(map, next_x, next_y, direction) {
            true
        } else {
            false
        }
    } else if map[next_y][next_x] == CartonL {
        if can_move_item(map, next_x, next_y, direction) {
            if direction == Up || direction == Down {
                if can_move_item(map, next_x + 1, next_y, direction) {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    } else if map[next_y][next_x] == CartonR {
        if can_move_item(map, next_x, next_y, direction) {
            if direction == Up || direction == Down {
                if can_move_item(map, next_x - 1, next_y, direction) {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn move_item(map: &mut Vec<Vec<MapItem>>, x: usize, y: usize, direction: Instruction) {
    let (next_x, next_y) = match direction {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    };
    if map[next_y][next_x] == Carton {
        move_item(map, next_x, next_y, direction);
    } else if map[next_y][next_x] == CartonL {
        move_item(map, next_x, next_y, direction);
        if direction == Up || direction == Down {
            move_item(map, next_x + 1, next_y, direction);
        }
    } else if map[next_y][next_x] == CartonR {
        move_item(map, next_x, next_y, direction);
        if direction == Up || direction == Down {
            move_item(map, next_x - 1, next_y, direction);
        }
    }
    if map[next_y][next_x] == Nothing {
        map[next_y][next_x] = map[y][x];
        map[y][x] = Nothing;
    }
}

pub fn sum_of_gps_coordinates(data: &str, wide: bool) -> usize {
    let ((robot_x, robot_y), mut map, instructions) = load_data(data, wide);
    // print_map(&map);
    follow_instructions(robot_x, robot_y, &mut map, instructions);
    map.iter().enumerate().map(|(y, row)| { 
        row.iter().enumerate()
            .filter(|(_, &item)| item == Carton || item == CartonL)
            .map(|(x, _)| y * 100 + x)
            .sum::<usize>() 
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const BIG_EXAMPLE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn example_1() {
        assert_eq!(sum_of_gps_coordinates(SMALL_EXAMPLE, false), 2028);
        assert_eq!(sum_of_gps_coordinates(BIG_EXAMPLE, false), 10092);
    }

    #[test]
    fn example_2() {
        assert_eq!(sum_of_gps_coordinates(BIG_EXAMPLE, true), 9021);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", sum_of_gps_coordinates(include_str!("../res/day15.txt"), false));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", sum_of_gps_coordinates(include_str!("../res/day15.txt"), true));
    }
}
