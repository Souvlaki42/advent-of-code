use std::ops::Div;

fn before(rule: (u128, u128), numbers: &[u128]) -> bool {
    let pos_x = numbers.iter().position(|&x| x == rule.0);
    let pos_y = numbers.iter().position(|&x| x == rule.1);
    if pos_x.is_none() || pos_y.is_none() {
        return true;
    }

    let pos_x = pos_x.unwrap();
    let pos_y = pos_y.unwrap();

    pos_x < pos_y
}

fn middle(rules: &[(u128, u128)], numbers: &[u128]) -> u128 {
    for (x, y) in rules {
        if !before((*x, *y), numbers) {
            return 0;
        }
    }
    numbers[numbers.len().div(2)]
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut sum = 0;
    let mut reached_updates = false;
    let mut rules: Vec<(u128, u128)> = Vec::new();
    for line in INPUT.lines() {
        if line.is_empty() {
            reached_updates = true;
            continue;
        }
        if reached_updates {
            let update_str = line.split(',').collect::<Vec<&str>>();
            let updates = update_str
                .iter()
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            // println!("{}", middle(&rules, &updates));
            sum += middle(&rules, &updates);
        } else {
            let rule_str = line.splitn(2, '|').collect::<Vec<&str>>();
            rules.push((
                rule_str[0].parse::<u128>().unwrap(),
                rule_str[1].parse::<u128>().unwrap(),
            ));
        }
    }
    println!("Part 1: {}", sum);
}
