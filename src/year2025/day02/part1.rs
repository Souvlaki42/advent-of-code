inventory::submit! {
    crate::Solution { year: 2025, day: 2, part: 1, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let data = contents.lines().next().unwrap();
    let ranges = data.split(",");
    let str_tuples = ranges.filter_map(|range| range.split_once("-"));
    let mut invalid_ids = Vec::new();

    for tuple in str_tuples {
        let start = tuple.0.parse::<u64>().unwrap();
        let end = tuple.1.parse::<u64>().unwrap();
        for num in start..=end {
            let mut num_str = num.to_string();
            let len = num_str.len();
            if len % 2 == 0 {
                let second_half = num_str.split_off(len / 2);
                if num_str == second_half {
                    invalid_ids.push(num);
                }
            }
        }
    }

    println!("Invalid ID sum: {}", invalid_ids.iter().sum::<u64>());
}
