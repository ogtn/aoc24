use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut cache = HashMap::new();

    input
        .split_ascii_whitespace()
        .flat_map(|string| string.parse())
        .map(|stone: u64| blink_rec(stone, 25, &mut cache))
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut cache = HashMap::new();

    input
        .split_ascii_whitespace()
        .flat_map(|string| string.parse())
        .map(|stone: u64| blink_rec(stone, 75, &mut cache))
        .sum()
}

fn blink_rec(stone: u64, level: i32, cache: &mut HashMap<(u64, i32), u64>) -> u64 {
    if let Some(cached) = cache.get(&(stone, level)) {
        return *cached;
    }

    let result = if level == 0 {
        1
    } else if stone == 0 {
        blink_rec(1, level - 1, cache)
    } else if let Some((left, right)) = digits(stone) {
        blink_rec(left, level - 1, cache) + blink_rec(right, level - 1, cache)
    } else {
        blink_rec(stone * 2024, level - 1, cache)
    };

    cache.insert((stone, level), result);

    result
}

fn digits(stone: u64) -> Option<(u64, u64)> {
    if stone < 10 {
        None
    } else if stone < 100 {
        Some((stone / 10, stone % 10))
    } else if stone < 1_000 {
        None
    } else if stone < 10_000 {
        Some((stone / 100, stone % 100))
    } else if stone < 100_000 {
        None
    } else if stone < 1_000_000 {
        Some((stone / 1_000, stone % 1_000))
    } else if stone < 10_000_000 {
        None
    } else if stone < 100_000_000 {
        Some((stone / 10_000, stone % 10_000))
    } else if stone < 1_000_000_000 {
        None
    } else if stone < 10_000_000_000 {
        Some((stone / 100_000, stone % 100_000))
    } else if stone < 100_000_000_000 {
        None
    } else if stone < 1_000_000_000_000 {
        Some((stone / 1_000_000, stone % 1_000_000))
    } else if stone < 10_000_000_000_000 {
        None
    } else if stone < 100_000_000_000_000 {
        Some((stone / 10_000_000, stone % 10_000_000))
    } else {
        None
    }
}

// fn blink_big(stone: u64, cache: &mut HashMap<(u64, i32), u64>) -> u32 {
//     let mut total = 0;
//     let mut stones = Vec::new();
//     stones.push((stone, 40));
//     let mut cached = 0;
//     let mut uncached = 0;

//     while let Some((stone, remaining_blinks)) = stones.pop() {
//         if cache.contains(&(stone, remaining_blinks)) {
//             cached += 1;
//         } else {
//             uncached += 1;
//             cache.insert((stone, remaining_blinks));
//         }

//         if remaining_blinks == 0 {
//             total += 1;
//         } else if stone == 0 {
//             stones.push((1, remaining_blinks - 1));
//         } else if let Some((left, right)) = digits(stone) {
//             stones.push((left, remaining_blinks - 1));
//             stones.push((right, remaining_blinks - 1));
//         } else {
//             if stone > u64::MAX / 2024 {
//                 println!("extra fuck at blink {}: {}", remaining_blinks, stone);
//             }

//             stones.push((stone * 2024, remaining_blinks - 1));
//         }
//     }

//     println!(
//         "caching: {}/{} = {:.2}%",
//         cached,
//         cached + uncached,
//         cached as f32 / (cached + uncached) as f32 * 100.
//     );

//     total
// }
