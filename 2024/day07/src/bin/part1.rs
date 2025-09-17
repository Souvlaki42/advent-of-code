fn combine<T: Clone>(arr: &[T], length: usize, current: &mut Vec<T>, result: &mut Vec<Vec<T>>) {
    if length == 0 {
        result.push(current.clone());
    } else {
        for x in arr {
            current.push(x.clone());
            combine(arr, length - 1, current, result);
            current.pop();
        }
    }
}

fn evaluate(test_val: &u64, test_items: &[u64]) -> bool {
    let operators = ['+', '*'];
    let mut combinations: Vec<Vec<char>> = Vec::new();
    combine(
        &operators,
        test_items.len() - 1,
        &mut vec![],
        &mut combinations,
    );
    for comb in combinations {
        let mut result = test_items[0];
        for i in 1..test_items.len() {
            match comb[i - 1] {
                '+' => result += test_items[i],
                '*' => result *= test_items[i],
                _ => unreachable!(),
            }
        }

        if result == *test_val {
            return true;
        }
    }
    false
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut sum = 0;
    for line in INPUT.lines() {
        let parts = line.splitn(2, ':').collect::<Vec<&str>>();
        let test_val = parts[0].parse::<u64>().unwrap();
        let test = parts[1].split(' ').collect::<Vec<&str>>();
        let test_items = test
            .iter()
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        if evaluate(&test_val, &test_items) {
            sum += test_val;
        }
    }
    println!("Part 1: {}", sum);
}
