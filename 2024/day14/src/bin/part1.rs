use std::collections::HashMap;

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn wrap_position(&self, map_size: (isize, isize)) -> (isize, isize) {
        let mut new_pos: (isize, isize) = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);

        if new_pos.0 > map_size.0 - 1 {
            new_pos.0 -= map_size.0;
        }

        if new_pos.0 < 0 {
            new_pos.0 += map_size.0;
        }

        if new_pos.1 > map_size.1 - 1 {
            new_pos.1 -= map_size.1;
        }

        if new_pos.1 < 0 {
            new_pos.1 += map_size.1;
        }

        new_pos
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let row = line.replacen("p=", "", 1).replacen("v=", "", 1);
        let parts = row.split_once(" ").unwrap();
        let pos = parts.0.split_once(",").unwrap();
        let vel = parts.1.split_once(",").unwrap();
        robots.push(Robot {
            pos: (
                pos.0.parse::<isize>().unwrap(),
                pos.1.parse::<isize>().unwrap(),
            ),
            vel: (
                vel.0.parse::<isize>().unwrap(),
                vel.1.parse::<isize>().unwrap(),
            ),
        });
    }
    robots
}

fn safety_factor(map: HashMap<(isize, isize), usize>, center: (isize, isize)) -> usize {
    let mut quadrants = (0, 0, 0, 0);
    for (key, value) in map {
        if key.0 < center.0 && key.1 < center.1 {
            quadrants.0 += value;
        }
        if key.0 > center.0 && key.1 < center.1 {
            quadrants.1 += value;
        }
        if key.0 < center.0 && key.1 > center.1 {
            quadrants.2 += value;
        }
        if key.0 > center.0 && key.1 > center.1 {
            quadrants.3 += value;
        }
    }

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn main() {
    const INPUT: &str = include_str!("../inputs/input.txt");
    const SIZE: (isize, isize) = (101, 103); // IMPORTANT: This changes with input file
    const TIMES: usize = 100;

    let mut robots = parse_input(INPUT);
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();

    for i in 0..SIZE.0 {
        for j in 0..SIZE.1 {
            if robots.iter().any(|r| r.pos == (i, j)) {
                map.entry((i, j)).or_insert(1);
            } else {
                map.entry((i, j)).or_default();
            };
        }
    }

    for _ in 1..=TIMES {
        for robot in &mut robots {
            let new_bot = Robot {
                pos: robot.wrap_position(SIZE),
                vel: robot.vel,
            };
            if *map.entry(robot.pos).or_default() > 0 {
                *map.entry(robot.pos).or_default() -= 1; // Decrement the count
            }
            map.entry(new_bot.pos).or_insert(0);
            *map.get_mut(&new_bot.pos).unwrap() += 1;
            *robot = new_bot;
        }
    }

    println!(
        "Part 1: {}",
        safety_factor(map, ((SIZE.0 - 1) / 2, (SIZE.1 - 1) / 2))
    );
}
