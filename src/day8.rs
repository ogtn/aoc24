use aoc_runner_derive::aoc;
use bitvec::vec::BitVec;

const SIZE: i8 = 50;

#[derive(Debug)]
struct Antenna {
    freq: u8,
    x: i8,
    y: i8,
}

fn parse(input: &str) -> Vec<Antenna> {
    let mut antennas = Vec::with_capacity(512);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            if *c != b'.' {
                antennas.push(Antenna {
                    freq: *c,
                    x: x as i8,
                    y: y as i8,
                });
            }
        }
    }

    antennas
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
    let antennas = parse(input);

    let mut nodes: BitVec = BitVec::repeat(false, 1 << 15);
    let mut sum = 0;

    for (idx, first) in antennas.iter().enumerate() {
        for second in &antennas[idx + 1..] {
            sum += mark_nodes(first, second, &mut nodes);
        }
    }

    sum
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
    let antennas = parse(input);

    let mut nodes: BitVec = BitVec::repeat(false, 1 << 15);
    let mut sum = 0;

    for (idx, first) in antennas.iter().enumerate() {
        for second in &antennas[idx + 1..] {
            sum += mark_harmonics(first, second, &mut nodes);
        }
    }

    sum
}

fn mark_nodes(first: &Antenna, second: &Antenna, nodes: &mut BitVec) -> u32 {
    let mut sum = 0;

    if first.freq == second.freq {
        let dx = first.x - second.x;
        let dy = first.y - second.y;

        let x = first.x + dx;
        let y = first.y + dy;

        if (0..SIZE).contains(&x) && (0..SIZE).contains(&y) {
            sum += set_if_needed(x, y, nodes);
        }

        let x = second.x - dx;
        let y = second.y - dy;

        if (0..SIZE).contains(&x) && (0..SIZE).contains(&y) {
            sum += set_if_needed(x, y, nodes);
        }
    }

    sum
}

fn mark_harmonics(first: &Antenna, second: &Antenna, nodes: &mut BitVec) -> u32 {
    let mut sum = 0;

    if first.freq == second.freq {
        let dx = first.x - second.x;
        let dy = first.y - second.y;

        let mut x = first.x;
        let mut y = first.y;

        while (0..SIZE).contains(&x) && (0..SIZE).contains(&y) {
            sum += set_if_needed(x, y, nodes);

            x += dx;
            y += dy;
        }

        x = first.x - dx;
        y = first.y - dy;

        while (0..SIZE).contains(&x) && (0..SIZE).contains(&y) {
            sum += set_if_needed(x, y, nodes);

            x -= dx;
            y -= dy;
        }
    }

    sum
}

#[inline(always)]
fn set_if_needed(x: i8, y: i8, nodes: &mut BitVec) -> u32 {
    let idx = (x as usize) << 8 | y as usize;

    let val = unsafe { nodes.get_unchecked_mut(idx) };

    if *val {
        val.commit(true);
        1
    } else {
        0
    }
}
