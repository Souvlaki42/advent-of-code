inventory::submit! {
    crate::Solution { year: 2025, day: 3, part: 2, run: run }
}

fn run() {
    let contents = include_str!("example.txt");
    // let mut total_joltage = 0;
    for line in contents.lines() {
        let mut largest: [u64; 12] = [0; 12];
        let mut joltages: Vec<u64> = line
            .chars()
            .map(|c| u64::from(c.to_digit(10).unwrap()))
            .collect();
        let mut max: u64 = 0;
        let mut index: usize;
        for j in 0..12 {
            max = joltages[0];
            index = 0;
            for i in 1..joltages.len() {
                if max < joltages[i] {
                    max = joltages[i];
                    index = i;
                }
            }
            largest[j] = max;
            joltages[index] = 0;
        }
        println!("Largest: {:?}", largest);
    }
    // println!("Total joltage: {}", total_joltage);
}
