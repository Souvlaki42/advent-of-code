use regex::Regex;

fn main() {
    let contents = include_str!("../inputs/input.txt");
    let code_regex = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))");
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
    let mut enabled = true;
    for mut instruction in matches {
        if instruction.contains("do()") {
            enabled = true;
            continue;
        } else if instruction.contains("don't()") {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        instruction.retain(|c| !"mul()".contains(c));
        let instruction: Vec<&str> = instruction.split(",").collect();
        let num1: u64 = instruction[0].parse().unwrap();
        let num2: u64 = instruction[1].parse().unwrap();
        let result = num1 * num2;
        sum += result;
    }

    println!("Part 2: {}", sum);
}
