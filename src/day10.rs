use std::collections::HashSet;

fn load_data(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(|line| line.bytes().map(|byte| byte - b'0').collect()).collect()
}

fn find_trail_end(map: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let current = map[y][x];
    if current == 9 {
        return [(x, y)].into();
    }
    let next = current + 1;
    let mut ends = vec![];
    if y > 0 && map[y - 1][x] == next {  // north
        ends.extend(find_trail_end(map, x, y - 1));
    }
    if x < map[0].len() - 1 && map[y][x + 1] == next {  // east
        ends.extend(find_trail_end(map, x + 1, y));
    }
    if y < map.len() - 1 && map[y + 1][x] == next {  // south
        ends.extend(find_trail_end(map, x, y + 1));
    }
    if x > 0 && map[y][x - 1] == next {  // west
        ends.extend(find_trail_end(map, x - 1, y));
    }
    ends
}

fn unique_ends_scoring(paths: Vec<(usize, usize)>) -> usize {
    let unique_ends: HashSet<(usize, usize)> = HashSet::from_iter(paths.into_iter());
    unique_ends.len()
}

fn unique_paths_scoring(paths: Vec<(usize, usize)>) -> usize {
    paths.len()
}

fn score_trailhead<F>(data: &str, scoring_algo: F) -> usize
where
    F: Fn(Vec<(usize, usize)>) -> usize,
{
    let map = load_data(data);
    let mut scores = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                scores += scoring_algo(find_trail_end(&map, x, y));
            }
        }
    }
    scores
}

pub fn sum_of_trailhead_unique_ends(data: &str) -> usize {
    score_trailhead(data, unique_ends_scoring)
}

pub fn sum_of_trailhead_paths(data: &str) -> usize {
    score_trailhead(data, unique_paths_scoring)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn example_1() {
        assert_eq!(sum_of_trailhead_unique_ends(EXAMPLE), 36);
    }

    #[test]
    fn example_2() {
        assert_eq!(sum_of_trailhead_paths(EXAMPLE), 81);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", sum_of_trailhead_unique_ends(include_str!("../res/day10.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", sum_of_trailhead_paths(include_str!("../res/day10.txt")));
    }
}
