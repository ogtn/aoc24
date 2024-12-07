use std::sync::atomic::Ordering::Relaxed;
use std::{
    ops::{Add, AddAssign},
    sync::atomic::AtomicU32,
};

use aoc_runner_derive::aoc;
use rayon::prelude::*;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let data = Data::new(input);

    let mut map = data.map;
    data.count_visited(&mut map)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let data = Data::new(input);
    let mut visited_map = data.map;
    data.mark_visited(&mut visited_map);
    let result = AtomicU32::new(0);

    visited_map.par_iter().enumerate().for_each(|(idx, val)| {
        if *val != b'#' && *val != b'.' {
            let mut map = data.map;
            map[idx] = b'#';

            if data.is_cycle(&mut map) {
                result.fetch_add(1, Relaxed);
            }
        }
    });

    result.load(Relaxed)
}

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl AddAssign<Self> for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add<Self> for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
}

impl Direction {
    fn next(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn movement(&self) -> Position {
        match self {
            Direction::North => Position { x: 0, y: -1 },
            Direction::East => Position { x: 1, y: 0 },
            Direction::South => Position { x: 0, y: 1 },
            Direction::West => Position { x: -1, y: 0 },
        }
    }

    fn as_byte(&self) -> u8 {
        *self as u8
    }
}

const USIZE: usize = 130;
const ISIZE: i32 = USIZE as i32;

#[derive(Debug)]
struct Data {
    map: [u8; USIZE * USIZE],
    start: Position,
}

impl Data {
    fn new(input: &str) -> Self {
        let mut map = [0; USIZE * USIZE];
        let mut start = Position::default();
        let mut found = false;

        for (n, line) in input.lines().enumerate() {
            let row = n * USIZE..(n + 1) * USIZE;
            map[row].clone_from_slice(line.as_bytes());

            if !found {
                if let Some(x) = line.find('^') {
                    start.x = x as i32;
                    start.y = n as i32;
                    found = true;
                }
            }
        }

        Self { map, start }
    }

    fn mark_visited(&self, map: &mut [u8; USIZE * USIZE]) {
        let mut pos = self.start;
        let mut direction = Direction::North;
        let mut movement = direction.movement();

        let idx = (pos.y * ISIZE + pos.x) as usize;
        map[idx] = direction.as_byte();

        loop {
            let next = pos + movement;
            let idx = (next.y * ISIZE + next.x) as usize;

            if next.x < 0 || next.x >= ISIZE || next.y < 0 || next.y >= ISIZE {
                return;
            }

            match map[idx] {
                b'#' => {
                    direction.next();
                    movement = direction.movement();
                }
                b'.' => {
                    map[idx] = direction.as_byte();
                    pos = next;
                }
                _ => {
                    pos = next;
                }
            }
        }
    }

    fn count_visited(&self, map: &mut [u8; USIZE * USIZE]) -> u32 {
        let mut pos = self.start;
        let mut direction = Direction::North;
        let mut movement = direction.movement();

        let idx = (pos.y * ISIZE + pos.x) as usize;
        map[idx] = direction.as_byte();
        let mut visited = 1;

        loop {
            let next = pos + movement;
            let idx = (next.y * ISIZE + next.x) as usize;

            if next.x < 0 || next.x >= ISIZE || next.y < 0 || next.y >= ISIZE {
                return visited;
            }

            match map[idx] {
                b'#' => {
                    direction.next();
                    movement = direction.movement();
                }
                b'.' => {
                    map[idx] = direction.as_byte();
                    visited += 1;
                    pos = next;
                }
                _ => {
                    pos = next;
                }
            }
        }
    }

    fn is_cycle(&self, map: &mut [u8; USIZE * USIZE]) -> bool {
        let mut pos = self.start;
        let mut direction = Direction::North;
        let mut movement = direction.movement();

        let idx = (pos.y * ISIZE + pos.x) as usize;
        map[idx] = direction.as_byte();

        loop {
            let next = pos + movement;
            let idx = (next.y * ISIZE + next.x) as usize;

            if next.x < 0 || next.x >= ISIZE || next.y < 0 || next.y >= ISIZE {
                return false;
            }

            match map[idx] {
                b'#' => {
                    direction.next();
                    movement = direction.movement();
                }
                b'.' => {
                    map[idx] = direction.as_byte();
                    pos = next;
                }
                val if val & direction.as_byte() == direction.as_byte() => {
                    return true;
                }
                _ => {
                    map[idx] |= direction.as_byte();
                    pos = next;
                }
            }
        }
    }
}
