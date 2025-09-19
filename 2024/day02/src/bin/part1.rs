fn is_safe(array: &[u8]) -> bool {
    let mut increasing: Option<bool> = None;
    for i in 1..array.len() {
        let diff = array[i].abs_diff(array[i - 1]);
        if !(1..=3).contains(&diff) {
            return false;
        }
        if increasing.is_none() {
            increasing = Some(array[i] > array[i - 1]);
        } else if (array[i] > array[i - 1]) != increasing.unwrap() {
            return false;
        }
    }
    true
}

fn main() {
    let contents = include_str!("../inputs/input.txt");
    let lines = contents.lines();
    let mut safe_count: u16 = 0;
    for line in lines {
        let report = line.split_whitespace().collect::<Vec<&str>>();
        let report: Vec<u8> = report.iter().map(|x| x.parse::<u8>().unwrap()).collect();
        if is_safe(&report) {
            safe_count += 1;
        }
    }
    println!("Unusual data contained {} safe reports", safe_count);
}
