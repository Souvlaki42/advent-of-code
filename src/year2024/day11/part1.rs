fn split_string_in_half(s: &str) -> (&str, &str) {
    let len = s.len();
    let mid = len / 2;
    (&s[..mid], &s[mid..])
}

inventory::submit! {
    crate::Solution { year: 2024, day: 11, part: 1, run: run }
}

fn run() {
    const INPUT: &str = include_str!("input.txt");
    const BLINKS: u128 = 25;
    let mut stones = INPUT
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    for _ in 1..=BLINKS {
        let mut new_stones: Vec<u128> = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }
            let stone_string = stone.to_string();
            let chars = stone_string.chars();
            let chars_count = chars.count();
            if chars_count % 2 == 0 {
                let (left, right) = split_string_in_half(stone_string.as_str());
                let left = left.parse::<u128>().unwrap();
                let right = right.parse::<u128>().unwrap();
                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }
    println!("Part 1: {}", stones.len());
}
