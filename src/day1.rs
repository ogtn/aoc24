use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut it = line.split_whitespace();

        let left: u32 = it.next().unwrap().parse().unwrap();
        let right: u32 = it.next().unwrap().parse().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    let (mut left_list, mut right_list) = parse(input);
    left_list.sort();
    right_list.sort();

    for (l, r) in left_list.iter().zip(right_list) {
        sum += l.abs_diff(r);
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut sum = 0;

    let (left_list, right_list) = parse(input);
    let mut right_freq = HashMap::new();

    for right in right_list {
        right_freq.entry(right).and_modify(|f| *f += 1).or_insert(1);
    }

    for n in left_list {
        sum += n * *right_freq.entry(n).or_default();
    }

    sum
}
