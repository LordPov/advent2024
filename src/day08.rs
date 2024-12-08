use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coords {
    x: i64,
    y: i64,
}

struct MapSummary {
    x: i64,
    y: i64,
    antennas: HashMap<u8, Vec<Coords>>,
}

fn load_data(data: &str) -> MapSummary {
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut antennas = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, freq) in line.bytes().enumerate() {
            if freq != b'.' {
                antennas.entry(freq).or_insert_with(|| vec![]).push(Coords { x: x as i64, y: y as i64 });
            }
            max_x = x;
        }
        max_y = y;
    }
    MapSummary {
        x: max_x as i64,
        y: max_y as i64,
        antennas,
    }
}

fn find_antinodes(antennas: &MapSummary) -> HashSet<Coords> {
    let mut antinodes = HashSet::new();
    for group in antennas.antennas.values() {
        for i in 0..group.len() {
            for j in 0..group.len() {
                if i != j {
                    let a = &group[i];
                    let b = &group[j];
                    let diff_x = b.x - a.x;
                    let diff_y = b.y - a.y;
                    let candidate = Coords { x: a.x - diff_x, y: a.y - diff_y };
                    if candidate.x >= 0 && candidate.x <= antennas.x && candidate.y >= 0 && candidate.y <= antennas.y {
                        antinodes.insert(candidate);
                    }
                }
            }
        }
    }
    antinodes
}

fn find_antinodes_with_harmonics(antennas: &MapSummary) -> HashSet<Coords> {
    let mut antinodes = HashSet::new();
    for group in antennas.antennas.values() {
        for i in 0..group.len() {
            for j in 0..group.len() {
                if i != j {
                    let a = &group[i];
                    let b = &group[j];
                    let diff_x = b.x - a.x;
                    let diff_y = b.y - a.y;
                    let mut multiplier = 0;
                    loop {
                        multiplier += 1;
                        let candidate = Coords { x: a.x - (diff_x * multiplier), y: a.y - (diff_y * multiplier) };
                        if candidate.x >= 0 && candidate.x <= antennas.x && candidate.y >= 0 && candidate.y <= antennas.y {
                            antinodes.insert(candidate);
                        } else {
                            break;
                        }
                    }
                    let mut multiplier = 0;
                    loop {
                        multiplier -= 1;
                        let candidate = Coords { x: a.x - (diff_x * multiplier), y: a.y - (diff_y * multiplier) };
                        if candidate.x >= 0 && candidate.x <= antennas.x && candidate.y >= 0 && candidate.y <= antennas.y {
                            antinodes.insert(candidate);
                        } else {
                            break;
                        }
                    }
                    antinodes.insert(group[i].clone());
                    antinodes.insert(group[j].clone());
                }
            }
        }
    }
    antinodes
}

pub fn unique_antinode_locations(data: &str) -> usize {
    find_antinodes(&load_data(data)).len()
}

pub fn unique_antinode_locations_with_harmonics(data: &str) -> usize {
    find_antinodes_with_harmonics(&load_data(data)).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn example_1() {
        assert_eq!(unique_antinode_locations(EXAMPLE), 14);
    }

    #[test]
    fn example_2() {
        assert_eq!(unique_antinode_locations_with_harmonics(EXAMPLE), 34);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", unique_antinode_locations(include_str!("../res/day08.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", unique_antinode_locations_with_harmonics(include_str!("../res/day08.txt")));
    }
}
