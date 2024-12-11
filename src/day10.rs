use aoc_runner_derive::aoc;
use bitvec::vec::BitVec;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut map = [0; 55 * 55];
    let mut size = 1;

    for (line_count, line) in input.lines().enumerate() {
        size = line.len();
        map[line_count * size..(line_count * size + size)].copy_from_slice(line.as_bytes());
    }

    let map = &map[0..size * size];
    let mut paths = 0;
    let mut candidates = Vec::with_capacity(32);

    for (idx, b) in map.iter().enumerate() {
        if *b == b'0' {
            let x = (idx % size) as i8;
            let y = (idx / size) as i8;

            candidates.push(Position { next: b'0', x, y });
            paths += find_paths(&mut candidates, map, size as i8);
        }
    }

    paths
}

#[derive(Debug)]
struct Position {
    next: u8,
    x: i8,
    y: i8,
}

fn find_paths(candidates: &mut Vec<Position>, map: &[u8], size: i8) -> i32 {
    let mut solutions = 0;
    let mut cache: BitVec = BitVec::repeat(false, size as usize * size as usize);

    while let Some(candidate) = candidates.pop() {
        if candidate.x >= 0 && candidate.x < size && candidate.y >= 0 && candidate.y < size {
            let idx = candidate.y as usize * size as usize + candidate.x as usize;

            if map[idx] == candidate.next && !cache[idx] {
                cache.set(idx, true);

                if candidate.next == b'9' {
                    solutions += 1;
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
            }
        }
    }

    solutions
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
    let mut candidates = Vec::with_capacity(32);

    for (idx, b) in map.iter().enumerate() {
        if *b == b'0' {
            let x = (idx % size) as i8;
            let y = (idx / size) as i8;

            candidates.push(Position { next: b'0', x, y });
            paths += find_all_paths(&mut candidates, map, size as i8);
        }
    }

    paths
}

fn find_all_paths(candidates: &mut Vec<Position>, map: &[u8], size: i8) -> usize {
    let mut count = 0;

    while let Some(candidate) = candidates.pop() {
        if candidate.x >= 0 && candidate.x < size && candidate.y >= 0 && candidate.y < size {
            let idx = candidate.y as usize * size as usize + candidate.x as usize;

            if map[idx] == candidate.next {
                if candidate.next == b'9' {
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
            }
        }
    }

    count
}
