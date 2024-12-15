const UPPER_TO_LOWER: u8 = 32;

struct Group {
    area: usize,
    edges: usize,
    corners: Vec<(usize, usize)>,
    _plant: u8,
}

fn load_data(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(|line| line.bytes().collect()).collect()
}

fn explore_group(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> Group {
    let y_len = map.len();
    let x_len = map[0].len();
    let plant = map[y][x];
    let counted = plant + UPPER_TO_LOWER;
    map[y][x] = counted;
    let mut area = 1;
    let mut edges = 0;
    let mut corners = vec![];

    // count corners
    let w_same = x != 0 && (map[y][x - 1] == plant || map[y][x - 1] == counted);
    let e_same = x != (x_len - 1) && (map[y][x + 1] == plant || map[y][x + 1] == counted);
    let n_same = y != 0 && (map[y - 1][x] == plant || map[y - 1][x] == counted);
    let s_same = y != (y_len - 1) && (map[y + 1][x] == plant || map[y + 1][x] == counted);
    let nw_same = x != 0 && y != 0 && (map[y - 1][x - 1] == plant || map[y - 1][x - 1] == counted);
    let ne_same = x != (x_len - 1) && y != 0 && (map[y - 1][x + 1] == plant || map[y - 1][x + 1] == counted);
    let sw_same = x != 0 && y != (y_len - 1) && (map[y + 1][x - 1] == plant || map[y + 1][x - 1] == counted);
    let se_same = x != (x_len - 1) && y != (y_len - 1) && (map[y + 1][x + 1] == plant || map[y + 1][x + 1] == counted);

    if !w_same && !n_same {
        corners.push((x, y));
    } else if w_same && n_same && !nw_same {
        corners.push((x, y));
    }
    if !w_same && !s_same {
        corners.push((x, y));
    } else if w_same && s_same && !sw_same {
        corners.push((x, y));
    }
    if !e_same && !n_same {
        corners.push((x, y));
    } else if e_same && n_same && !ne_same {
        corners.push((x, y));
    }
    if !e_same && !s_same {
        corners.push((x, y));
    } else if e_same && s_same && !se_same {
        corners.push((x, y));
    }

    // count edges and explore neighbour plants
    if x == 0 {
        edges += 1;
    } else if map[y][x - 1] == plant {
        let group = explore_group(map, x - 1, y);
        area += group.area;
        edges += group.edges;
        corners.extend(group.corners);
    } else if map[y][x - 1] != counted {
        edges += 1;
    }

    if x == x_len - 1 {
        edges += 1;
    } else if map[y][x + 1] == plant {
        let group = explore_group(map, x + 1, y);
        area += group.area;
        edges += group.edges;
        corners.extend(group.corners);
    } else if map[y][x + 1] != counted {
        edges += 1;
    }

    if y == 0 {
        edges += 1;
    } else if map[y - 1][x] == plant {
        let group = explore_group(map, x, y - 1);
        area += group.area;
        edges += group.edges;
        corners.extend(group.corners);
    } else if map[y - 1][x] != counted {
        edges += 1;
    }

    if y == y_len - 1 {
        edges += 1;
    } else if map[y + 1][x] == plant {
        let group = explore_group(map, x, y + 1);
        area += group.area;
        edges += group.edges;
        corners.extend(group.corners);
    } else if map[y + 1][x] != counted {
        edges += 1;
    }

    Group {
        area,
        edges,
        corners,
        _plant: plant,
    }
}

fn find_groups(map: &mut Vec<Vec<u8>>) -> Vec<Group> {
    let mut groups = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] < b'a' {
                groups.push(explore_group(map, x, y));
            }
        }
    }
    groups
}

pub fn total_fencing_price(data: &str) -> usize {
    find_groups(&mut load_data(data)).into_iter().map(|group| group.area * group.edges).sum()
}

pub fn discounted_fencing_price(data: &str) -> usize {
    let mut map = load_data(data);
    find_groups(&mut map).into_iter().map(|group| group.area * group.corners.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const EXAMPLE_B: &str = r"AAAA
BBCD
BBCC
EEEC";
    const EXAMPLE_C: &str = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    const EXAMPLE_D: &str = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn example_1() {
        assert_eq!(total_fencing_price(EXAMPLE_A), 1930);
    }

    #[test]
    fn example_2() {
        assert_eq!(discounted_fencing_price(EXAMPLE_A), 1206);
        assert_eq!(discounted_fencing_price(EXAMPLE_B), 80);
        assert_eq!(discounted_fencing_price(EXAMPLE_C), 236);
        assert_eq!(discounted_fencing_price(EXAMPLE_D), 368);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", total_fencing_price(include_str!("../res/day12.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", discounted_fencing_price(include_str!("../res/day12.txt")));
    }
}
