#[derive(Debug)]
struct Machine {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
}

impl Machine {
    fn calculate_tokens(&self) -> Option<u32> {
        let mut minimum = (101, 101);
        for i in 1..=100 {
            for j in 1..=100 {
                if i * self.button_a.0 + j * self.button_b.0 == self.prize.0
                    && i * self.button_a.1 + j * self.button_b.1 == self.prize.1
                    && i < minimum.0
                    && j < minimum.1
                {
                    minimum = (i, j);
                }
            }
        }

        if minimum.0 != 101 && minimum.1 != 101 {
            return Some(minimum.0 * 3 + minimum.1);
        }
        None
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut button_a: (u32, u32) = (0, 0);
    let mut button_b: (u32, u32) = (0, 0);
    let mut machines: Vec<Machine> = Vec::new();

    for line in input.lines() {
        if line.contains("A") {
            let temp = line.replacen("Button A: X+", "", 1).replacen(" Y+", "", 1);
            let cords = temp.split_once(",").unwrap();
            let x = cords.0.parse::<u32>().unwrap();
            let y = cords.1.parse::<u32>().unwrap();
            button_a = (x, y);
        } else if line.contains("B") {
            let temp = line.replacen("Button B: X+", "", 1).replacen(" Y+", "", 1);
            let cords = temp.split_once(",").unwrap();
            let x = cords.0.parse::<u32>().unwrap();
            let y = cords.1.parse::<u32>().unwrap();
            button_b = (x, y)
        } else if line.contains("Prize") {
            let temp = line.replacen("Prize: X=", "", 1).replacen(" Y=", "", 1);
            let cords = temp.split_once(",").unwrap();
            let x = cords.0.parse::<u32>().unwrap();
            let y = cords.1.parse::<u32>().unwrap();
            machines.push(Machine {
                button_a,
                button_b,
                prize: (x, y),
            });
        }
    }

    machines
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let machines = parse_input(INPUT);
    let mut sum = 0;

    for machine in machines {
        sum += machine.calculate_tokens().unwrap_or_default();
    }

    println!("Part 1: {}", sum);
}
