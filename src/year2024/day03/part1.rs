use regex::Regex;

inventory::submit! {
    crate::Solution { year: 2024, day: 3, part: 1, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let code_regex = Regex::new(r"mul\(\d+,\d+\)");
    if code_regex.is_err() {
        println!("Regex error");
        return;
    }

    let code_regex = code_regex.unwrap();
    let matches: Vec<String> = code_regex
        .find_iter(contents)
        .map(|m| m.as_str().to_string())
        .collect();
    let mut sum = 0;
    for mut instruction in matches {
        instruction.retain(|c| !"mul()".contains(c));
        let instruction: Vec<&str> = instruction.split(",").collect();
        let num1: u64 = instruction[0].parse().unwrap();
        let num2: u64 = instruction[1].parse().unwrap();
        let result = num1 * num2;
        sum += result;
    }

    println!("Part 1: {}", sum);
}
