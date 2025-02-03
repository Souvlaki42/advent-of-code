use std::collections::HashMap;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator};

#[derive(Debug, PartialEq)]
struct Robot {
    x: usize,
    y: usize,
}

impl Robot {
    fn check_towards(
        &self,
        entities: HashMap<(usize, usize), Entity>,
        direction: &Instruction,
    ) -> Vec<Entity> {
        let mut increaser = 1;
        let mut direction_entities: Vec<Entity> = Vec::new();
        match direction {
            Instruction::Up => {
                while *entities
                    .get(&(self.x, self.y - increaser))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *entities
                            .get(&(self.x, self.y - increaser))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
            Instruction::Down => {
                while *entities
                    .get(&(self.x, self.y + increaser))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *entities
                            .get(&(self.x, self.y + increaser))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
            Instruction::Left => {
                while *entities
                    .get(&(self.x - increaser, self.y))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *entities
                            .get(&(self.x - increaser, self.y))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
            Instruction::Right => {
                while *entities
                    .get(&(self.x + increaser, self.y))
                    .unwrap_or(&Entity::Air)
                    != Entity::Wall
                {
                    direction_entities.push(
                        *entities
                            .get(&(self.x + increaser, self.y))
                            .unwrap_or(&Entity::Air),
                    );
                    increaser += 1;
                }
                direction_entities.push(Entity::Wall);
            }
        }
        direction_entities
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Entity {
    Wall,
    Box,
    Air,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

fn slide_boxes(entities: Vec<Entity>) -> Vec<Entity> {
    let mut new_entities = entities.clone();

    if let Some(i) = new_entities.par_iter().position_last(|e| *e == Entity::Air) {
        new_entities.remove(i);
        new_entities.insert(0, Entity::Air);
    }
    new_entities
}

fn parse_input(input: &str) -> (HashMap<(usize, usize), Entity>, Vec<Instruction>, Robot) {
    let mut entities = HashMap::new();
    let mut instructions = Vec::new();
    let mut robot: Robot = Robot { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
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
                    robot.x = x;
                    robot.y = y;
                }
                '^' => instructions.push(Instruction::Up),
                '<' => instructions.push(Instruction::Left),
                '>' => instructions.push(Instruction::Right),
                'v' => instructions.push(Instruction::Down),
                _ => {}
            }
        }
    }

    (entities, instructions, robot)
}

fn calculate_movement(input: &str) -> HashMap<(usize, usize), Entity> {
    let (mut entities, instructions, mut robot) = parse_input(input);
    for instruction in instructions {
        let towards = robot.check_towards(entities.clone(), &instruction).clone();
        println!(
            "Robot({}, {}) -> {:?} {:?}",
            robot.x, robot.y, instruction, towards
        );
        match instruction {
            Instruction::Up => {
                if towards[0] == Entity::Box {
                    let slide = slide_boxes(towards.clone());
                    for (i, slide_item) in slide.iter().enumerate() {
                        entities.insert((robot.x, robot.y - i - 1), *slide_item);
                    }
                }

                if towards[0] == Entity::Air {
                    entities.insert((robot.x, robot.y - 1), Entity::Air);
                    robot.y -= 1;
                }
            }
            Instruction::Down => {
                if towards[0] == Entity::Box {
                    let slide = slide_boxes(towards.clone());
                    for (i, slide_item) in slide.iter().enumerate() {
                        entities.insert((robot.x, robot.y + i + 1), *slide_item);
                    }
                }

                if towards[0] == Entity::Air {
                    entities.insert((robot.x, robot.y + 1), Entity::Air);
                    robot.y += 1;
                }
            }
            Instruction::Left => {
                if towards[0] == Entity::Box {
                    let slide = slide_boxes(towards.clone());
                    for (i, slide_item) in slide.iter().enumerate() {
                        entities.insert((robot.x - i - 1, robot.y), *slide_item);
                    }
                }

                if towards[0] == Entity::Air {
                    entities.insert((robot.x - 1, robot.y), Entity::Air);
                    robot.x -= 1;
                }
            }
            Instruction::Right => {
                if towards[0] == Entity::Box {
                    let slide = slide_boxes(towards.clone());
                    for (i, slide_item) in slide.iter().enumerate() {
                        entities.insert((robot.x + i + 1, robot.y), *slide_item);
                    }
                }

                if towards[0] == Entity::Air {
                    entities.insert((robot.x + 1, robot.y), Entity::Air);
                    robot.x += 1;
                }
            }
        }
    }
    entities
}

fn main() {
    const INPUT: &str = include_str!("input2.txt");
    let entities = calculate_movement(INPUT);
    let mut sum = 0;

    for ((x, y), value) in entities {
        if value == Entity::Box {
            // println!("Box({}, {})", x, y);
            sum += y * 100 + x;
        }
    }

    println!("Part 1: {}", sum);
}
