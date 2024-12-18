use aoc_runner_derive::aoc;

const SIZE: usize = 71;
const BYTES_COUNT: usize = 1024;

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u16 {
    let mut map = [false; SIZE * SIZE];

    for line in input.lines().take(BYTES_COUNT) {
        let mut it = line.split(',');
        let x: usize = it.next().unwrap().parse().unwrap();
        let y: usize = it.next().unwrap().parse().unwrap();

        map[y * SIZE + x] = true;
    }

    find_shortest_iter(&map)
}

pub fn find_shortest_rec(
    x: usize,
    y: usize,
    time: u32,
    map: &[bool; SIZE * SIZE],
    visited: &mut [u32; SIZE * SIZE],
) -> u32 {
    if !(0..SIZE).contains(&x) || !(0..SIZE).contains(&y) {
        return u32::MAX;
    } else if x == SIZE - 1 && y == SIZE - 1 {
        return time;
    }

    let idx = y * SIZE + x;

    if visited[idx] != 0 && visited[idx] <= time {
        return u32::MAX;
    }

    if map[idx] {
        return u32::MAX;
    }

    visited[idx] = time;

    [
        find_shortest_rec(x + 1, y, time + 1, map, visited),
        find_shortest_rec(x - 1, y, time + 1, map, visited),
        find_shortest_rec(x, y + 1, time + 1, map, visited),
        find_shortest_rec(x, y - 1, time + 1, map, visited),
    ]
    .into_iter()
    .min()
    .unwrap()
}

struct State {
    x: usize,
    y: usize,
    time: u16,
}

pub fn find_shortest_iter(map: &[bool; SIZE * SIZE]) -> u16 {
    let mut work = Vec::new();
    let mut visited = [0; SIZE * SIZE];
    let mut shortest = u16::MAX;

    work.push(State {
        x: 0,
        y: 0,
        time: 0,
    });

    while let Some(state) = work.pop() {
        if !(0..SIZE).contains(&state.x) || !(0..SIZE).contains(&state.y) {
            continue;
        } else if state.x == SIZE - 1 && state.y == SIZE - 1 {
            shortest = shortest.min(state.time);
            continue;
        }

        let idx = state.y * SIZE + state.x;

        if visited[idx] != 0 && visited[idx] <= state.time {
            continue;
        }

        if map[idx] {
            continue;
        }

        visited[idx] = state.time;
        work.push(State {
            x: state.x + 1,
            y: state.y,
            time: state.time + 1,
        });
        work.push(State {
            x: state.x - 1,
            y: state.y,
            time: state.time + 1,
        });
        work.push(State {
            x: state.x,
            y: state.y + 1,
            time: state.time + 1,
        });
        work.push(State {
            x: state.x,
            y: state.y - 1,
            time: state.time + 1,
        });
    }

    shortest
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let mut map = [u16::MAX; SIZE * SIZE];
    let mut time_max = 0;

    for (time, line) in input.lines().enumerate() {
        let mut it = line.split(',');
        let x: usize = it.next().unwrap().parse().unwrap();
        let y: usize = it.next().unwrap().parse().unwrap();

        map[y * SIZE + x] = time as u16;
        time_max = time as u16;
    }

    let (mut x, mut y) = (0, 0);
    let mut time_min = BYTES_COUNT as u16;

    while time_max - time_min > 2 {
        let start_time = (time_min + time_max) / 2;

        match find_first(&map, start_time) {
            Some(state) => {
                x = state.x_stop;
                y = state.y_stop;

                time_min = start_time;
            }
            None => {
                time_max = start_time;
            }
        }
    }

    format!("{},{}", x, y)
}

#[derive(Debug)]
struct State2 {
    x: usize,
    y: usize,
    margin: u16,
    x_stop: usize,
    y_stop: usize,
}

fn find_first(map: &[u16; SIZE * SIZE], start_time: u16) -> Option<State2> {
    let mut work = Vec::new();
    let mut visited = [false; SIZE * SIZE];

    work.push(State2 {
        x: 0,
        y: 0,
        margin: u16::MAX,
        x_stop: 0,
        y_stop: 0,
    });

    while let Some(mut state) = work.pop() {
        if !(0..SIZE).contains(&state.x) || !(0..SIZE).contains(&state.y) {
            continue;
        } else if state.x == SIZE - 1 && state.y == SIZE - 1 {
            return Some(state);
        }

        let idx = state.y * SIZE + state.x;

        if visited[idx] {
            continue;
        }

        if map[idx] <= start_time {
            continue;
        }

        let margin = map[idx] - start_time;

        if margin < state.margin {
            state.margin = margin;
            state.x_stop = state.x;
            state.y_stop = state.y;
        }

        visited[idx] = true;

        work.push(State2 {
            x: state.x + 1,
            y: state.y,
            margin: state.margin,
            x_stop: state.x_stop,
            y_stop: state.y_stop,
        });
        work.push(State2 {
            x: state.x - 1,
            y: state.y,
            margin: state.margin,
            x_stop: state.x_stop,
            y_stop: state.y_stop,
        });
        work.push(State2 {
            x: state.x,
            y: state.y + 1,
            margin: state.margin,
            x_stop: state.x_stop,
            y_stop: state.y_stop,
        });
        work.push(State2 {
            x: state.x,
            y: state.y - 1,
            margin: state.margin,
            x_stop: state.x_stop,
            y_stop: state.y_stop,
        });
    }

    None
}
