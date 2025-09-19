fn is_safe(array: &[u8]) -> bool {
    let mut increasing: Option<bool> = None;
    for i in 1..array.len() {
        let diff = array[i].abs_diff(array[i - 1]);
        if !(1..=3).contains(&diff) {
            return false;
        }
        if let Some(increasing) = increasing {
            if (array[i] > array[i - 1]) != increasing {
                return false;
            }
        } else {
            increasing = Some(array[i] > array[i - 1]);
        }
    }
    true
}

fn is_tolerable_safe(array: &[u8]) -> bool {
    let safe = is_safe(array);
    if safe {
        return true;
    }
    for i in 0..array.len() {
        let mut new_array = array.to_vec();
        new_array.remove(i);
        let safe = is_safe(&new_array);
        if safe {
            return true;
        }
    }
    false
}

fn main() {
    let contents = include_str!("../inputs/input.txt");
    let lines = contents.lines();
    let mut safe_count = 0;
    for line in lines {
        let report = line.split_whitespace().collect::<Vec<&str>>();
        let report: Vec<u8> = report.iter().map(|x| x.parse::<u8>().unwrap()).collect();
        let safe = is_tolerable_safe(&report);
        if safe {
            safe_count += 1;
        }
    }
    println!("Part 2: {}", safe_count);
}
