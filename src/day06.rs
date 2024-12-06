use Direction::{East, South, West};
use crate::day06::Direction::North;
use crate::day06::GuardPath::{Exit, Looping};
use crate::day06::MapItem::{Empty, Guard, Obstacle, Visited};

#[derive(PartialEq, Clone)]
enum MapItem {
    Empty,
    Obstacle,
    Guard,
    Visited,
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq)]
enum GuardPath {
    Exit,
    Looping,
}

fn load_map(data: &str) -> Vec<Vec<MapItem>> {
    let mut map = vec![];
    for line in data.lines() {
        map.push(line.chars().map(|c| match c {
            '.' => Empty,
            '#' => Obstacle,
            '^' => Guard,
            other => panic!("Unexpected map item: {}", other),
        }).collect());
    }
    map
}

fn locate_guard(map: &Vec<Vec<MapItem>>) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Guard {
                return Some((x, y));
            }
        }
    }
    None
}

fn guard_looping(turns: &Vec<(usize, usize)>) -> bool {
    if turns.len() > 3 {
        let ultimate = turns.len() - 1;
        let penultimate = turns.len() - 2;
        for i in 0..penultimate {
            if turns[i] == turns[penultimate] && turns[i + 1] == turns[ultimate] {
                return true;
            }
        }
    }
    false
}

fn travel_map(map: &mut Vec<Vec<MapItem>>) -> GuardPath {
    let (mut guard_x, mut guard_y) = match locate_guard(&map) {
        None => panic!("No guard!"),
        Some(coords) => coords,
    };
    let mut direction = North;
    let mut turns = vec![];

    loop {
        map[guard_y][guard_x] = Visited;
        match direction {
            North => {
                if guard_y == 0 {
                    return Exit;
                } else if map[guard_y - 1][guard_x] == Obstacle {
                    turns.push((guard_x, guard_y));
                    direction = East;
                } else {
                    guard_y -= 1;
                }
            }
            East => {
                if guard_x == (map[0].len() - 1) {
                    return Exit;
                } else if map[guard_y][guard_x + 1] == Obstacle {
                    turns.push((guard_x, guard_y));
                    direction = South;
                } else {
                    guard_x += 1;
                }
            }
            South => {
                if guard_y == (map.len() - 1) {
                    return Exit;
                } else if map[guard_y + 1][guard_x] == Obstacle {
                    turns.push((guard_x, guard_y));
                    direction = West;
                } else {
                    guard_y += 1;
                }
            }
            West => {
                if guard_x == 0 {
                    return Exit;
                } else if map[guard_y][guard_x - 1] == Obstacle {
                    turns.push((guard_x, guard_y));
                    direction = North;
                } else {
                    guard_x -= 1;
                }
            }
        }
        if guard_looping(&turns) {
            return Looping;
        }
    }
}

fn count_visited(map: &Vec<Vec<MapItem>>) -> usize {
    map.iter().map(|row| row.iter().filter(|&pos| *pos == Visited).count()).sum()
}

pub fn count_path(data: &str) -> usize {
    let mut map = load_map(data);
    travel_map(&mut map);
    count_visited(&map)
}

pub fn count_guard_loops(data: &str) -> usize {
    let mut map = load_map(data);
    let clean_map = map.clone();
    travel_map(&mut map);
    let mut loops = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Visited && clean_map[y][x] != Guard {
                let mut trial = clean_map.clone();
                trial[y][x] = Obstacle;
                if travel_map(&mut trial) == Looping {
                    loops += 1;
                }
            }
        }
    }
    loops
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn example_1() {
        assert_eq!(count_path(EXAMPLE), 41);
    }

    #[test]
    fn example_2() {
        assert_eq!(count_guard_loops(EXAMPLE), 6);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", count_path(include_str!("../res/day06.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", count_guard_loops(include_str!("../res/day06.txt")));
    }
}
