#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn solve_machine(&self) -> isize {
        let det = self.a.0 * self.b.1 - self.a.1 * self.b.0;
        let a = (self.prize.0 * self.b.1 - self.prize.1 * self.b.0) / det;
        let b = (self.a.0 * self.prize.1 - self.a.1 * self.prize.0) / det;
        if (self.a.0 * a + self.b.0 * b, self.a.1 * a + self.b.1 * b)
            == (self.prize.0, self.prize.1)
        {
            a * 3 + b
        } else {
            0
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut a: (isize, isize) = (0, 0);
    let mut b: (isize, isize) = (0, 0);
    let mut machines: Vec<Machine> = Vec::new();

    for line in input.lines() {
        if line.contains("A") {
            let temp = line.replacen("Button A: X+", "", 1).replacen(" Y+", "", 1);
            let cords = temp.split_once(",").unwrap();
            let x = cords.0.parse::<isize>().unwrap();
            let y = cords.1.parse::<isize>().unwrap();
            a = (x, y);
        } else if line.contains("B") {
            let temp = line.replacen("Button B: X+", "", 1).replacen(" Y+", "", 1);
            let cords = temp.split_once(",").unwrap();
            let x = cords.0.parse::<isize>().unwrap();
            let y = cords.1.parse::<isize>().unwrap();
            b = (x, y)
        } else if line.contains("Prize") {
            let temp = line.replacen("Prize: X=", "", 1).replacen(" Y=", "", 1);
            let cords = temp.split_once(",").unwrap();
            let x = cords.0.parse::<isize>().unwrap();
            let y = cords.1.parse::<isize>().unwrap();
            machines.push(Machine {
                a,
                b,
                prize: (x + 10000000000000, y + 10000000000000),
            });
        }
    }

    machines
}

inventory::submit! {
    crate::Solution { year: 2024, day: 13, part: 2, run: run }
}

fn run() {
    const INPUT: &str = include_str!("input.txt");
    let machines = parse_input(INPUT);
    let mut sum = 0;

    for machine in machines {
        sum += machine.solve_machine();
    }

    println!("Part 2: {}", sum);
}
