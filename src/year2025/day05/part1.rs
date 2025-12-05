use std::collections::HashSet;

inventory::submit! {
    crate::Solution { year: 2025, day: 5, part: 1, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let mut passed_ranges = false;
    let mut available: HashSet<u64> = HashSet::new();
    let mut fresh: HashSet<u64> = HashSet::new();
    let mut ranges: HashSet<(u64, u64)> = HashSet::new();

    for line in contents.lines() {
        if line.is_empty() {
            passed_ranges = true;
            continue;
        }
        if passed_ranges {
            available.insert(line.parse().unwrap());
        } else {
            let range_str = line.split_once("-").unwrap();
            ranges.insert((range_str.0.parse().unwrap(), range_str.1.parse().unwrap()));
        }
    }

    for ingredient in available {
        for range in &ranges {
            if ingredient >= range.0 && ingredient <= range.1 {
                fresh.insert(ingredient);
            }
        }
    }

    println!("Fresh: {}", fresh.len());
}
