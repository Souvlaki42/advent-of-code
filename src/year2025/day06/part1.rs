use std::collections::HashMap;

inventory::submit! {
    crate::Solution { year: 2025, day: 6, part: 1, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let mut worksheet: HashMap<usize, (Vec<u128>, char)> = HashMap::new();
    let mut result: u128 = 0;

    for line in contents.lines() {
        let parts = line.split_whitespace();
        for (i, part) in parts.enumerate() {
            let problem = worksheet.entry(i).or_insert((Vec::new(), '0'));
            if part == "+" || part == "*" {
                (*problem).1 = part.chars().next().unwrap();
            } else {
                (*problem).0.push(part.parse().unwrap());
            }
        }
    }

    for (values, action) in worksheet.values() {
        if action == &'+' {
            result += values.iter().sum::<u128>();
        } else {
            result += values.iter().product::<u128>();
        }
    }

    println!("Result: {}", result);
}
