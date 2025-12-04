inventory::submit! {
    crate::Solution { year: 2025, day: 3, part: 1, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let mut total_joltage = 0;
    for line in contents.lines() {
        let mut first = 0;
        let mut second = 0;
        let mut first_index = 0;
        let joltages: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        for i in 0..joltages.len() - 1 {
            if joltages[i] > first {
                first = joltages[i];
                first_index = i;
            }
        }
        for i in first_index + 1..joltages.len() {
            if joltages[i] > second {
                second = joltages[i];
            }
        }
        let bank_max: u32 = (first.to_string() + second.to_string().as_str())
            .parse()
            .unwrap();
        total_joltage += bank_max;
    }
    println!("Total joltage: {}", total_joltage);
}
