inventory::submit! {
    crate::Solution { year: 2025, day: 1, part: 1, run: run }
}

fn run() {
    let contents = include_str!("input.txt");
    let lines = contents.lines();
    let mut current: i32 = 50;
    let mut password: u32 = 0;
    for line in lines {
        let mut chars = line.chars();
        let direction = match chars.next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => 0,
        };
        let value = chars.as_str().parse::<u32>().unwrap();
        println!("Direction: {} Value: {}", direction, value);
        for _ in 0..value {
            current += direction;
            if current > 99 {
                current = 0;
            }
            if current < 0 {
                current = 99;
            }
        }
        if current == 0 {
            password += 1;
        }
        println!("Current: {} Password: {}", current, password);
    }
}
