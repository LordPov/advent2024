fn load_data(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn e_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, x_len: usize, _y_len: usize) -> u64 {
    if x <= (x_len - 4) {
        if search[y][x] == 'X' && search[y][x + 1] == 'M' && search[y][x + 2] == 'A' && search[y][x + 3] == 'S' {
            return 1;
        }
    }
    0
}

fn w_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, _x_len: usize, _y_len: usize) -> u64 {
    if x >= 3 {
        if search[y][x] == 'X' && search[y][x - 1] == 'M' && search[y][x - 2] == 'A' && search[y][x - 3] == 'S' {
            return 1;
        }
    }
    0
}

fn n_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, _x_len: usize, _y_len: usize) -> u64 {
    if y >= 3 {
        if search[y][x] == 'X' && search[y - 1][x] == 'M' && search[y - 2][x] == 'A' && search[y - 3][x] == 'S' {
            return 1;
        }
    }
    0
}

fn s_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, _x_len: usize, y_len: usize) -> u64 {
    if y <= (y_len - 4) {
        if search[y][x] == 'X' && search[y + 1][x] == 'M' && search[y + 2][x] == 'A' && search[y + 3][x] == 'S' {
            return 1;
        }
    }
    0
}

fn ne_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, x_len: usize, _y_len: usize) -> u64 {
    if x <= (x_len - 4) && y >= 3 {
        if search[y][x] == 'X' && search[y - 1][x + 1] == 'M' && search[y - 2][x + 2] == 'A' && search[y - 3][x + 3] == 'S' {
            return 1;
        }
    }
    0
}

fn se_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, x_len: usize, y_len: usize) -> u64 {
    if x <= (x_len - 4) && y <= (y_len - 4) {
        if search[y][x] == 'X' && search[y + 1][x + 1] == 'M' && search[y + 2][x + 2] == 'A' && search[y + 3][x + 3] == 'S' {
            return 1;
        }
    }
    0
}

fn sw_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, _x_len: usize, y_len: usize) -> u64 {
    if x >= 3 && y <= (y_len - 4) {
        if search[y][x] == 'X' && search[y + 1][x - 1] == 'M' && search[y + 2][x - 2] == 'A' && search[y + 3][x - 3] == 'S' {
            return 1;
        }
    }
    0
}

fn nw_xmas(search: &Vec<Vec<char>>, x: usize, y: usize, _x_len: usize, _y_len: usize) -> u64 {
    if x >= 3 && y >= 3 {
        if search[y][x] == 'X' && search[y - 1][x - 1] == 'M' && search[y - 2][x - 2] == 'A' && search[y - 3][x - 3] == 'S' {
            return 1;
        }
    }
    0
}

pub fn find_xmas(data: &str) -> u64 {
    let search = load_data(data);
    let mut found = 0;
    let y_len = search.len();
    let x_len = search[0].len();
    for y in 0..y_len {
        for x in 0..x_len {
            if search[y][x] == 'X' {
                found += e_xmas(&search, x, y, x_len, y_len);
                found += w_xmas(&search, x, y, x_len, y_len);
                found += n_xmas(&search, x, y, x_len, y_len);
                found += s_xmas(&search, x, y, x_len, y_len);
                found += ne_xmas(&search, x, y, x_len, y_len);
                found += se_xmas(&search, x, y, x_len, y_len);
                found += sw_xmas(&search, x, y, x_len, y_len);
                found += nw_xmas(&search, x, y, x_len, y_len);
            }
        }
    }
    found
}

fn verify_x_mas(search: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    (search[y - 1][x - 1] == 'M' && search[y - 1][x + 1] == 'S' && search[y + 1][x - 1] == 'M' && search[y + 1][x + 1] == 'S') ||
        (search[y - 1][x - 1] == 'M' && search[y - 1][x + 1] == 'M' && search[y + 1][x - 1] == 'S' && search[y + 1][x + 1] == 'S') ||
        (search[y - 1][x - 1] == 'S' && search[y - 1][x + 1] == 'M' && search[y + 1][x - 1] == 'S' && search[y + 1][x + 1] == 'M') ||
        (search[y - 1][x - 1] == 'S' && search[y - 1][x + 1] == 'S' && search[y + 1][x - 1] == 'M' && search[y + 1][x + 1] == 'M')
}

pub fn find_x_mas(data: &str) -> u64 {
    let search = load_data(data);
    let mut found = 0;
    let y_len = search.len();
    let x_len = search[0].len();
    for y in 1..(y_len - 1) {
        for x in 1..(x_len - 1) {
            if search[y][x] == 'A' {
                if verify_x_mas(&search, x, y) {
                    found += 1;
                }
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn example_1() {
        assert_eq!(find_xmas(EXAMPLE), 18);
    }

    #[test]
    fn example_2() {
        assert_eq!(find_x_mas(EXAMPLE), 9);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", find_xmas(include_str!("../res/day04.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", find_x_mas(include_str!("../res/day04.txt")));
    }
}
