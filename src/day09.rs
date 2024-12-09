use crate::day09::Element::{File, Gap};

#[derive(Clone)]
enum Element {
    File { id: u64, length: u64 },
    Gap { length: u64 },
}

fn naive_load_data(data: &str) -> Vec<i64> {
    let mut id = 0;
    let mut disk = vec![];
    for (i, value) in data.bytes().map(|byte| byte - b'0').enumerate() {
        if i % 2 == 0 {
            for _ in 0..value {
                disk.push(id);
            }
            id += 1;
        } else {
            for _ in 0..value {
                disk.push(-1);
            }
        }
    }
    disk
}

fn smart_load_data(data: &str) -> Vec<Element> {
    let mut id = 0;
    let mut disk = vec![];
    for (i, length) in data.bytes().map(|byte| byte - b'0').enumerate() {
        if i % 2 == 0 {
            disk.push(File { id, length: length as u64 });
            id += 1;
        } else {
            disk.push(Gap { length: length as u64 });
        }
    }
    disk
}

fn naive_compact_disk(mut disk: Vec<i64>) -> Vec<i64> {
    let mut gap_idx = 0;
    let mut filler_idx = disk.len() - 1;
    loop {
        if disk[gap_idx] == -1 {
            while disk[filler_idx] == -1 {
                filler_idx -= 1;
            }
            if gap_idx >= filler_idx {
                break;
            } else {
                disk.swap(gap_idx, filler_idx);
            }
        } else {
            gap_idx += 1;
        }
    }
    disk
}

fn smart_compact_disk(mut disk: Vec<Element>) -> Vec<Element> {
    let start_id = disk.iter().rev().filter_map(|elem| match elem {
        File { id, length: _ } => Some(*id),
        Gap { .. } => None,
    }).next().unwrap();
    for curr_id in (1..=start_id).rev() {
        for i in 0..disk.len() {
            if let File { id, length: file_length } = disk[i].clone() {
                if id == curr_id {
                    if let Some(gap_idx) = disk.iter().enumerate().take(i).filter_map(|(idx, elem)| match elem {
                        File { .. } => None,
                        Gap { length } => if *length >= file_length { Some(idx) } else { None },
                    }).next() {
                        disk[i] = Gap { length: file_length };
                        disk.insert(gap_idx, File { id, length: file_length });
                        if let Gap { length } = &mut disk[gap_idx + 1] {
                            *length -= file_length;
                        }
                    }
                    break;
                }
            }
        }
    }
    disk
}

fn naive_representation(disk: Vec<Element>) -> Vec<i64> {
    let mut naive = vec![];
    for element in disk {
        match element {
            File { id, length } => {
                for _ in 0..length {
                    naive.push(id as i64);
                }
            }
            Gap { length } => {
                for _ in 0..length {
                    naive.push(-1);
                }
            }
        }
    }
    naive
}

fn checksum(disk: Vec<i64>) -> i64 {
    disk.into_iter().enumerate().filter(|(_, id)| *id >= 0).map(|(i, id)| (i as i64) * id).sum()
}

pub fn checksum_compacted_disk_naive(data: &str) -> i64 {
    checksum(naive_compact_disk(naive_load_data(data)))
}

pub fn checksum_compacted_disk_smart(data: &str) -> i64 {
    checksum(naive_representation(smart_compact_disk(smart_load_data(data))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"2333133121414131402";

    #[test]
    fn example_1() {
        assert_eq!(checksum_compacted_disk_naive(EXAMPLE), 1928);
    }

    #[test]
    fn example_2() {
        assert_eq!(checksum_compacted_disk_smart(EXAMPLE), 2858);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", checksum_compacted_disk_naive(include_str!("../res/day09.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", checksum_compacted_disk_smart(include_str!("../res/day09.txt")));
    }
}
