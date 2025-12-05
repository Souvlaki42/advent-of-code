inventory::submit! {
    crate::Solution { year: 2025, day: 4, part: 2, run: run }
}

fn run() {
    let contents = include_str!("example.txt");
    let mut total_joltage = 0;
    for line in contents.lines() {
        let mut bank: [u32; 15] = [0; 15];
        let mut index = 0;
        let joltages: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        for i in 0..12 {
            for j in index + 1..joltages.len() - i {
                if joltages[j] > bank[i] {
                    bank[i] = joltages[j];
                    index = i;
                }
            }
        }
        println!("Bank: {:?}", bank);
        let bank_max: u64 = bank
            .iter()
            .map(|j| j.to_string())
            .collect::<Vec<String>>()
            .concat()
            .parse()
            .unwrap();
        total_joltage += bank_max;
    }
    println!("Total joltage: {}", total_joltage);
}
