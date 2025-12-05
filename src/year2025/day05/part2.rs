use itertools::Itertools;

inventory::submit! {
    crate::Solution { year: 2025, day: 5, part: 2, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut fresh_count: u64 = 0;
    let mut max_id: u64 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            break;
        }
        let (start_str, end_str) = line.split_once("-").unwrap();
        ranges.push((start_str.parse().unwrap(), end_str.parse().unwrap()));
    }

    ranges = ranges.into_iter().sorted().collect();

    for (start, end) in ranges {
        if end >= max_id {
            fresh_count += end - start.max(max_id) + 1;
            max_id = end + 1
        }
    }

    println!("Fresh: {}", fresh_count);
}
