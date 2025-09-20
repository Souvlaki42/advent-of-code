use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

#[derive(Debug)]
struct Simulation {
    robot: (usize, usize),
    entities: EntityMap,
    moveset: Vec<Direction>,
    grid_size: (usize, usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Entity {
    Wall,
    BoxLeft,
    BoxRight,
    Air,
    Player,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct EntityMap {
    inner: Vec<Vec<Entity>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct BoxTreeLeaf {
    left: usize,
    y: usize,
}

impl EntityMap {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn set(&mut self, key: (usize, usize), value: Entity) {
        let (col, row) = key;

        while self.inner.len() <= row {
            self.inner.push(Vec::new());
        }

        while self.inner[row].len() <= col {
            self.inner[row].push(Entity::Air);
        }
        self.inner[row][col] = value;
    }

    fn get(&self, key: (usize, usize)) -> Option<Entity> {
        let (x, y) = key;

        if y < self.inner.len() && x < self.inner[y].len() {
            Some(self.inner[y][x])
        } else {
            None
        }
    }

    fn row(&mut self, index: usize) -> Option<&mut Vec<Entity>> {
        if self.inner.len() > index {
            return Some(self.inner.get_mut(index).unwrap());
        }
        None
    }

    fn resolve_to_leaf(&self, key: (usize, usize)) -> Option<BoxTreeLeaf> {
        let entity = self.get(key)?;

        if entity == Entity::BoxLeft {
            return Some(BoxTreeLeaf {
                left: key.0,
                y: key.1,
            });
        };

        if entity == Entity::BoxRight {
            return Some(BoxTreeLeaf {
                left: key.0 - 1,
                y: key.1,
            });
        }

        None
    }
}

impl Simulation {
    fn new(file_path: &str) -> Self {
        let mut entities = EntityMap::new();
        let mut moveset = Vec::new();
        let mut robot = (0, 0);
        let mut line_count = 0;
        let mut x_list: Vec<usize> = Vec::new();

        for (y, line) in file_path.lines().enumerate() {
            line_count += 1;
            for (orig_x, ch) in line.chars().enumerate() {
                let x = 2 * orig_x;
                match ch {
                    '#' => {
                        entities.set((x, y), Entity::Wall);
                        entities.set((x + 1, y), Entity::Wall);
                        x_list.push(x);
                        x_list.push(x + 1);
                    }
                    'O' => {
                        entities.set((x, y), Entity::BoxLeft);
                        entities.set((x + 1, y), Entity::BoxRight);
                        x_list.push(x);
                        x_list.push(x + 1);
                    }
                    '.' => {
                        entities.set((x, y), Entity::Air);
                        entities.set((x + 1, y), Entity::Air);
                        x_list.push(x);
                        x_list.push(x + 1);
                    }
                    '@' => {
                        entities.set((x, y), Entity::Player);
                        entities.set((x + 1, y), Entity::Air);
                        robot = (x, y);
                        x_list.push(x);
                        x_list.push(x + 1);
                    }
                    '^' => moveset.push(Direction::Up),
                    '<' => moveset.push(Direction::Left),
                    '>' => moveset.push(Direction::Right),
                    'v' => moveset.push(Direction::Down),
                    _ => {}
                }
            }
        }

        Self {
            robot,
            entities,
            moveset,
            grid_size: (x_list.iter().max().unwrap_or(&0) + 1, line_count),
        }
    }

    fn sum_gps(&self) -> (usize, usize) {
        let mut sum = 0;
        let mut count = 0;
        for (y, vec) in self.entities.inner.iter().enumerate() {
            for (x, entity) in vec.iter().enumerate() {
                if *entity == Entity::BoxLeft {
                    count += 1;
                    sum += 100 * y + x;
                }
            }
        }
        (sum, count)
    }

    fn print_map(&self) {
        let (max_x, max_y) = self.grid_size;
        for y in 0..max_y {
            for x in 0..max_x {
                let entity = self.entities.get((x, y)).unwrap_or(Entity::Air);
                let ch = match entity {
                    Entity::Air => '.',
                    Entity::BoxLeft => '[',
                    Entity::BoxRight => ']',
                    Entity::Wall => '#',
                    Entity::Player => '@',
                };
                print!("{}", ch);
            }
            println!();
        }
    }

    fn incement_robot_coords_by_direction(
        &mut self,
        direction: Direction,
    ) -> Result<(usize, usize), String> {
        match direction {
            Direction::Up => {
                if self.robot.1 == 0 {
                    return Err("New position is out of bounds".to_string());
                }
                let new_pos = (self.robot.0, self.robot.1 - 1);
                Ok(new_pos)
            }
            Direction::Down => {
                let new_pos = (self.robot.0, self.robot.1 + 1);
                if new_pos.1 >= self.grid_size.1 {
                    return Err("New position is out of bounds".to_string());
                }
                Ok(new_pos)
            }
            Direction::Left => {
                if self.robot.0 == 0 {
                    return Err("New position is out of bounds".to_string());
                }
                let new_pos = (self.robot.0 - 1, self.robot.1);
                Ok(new_pos)
            }
            Direction::Right => {
                let new_pos = (self.robot.0 + 1, self.robot.1);
                if new_pos.0 >= self.grid_size.0 {
                    return Err("New position is out of bounds".to_string());
                }
                Ok(new_pos)
            }
        }
    }

    fn move_on_clear(&mut self, direction: Direction) {
        if let Ok(new_pos) = self.incement_robot_coords_by_direction(direction)
            && let Some(Entity::Air) = self.entities.get(new_pos)
        {
            self.entities.set(self.robot, Entity::Air);
            self.entities.set(new_pos, Entity::Player);
            self.robot = new_pos;
        }
    }

    fn get_box_tree(&mut self, direction: Direction) -> Result<Vec<BoxTreeLeaf>, String> {
        let mut box_tree: Vec<BoxTreeLeaf> = Vec::new();
        let mut to_visit: VecDeque<BoxTreeLeaf> = VecDeque::new();
        let mut visited: HashSet<BoxTreeLeaf> = HashSet::new();

        let start_box_pos = self.incement_robot_coords_by_direction(direction)?;

        let start_box_maybe = self.entities.resolve_to_leaf(start_box_pos);

        if start_box_maybe.is_none() {
            return Ok(Vec::new());
        }

        let start = start_box_maybe.unwrap();
        to_visit.push_back(start);

        while let Some(current) = to_visit.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            let BoxTreeLeaf { left: lx, y } = current;
            let dest_y = match direction {
                Direction::Up => y - 1,
                Direction::Down => y + 1,
                _ => unreachable!(),
            };

            let dest_leafs = [(lx, dest_y), (lx + 1, dest_y)];

            let mut deps = Vec::new();
            for (x, y) in dest_leafs {
                let entity = self
                    .entities
                    .get((x, y))
                    .ok_or("Box entity not found. Aborting".to_string())?;
                if entity == Entity::Air {
                    continue;
                } else if entity == Entity::Wall {
                    return Err("Wall detected. Aborting".to_string());
                } else if entity == Entity::BoxLeft || entity == Entity::BoxRight {
                    deps.push(self.entities.resolve_to_leaf((x, y)).unwrap());
                }
            }

            box_tree.push(current);

            for dep in deps {
                to_visit.push_back(dep);
            }
        }

        match direction {
            Direction::Up => {
                box_tree.sort_by_key(|b| b.y);
            }
            Direction::Down => {
                box_tree.sort_by_key(|b| Reverse(b.y));
            }
            _ => unreachable!(),
        }

        Ok(box_tree)
    }

    fn try_move_boxes(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Left => {
                let in_front = self
                    .entities
                    .get((self.robot.0.saturating_sub(1), self.robot.1));
                if in_front == Some(Entity::BoxRight) {
                    let row = self.entities.row(self.robot.1).unwrap();

                    let airs = row.iter().positions(|e| *e == Entity::Air);
                    let airs_front = airs.rev().find(|p| *p < self.robot.0);

                    if let Some(p) = airs_front {
                        let _ = row.remove(p);

                        row.insert(self.robot.0, Entity::Air);

                        self.robot = (self.robot.0 - 1, self.robot.1);

                        true
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            Direction::Right => {
                let in_front = self.entities.get((self.robot.0 + 1, self.robot.1));
                if in_front == Some(Entity::BoxLeft) {
                    let row = self.entities.row(self.robot.1).unwrap();

                    let mut airs = row.iter().positions(|e| *e == Entity::Air);
                    let airs_front = airs.find(|p| *p > self.robot.0);

                    if let Some(p) = airs_front {
                        let _ = row.remove(p);

                        row.insert(self.robot.0, Entity::Air);

                        self.robot = (self.robot.0 + 1, self.robot.1);

                        true
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            direction => {
                let dest = match self.incement_robot_coords_by_direction(direction) {
                    Ok(pos) => pos,
                    Err(_) => return false, // out of bounds -> no move
                };

                match self.entities.get(dest) {
                    Some(Entity::Wall) | None => {
                        // blocked by wall or OOB
                        return true;
                    }
                    Some(Entity::Air) => {
                        // just move the robot, no boxes
                        self.entities.set(self.robot, Entity::Air);
                        self.entities.set(dest, Entity::Player);
                        self.robot = dest;
                        return true;
                    }
                    _ => {} // Box case handled below
                }

                // --- existing box-moving code follows ---
                match self.get_box_tree(direction) {
                    Ok(box_tree) => {
                        for box_leaf in box_tree {
                            let BoxTreeLeaf { left: x, y } = box_leaf;

                            // clear old
                            self.entities.set((x, y), Entity::Air);
                            self.entities.set((x + 1, y), Entity::Air);

                            // write new
                            match direction {
                                Direction::Up => {
                                    self.entities.set((x, y - 1), Entity::BoxLeft);
                                    self.entities.set((x + 1, y - 1), Entity::BoxRight);
                                }
                                Direction::Down => {
                                    self.entities.set((x, y + 1), Entity::BoxLeft);
                                    self.entities.set((x + 1, y + 1), Entity::BoxRight);
                                }
                                _ => unreachable!(),
                            }
                        }

                        let new_robot = match direction {
                            Direction::Up => (self.robot.0, self.robot.1 - 1),
                            Direction::Down => (self.robot.0, self.robot.1 + 1),
                            _ => unreachable!(),
                        };

                        // clear old position
                        self.entities.set(self.robot, Entity::Air);

                        // set new position
                        self.entities.set(new_robot, Entity::Player);
                        self.robot = new_robot;

                        true
                    }
                    Err(_) => true,
                }
            }
        }
    }

    fn run(&mut self) {
        println!("Initial state:");
        self.print_map();
        for direction in self.moveset.clone() {
            println!("{:?}", direction);
            if !self.try_move_boxes(direction) {
                self.move_on_clear(direction);
            };
            self.print_map();
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/input2.txt");
    let mut simulation = Simulation::new(INPUT);
    simulation.run();

    let (sum, count) = simulation.sum_gps();
    println!("Part 2: {} [{}]", sum, count);
}
