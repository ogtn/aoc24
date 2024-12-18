use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    let mut a_x = 0;
    let mut a_y = 0;
    let mut b_x = 0;
    let mut b_y = 0;

    for line in input.lines() {
        if line.starts_with("Button A:") {
            (a_x, a_y) = parse_button(line);
        } else if line.starts_with("Button B:") {
            (b_x, b_y) = parse_button(line);
        } else if line.starts_with("Prize") {
            let (x, y) = parse_prize(line);

            for i in 0..=100 {
                for j in 0..=100 {
                    let res_x = a_x * i + b_x * j;
                    let res_y = a_y * i + b_y * j;

                    if res_x == x && res_y == y {
                        sum += i * 3 + j;
                    }
                }
            }
        }
    }

    sum
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> u32 {
    0
}

fn parse_button(line: &str) -> (u32, u32) {
    let x = line.find("X+").unwrap() + 2;
    let mid = line.find(',').unwrap();
    let y = line.find("Y+").unwrap() + 2;

    let x = line[x..mid].parse().unwrap();
    let y = line[y..].parse().unwrap();

    (x, y)
}

fn parse_prize(line: &str) -> (u32, u32) {
    let x = line.find("X=").unwrap() + 2;
    let mid = line.find(',').unwrap();
    let y = line.find("Y=").unwrap() + 2;

    let x = line[x..mid].parse().unwrap();
    let y = line[y..].parse().unwrap();

    (x, y)
}
