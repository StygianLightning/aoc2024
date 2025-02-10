use aoc2024::{grid::Grid, index2::uidx2};

#[derive(Debug)]
struct Lock {
    lengths: Vec<u32>,
}

#[derive(Debug)]
struct Key {
    lengths: Vec<u32>,
}

fn parse_locks_and_keys(text: &str) -> (Vec<Lock>, Vec<Key>, u32) {
    let mut locks = vec![];
    let mut keys = vec![];

    let mut current_lines: Vec<&str> = vec![];
    let mut max_len = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            if current_lines.is_empty() {
                continue;
            }

            max_len = current_lines.len() as u32;
            let is_lock = current_lines.first().unwrap().starts_with("#");
            let lengths = extract_lens(&current_lines);
            if is_lock {
                locks.push(Lock { lengths });
            } else {
                keys.push(Key { lengths });
            }
            current_lines.clear();
            continue;
        }

        current_lines.push(line);
    }

    (locks, keys, max_len)
}

fn extract_lens(lines: &[&str]) -> Vec<u32> {
    // convert current lines to lock or key
    let first_line = lines.first().unwrap();
    let is_lock = first_line.starts_with("#");
    let mut grid = Grid::new_with_provider(uidx2(first_line.len() as _, lines.len() as _), || '.');
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.char_indices() {
            let idx = uidx2(x as _, y as _);
            grid[idx] = c;
        }
    }

    let mut lengths = vec![];

    let rows_in_order = if is_lock {
        (0..lines.len()).collect::<Vec<_>>()
    } else {
        (0..lines.len()).rev().collect::<Vec<_>>()
    };

    for column in 0..first_line.len() {
        let mut len = 0;
        for row in rows_in_order.iter() {
            let idx = uidx2(column as u32, *row as u32);
            if grid[idx] == '.' {
                break;
            } else {
                len += 1;
            }
        }
        lengths.push(len - 1); // base level not counted in examples
    }

    lengths
}

fn part1(locks: &[Lock], keys: &[Key], max_len: u32) -> u32 {
    let mut ret = 0;
    for lock in locks {
        for key in keys {
            if check_fits(lock, key, max_len) {
                ret += 1;
            }
        }
    }

    ret
}

fn check_fits(lock: &Lock, key: &Key, max_len: u32) -> bool {
    for column in 0..lock.lengths.len() {
        let a = lock.lengths[column];
        let b = key.lengths[column];
        if a + b + 2 > max_len {
            return false;
        }
    }

    true
}

fn main() {
    let input = std::fs::read_to_string("input/day25.txt").unwrap();
    let (locks, keys, max_len) = parse_locks_and_keys(&input);
    println!("locks: {locks:#?}");
    println!("keys: {keys:#?}");
    println!("max len: {max_len}");

    let part1_res = part1(&locks, &keys, max_len);
    println!("part 1 result: {part1_res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_extract() {
        let input = vec!["###", "##.", "#..", "..."];
        let lengths = extract_lens(&input);
        assert_eq!(lengths, vec![2, 1, 0]);
    }
}
