use std::collections::HashMap;

fn split_string_in_half(s: &str) -> (&str, &str) {
    let len = s.len();
    let mid = len / 2;
    (&s[..mid], &s[mid..])
}

inventory::submit! {
    crate::Solution { year: 2024, day: 11, part: 2, run: run }
}

fn run() {
    const INPUT: &str = include_str!("input.txt");
    const BLINKS: u128 = 75;
    let mut stones: HashMap<u128, u128> = HashMap::new();

    for stone in INPUT
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
    {
        *stones.entry(stone).or_insert(0) += 1;
    }

    for _ in 1..=BLINKS {
        let mut new_stones: HashMap<u128, u128> = HashMap::new();
        for (stone, count) in stones {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
                continue;
            }
            let stone_string = stone.to_string();
            let chars = stone_string.chars();
            let chars_count = chars.count();
            if chars_count % 2 == 0 {
                let (left, right) = split_string_in_half(stone_string.as_str());
                let left = left.parse::<u128>().unwrap();
                let right = right.parse::<u128>().unwrap();
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }
        stones = new_stones;
    }
    println!("Part 2: {}", stones.values().sum::<u128>());
}
