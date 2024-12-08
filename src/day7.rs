use aoc_runner_derive::aoc;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn parse(input: &str) -> Vec<Vec<u64>> {
    let mut equations = Vec::with_capacity(1000);

    for line in input.lines() {
        equations.push(line.split(&[':', ' ']).flat_map(|s| s.parse()).collect());
    }

    equations
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    parse(input)
        .par_iter()
        .map(|equation| eval_iter(equation))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    parse(input)
        .par_iter()
        .map(|equation| eval_iter_2(equation))
        .sum()
}

fn eval_iter(equation: &[u64]) -> u64 {
    let expected = unsafe { *equation.get_unchecked(0) };
    let start = unsafe { *equation.get_unchecked(1) };

    let mut fifo = Vec::with_capacity(64);
    fifo.push((2, start));

    while let Some((level, acc)) = fifo.pop() {
        if level == equation.len() && acc == expected {
            return expected;
        } else if acc <= expected && level < equation.len() {
            let n = unsafe { equation.get_unchecked(level) };

            fifo.push((level + 1, acc + n));
            fifo.push((level + 1, acc * n));
        }
    }

    0
}

fn eval_iter_2(equation: &[u64]) -> u64 {
    let expected = unsafe { *equation.get_unchecked(0) };
    let start = unsafe { *equation.get_unchecked(1) };

    let mut fifo = Vec::with_capacity(64);
    fifo.push((2, start));

    while let Some((level, acc)) = fifo.pop() {
        if level == equation.len() && acc == expected {
            return expected;
        } else if acc <= expected && level < equation.len() {
            let n = unsafe { *equation.get_unchecked(level) };

            fifo.push((level + 1, acc + n));
            fifo.push((level + 1, acc * n));
            fifo.push((level + 1, concat(acc, n)));
        }
    }

    0
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    let mut n = 1;
    let mut cpy = rhs;

    while cpy > 0 {
        n *= 10;
        cpy /= 10
    }

    lhs * n + rhs
}
