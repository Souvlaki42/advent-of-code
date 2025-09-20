use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Simulation<'a> {
    robot: (usize, usize),
    entities: HashMap<(usize, usize), Entity>,
    moveset: Vec<Instruction>,
    input: (&'a str, usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Entity {
    Wall,
    Box,
    Air,
    Player,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl<'a> Simulation<'a> {
    fn new(file: &'a str) -> Self {
        let mut entities = HashMap::new();
        let mut moveset = Vec::new();
        let mut robot = (0, 0);
        let mut line_count: usize = 0;

        for (y, line) in file.lines().enumerate() {
            line_count += 1;
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        entities.insert((x, y), Entity::Wall);
                    }
                    'O' => {
                        entities.insert((x, y), Entity::Box);
                    }
                    '.' => {
                        entities.insert((x, y), Entity::Air);
                    }
                    '@' => {
                        entities.insert((x, y), Entity::Player);
                        robot.0 = x;
                        robot.1 = y;
                    }
                    '^' => moveset.push(Instruction::Up),
                    '<' => moveset.push(Instruction::Left),
                    '>' => moveset.push(Instruction::Right),
                    'v' => moveset.push(Instruction::Down),
                    _ => {}
                }
            }
        }

        Self {
            robot,
            entities,
            moveset,
            input: (file, line_count),
        }
    }

    fn set_pos(&mut self, change_pos: (isize, isize)) {
        let robot_pos_unwrapped = (
            isize::try_from(self.robot.0).unwrap_or(0),
            isize::try_from(self.robot.1).unwrap_or(0),
        );

        let new_pos = (
            usize::try_from(robot_pos_unwrapped.0 + change_pos.0)
                .expect("Robot X isn't a usize anymore..."),
            usize::try_from(robot_pos_unwrapped.1 + change_pos.1)
                .expect("Robot Y isn't a usize anymore..."),
        );
        if new_pos == self.robot {
            println!("Robot position didn't changed: {:?}", self.robot);
            return;
        }

        self.entities.insert(self.robot, Entity::Air);
        self.entities.insert(new_pos, Entity::Player);
        self.robot = new_pos;
    }

    fn print_map(&self) {
        let max_x = self.entities.keys().map(|(x, _)| *x).max().unwrap_or(0) + 1;
        let max_y = self.input.1;

        for y in 0..max_y {
            for x in 0..max_x {
                let entity = self.entities.get(&(x, y)).unwrap_or(&Entity::Air);
                let ch = match entity {
                    Entity::Air => '.',
                    Entity::Box => 'O',
                    Entity::Wall => '#',
                    Entity::Player => '@',
                };
                print!("{}", ch);
            }
            println!();
        }
    }

    fn sum_gps(&self) -> (usize, usize) {
        let mut sum = 0;
        let mut count = 0;
        for ((x, y), entity) in &self.entities {
            if *entity == Entity::Box {
                count += 1;
                sum += 100 * y + x;
            }
        }
        (sum, count)
    }

    fn check_towards(&self, direction: &Instruction) -> Vec<Entity> {
        let mut increaser = 1;
        let mut direction_entities: Vec<Entity> = Vec::new();
        match direction {
            Instruction::Up => {
                while *self
                    .entities
                    .get(&(self.robot.0, self.robot.1 - increaser))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *self
                            .entities
                            .get(&(self.robot.0, self.robot.1 - increaser))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
            Instruction::Down => {
                while *self
                    .entities
                    .get(&(self.robot.0, self.robot.1 + increaser))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *self
                            .entities
                            .get(&(self.robot.0, self.robot.1 + increaser))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
            Instruction::Left => {
                while *self
                    .entities
                    .get(&(self.robot.0 - increaser, self.robot.1))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *self
                            .entities
                            .get(&(self.robot.0 - increaser, self.robot.1))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
            Instruction::Right => {
                while *self
                    .entities
                    .get(&(self.robot.0 + increaser, self.robot.1))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *self
                            .entities
                            .get(&(self.robot.0 + increaser, self.robot.1))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
        }
        direction_entities
    }

    fn slide_boxes(&self, entities: Vec<Entity>) -> Vec<Entity> {
        let mut new_entities = entities.clone();

        if let Some(i) = new_entities.iter().position(|e| *e == Entity::Air) {
            new_entities.remove(i);
            new_entities.insert(0, Entity::Air);
        }
        new_entities
    }

    fn run(&mut self) {
        println!("Initial state:");
        self.print_map();
        for instruction in self.moveset.clone() {
            println!("{:?}", instruction);
            let mut towards = self.check_towards(&instruction);
            match instruction {
                Instruction::Up => {
                    if towards[0] == Entity::Box {
                        let slide = self.slide_boxes(towards.clone());
                        for (i, slide_item) in slide.iter().enumerate() {
                            self.entities
                                .insert((self.robot.0, self.robot.1 - i - 1), *slide_item);
                        }
                        towards = self.check_towards(&instruction);
                    }

                    if towards[0] == Entity::Air {
                        self.set_pos((0, -1));
                    }
                }
                Instruction::Down => {
                    if towards[0] == Entity::Box {
                        let slide = self.slide_boxes(towards.clone());
                        for (i, slide_item) in slide.iter().enumerate() {
                            self.entities
                                .insert((self.robot.0, self.robot.1 + i + 1), *slide_item);
                        }
                        towards = self.check_towards(&instruction);
                    }

                    if towards[0] == Entity::Air {
                        self.set_pos((0, 1));
                    }
                }
                Instruction::Left => {
                    if towards[0] == Entity::Box {
                        let slide = self.slide_boxes(towards.clone());
                        for (i, slide_item) in slide.iter().enumerate() {
                            self.entities
                                .insert((self.robot.0 - i - 1, self.robot.1), *slide_item);
                        }
                        towards = self.check_towards(&instruction);
                    }

                    if towards[0] == Entity::Air {
                        self.set_pos((-1, 0));
                    }
                }
                Instruction::Right => {
                    if towards[0] == Entity::Box {
                        let slide = self.slide_boxes(towards.clone());
                        for (i, slide_item) in slide.iter().enumerate() {
                            self.entities
                                .insert((self.robot.0 + i + 1, self.robot.1), *slide_item);
                        }
                        towards = self.check_towards(&instruction);
                    }

                    if towards[0] == Entity::Air {
                        self.set_pos((1, 0));
                    }
                }
            }
            self.print_map();
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/input.txt");
    let mut simulation = Simulation::new(INPUT);
    simulation.run();

    let (sum, count) = simulation.sum_gps();
    println!("Part 1: {} [{}]", sum, count);
}
