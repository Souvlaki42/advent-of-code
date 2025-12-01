use std::collections::HashMap;

inventory::submit! {
    crate::Solution { year: 2024, day: 1, part: 2, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let lines = contents.lines();
    let mut list1 = Vec::<u32>::new();
    let mut list1unique = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();
    for line in lines {
        let list_parts = line.split_whitespace().collect::<Vec<&str>>();
        let num1 = list_parts[0].parse::<u32>().unwrap();
        let num2 = list_parts[1].parse::<u32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }
    for num in list1.iter() {
        if !list1unique.contains(num) {
            list1unique.push(*num);
        }
    }
    let mut ocurrences = HashMap::<u32, u32>::new();
    for num in list1unique.iter() {
        ocurrences.insert(*num, 0);
        for num2 in list2.iter() {
            if *num == *num2 {
                ocurrences.insert(*num, ocurrences.get(num).unwrap() + 1);
            }
        }
    }
    let mut sum = 0;
    for (key, value) in ocurrences.iter() {
        sum += *key * *value;
    }
    println!("Sum of product of uniques: {}", sum);
}
