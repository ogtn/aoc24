use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut map = [0; 55 * 55];
    let mut size = 1;

    for (line_count, line) in input.lines().enumerate() {
        size = line.len();
        map[line_count * size..(line_count * size + size)].copy_from_slice(line.as_bytes());
    }

    let map = &map[0..size * size];
    let mut paths = 0;

    for (idx, b) in map.iter().enumerate() {
        if *b == b'0' {
            let x = (idx % size) as i8;
            let y = (idx / size) as i8;

            // println!("start @ ({};{})", x, y);
            paths += find_paths(map, size as i8, x, y);
        }
    }

    paths
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    next: u8,
    x: i8,
    y: i8,
}

fn find_paths(map: &[u8], size: i8, x: i8, y: i8) -> usize {
    let mut candidates = Vec::new();
    let mut set = HashSet::new();

    candidates.push(Position { next: b'0', x, y });

    while let Some(candidate) = candidates.pop() {
        // println!("checking {:?}", candidate);

        if candidate.x >= 0 && candidate.x < size && candidate.y >= 0 && candidate.y < size {
            // println!("in bounds");

            let idx = candidate.y as usize * size as usize + candidate.x as usize;

            if map[idx] == candidate.next {
                if candidate.next == b'9' {
                    // println!("found 9: {:?}", candidate);
                    set.insert(candidate);
                } else {
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x + 1,
                        y: candidate.y,
                    });
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x - 1,
                        y: candidate.y,
                    });
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x,
                        y: candidate.y + 1,
                    });
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x,
                        y: candidate.y - 1,
                    });
                }
            } else {
                // println!("value is {}, not {}", map[idx], candidate.next);
            }
        }
    }

    // println!("rating: {}", set.len());

    set.len()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut map = [0; 55 * 55];
    let mut size = 1;

    for (line_count, line) in input.lines().enumerate() {
        size = line.len();
        map[line_count * size..(line_count * size + size)].copy_from_slice(line.as_bytes());
    }

    let map = &map[0..size * size];
    let mut paths = 0;

    for (idx, b) in map.iter().enumerate() {
        if *b == b'0' {
            let x = (idx % size) as i8;
            let y = (idx / size) as i8;

            // println!("start @ ({};{})", x, y);
            paths += find_all_paths(map, size as i8, x, y);
        }
    }

    paths
}

fn find_all_paths(map: &[u8], size: i8, x: i8, y: i8) -> usize {
    let mut candidates = Vec::new();
    let mut count = 0;

    candidates.push(Position { next: b'0', x, y });

    while let Some(candidate) = candidates.pop() {
        // println!("checking {:?}", candidate);

        if candidate.x >= 0 && candidate.x < size && candidate.y >= 0 && candidate.y < size {
            // println!("in bounds");

            let idx = candidate.y as usize * size as usize + candidate.x as usize;

            if map[idx] == candidate.next {
                if candidate.next == b'9' {
                    // println!("found 9: {:?}", candidate);
                    count += 1;
                } else {
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x + 1,
                        y: candidate.y,
                    });
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x - 1,
                        y: candidate.y,
                    });
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x,
                        y: candidate.y + 1,
                    });
                    candidates.push(Position {
                        next: candidate.next + 1,
                        x: candidate.x,
                        y: candidate.y - 1,
                    });
                }
            } else {
                // println!("value is {}, not {}", map[idx], candidate.next);
            }
        }
    }

    // println!("rating: {}", set.len());

    count
}
