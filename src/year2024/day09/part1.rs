inventory::submit! {
    crate::Solution { year: 2024, day: 9, part: 1, run: run }
}

fn run() {
    const INPUT: &str = include_str!("input.txt");
    let mut blocks: Vec<String> = Vec::new();
    let mut id = 0;
    for (i, char) in INPUT.chars().enumerate() {
        let num = char.to_digit(10).unwrap();
        if i == 0 || i % 2 == 0 {
            for _ in 0..num {
                blocks.push(id.to_string());
            }
            id += 1;
        } else {
            for _ in 0..num {
                blocks.push(".".to_string());
            }
        }
    }
    while blocks.contains(&".".to_string()) {
        let first_dot = blocks.iter().position(|x| x == ".").unwrap();
        blocks.swap_remove(first_dot);
    }
    let mut sum = 0;
    for (i, block) in blocks.iter().enumerate() {
        let num = block.parse::<u128>().unwrap();
        sum += (i as u128) * num;
    }
    println!("Part 1: {}", sum);
}
