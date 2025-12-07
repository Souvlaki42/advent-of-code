use std::collections::HashMap;

inventory::submit! {
    crate::Solution { year: 2025, day: 6, part: 2, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let mut lines: Vec<&str> = contents.lines().collect();
    let mut normal_numbers: Vec<Vec<String>> = Vec::new();
    let mut worksheet: Vec<(Vec<u128>, char)> = Vec::new();
    let mut total_result: u128 = 0;
    let symbols = lines.pop().unwrap();
    let mut cut_points: HashMap<usize, bool> = HashMap::new();
    let mut operators: Vec<char> = Vec::new();
    for (i, char) in symbols.char_indices() {
        if char == '+' || char == '*' {
            cut_points.insert(i, true);
            operators.push(char);
        }
    }
    for line in lines {
        let mut parts: Vec<String> = Vec::new();
        let mut part = String::new();
        let chars: Vec<char> = line.chars().collect();
        for (i, char) in chars.iter().enumerate() {
            let cut_point = cut_points.get(&(i + 1));
            if cut_point.is_some() {
                parts.push(part);
                part = String::new();
            } else if char == &' ' {
                part += "0";
            } else {
                part += char.to_string().as_str();
            }

            if i + 1 == chars.len() {
                parts.push(part.clone());
            }
        }
        normal_numbers.push(parts);
    }

    let length = normal_numbers.first().unwrap().len();

    for i in 0..length {
        let mut strings = Vec::new();
        for j in 0..normal_numbers.len() {
            strings.push(normal_numbers[j][i].clone());
        }

        let string_chars: Vec<Vec<char>> = strings.iter().map(|s| s.chars().collect()).collect();
        let mut numbers: Vec<u128> = Vec::new();

        for i in 0..strings.first().unwrap().len() {
            let mut str = String::new();
            for j in 0..strings.len() {
                str += string_chars[j][i].to_string().as_str();
            }
            numbers.push(str.replace("0", "").parse().unwrap());
        }

        worksheet.push((numbers, operators[i]));
    }

    for (math, operator) in worksheet {
        if operator == '+' {
            total_result += math.iter().sum::<u128>();
        } else {
            total_result += math.iter().product::<u128>();
        }
    }

    println!("Total result: {}", total_result);
}
