use rayon::{iter::ParallelIterator, slice::ParallelSlice};

inventory::submit! {
    crate::Solution { year: 2025, day: 2, part: 2, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let data = contents.lines().next().unwrap();
    let ranges = data.split(",");
    let str_tuples = ranges.filter_map(|range| range.split_once("-"));
    let mut invalid_ids = Vec::<u64>::new();

    for tuple in str_tuples {
        let start = tuple.0.parse::<u64>().unwrap();
        let end = tuple.1.parse::<u64>().unwrap();
        for num in start..=end {
            let num_str = num.to_string();
            for i in 1..=num_str.len() / 2 {
                let first_part = num_str.chars().take(i).collect::<String>();
                if num_str
                    .as_bytes()
                    .par_chunks(i)
                    .map(str::from_utf8)
                    .all(|chunk| chunk.unwrap() == first_part)
                {
                    invalid_ids.push(num);
                    break;
                }
            }
        }
    }

    println!("Invalid ID sum: {}", invalid_ids.iter().sum::<u64>());
}
