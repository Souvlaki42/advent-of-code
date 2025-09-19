use itertools::Itertools;

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

impl EntityMap {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn set(&mut self, key: (usize, usize), value: Entity) {
        let (col, row) = key;

        // Ensure the outer vector has enough rows
        while self.inner.len() <= row {
            self.inner.push(Vec::new());
        }
        // Ensure the inner vector has enough columns
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
}

type BoxTree = Vec<Vec<(usize, usize)>>;

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

    fn get_box_tree(&mut self) -> BoxTree {
        let mut increaser = 1;
        let mut box_tree: BoxTree = Vec::new();
        while self.robot.1 - increaser > 0 {
            if box_tree.len() < increaser {
                box_tree.push(Vec::new());
            }

            let row = self.entities.row(self.robot.1 - increaser).unwrap();

            let box_row = row
                .iter()
                .enumerate()
                .filter(|(k, e)| {
                    if **e != Entity::BoxLeft && **e != Entity::BoxRight {
                        return false;
                    }

                    if **e == Entity::BoxLeft
                        && *k != self.robot.0
                        && !box_tree[increaser - 1].contains(&(*k + 1, self.robot.1 + increaser))
                        && (increaser >= 2
                            && !box_tree[increaser - 2].contains(&(*k, self.robot.1 + increaser)))
                    {
                        return false;
                    }

                    if **e == Entity::BoxRight
                        && *k != self.robot.0
                        && !box_tree[increaser - 1].contains(&(*k - 1, self.robot.1 + increaser))
                        && (increaser >= 2
                            && !box_tree[increaser - 2].contains(&(*k, self.robot.1 + increaser)))
                    {
                        return false;
                    }

                    true
                })
                .map(|(k, _)| (k, self.robot.1 + increaser))
                .collect_vec();

            box_tree.push(box_row);
            increaser += 1;
        }

        box_tree
    }

    fn move_on_clear(&mut self, direction: Direction) {
        let new_pos = match direction {
            Direction::Up => (self.robot.0, self.robot.1 - 1),
            Direction::Down => (self.robot.0, self.robot.1 + 1),
            Direction::Left => (self.robot.0 - 1, self.robot.1),
            Direction::Right => (self.robot.0 + 1, self.robot.1),
        };

        if let Some(entity) = self.entities.get(new_pos) {
            if entity == Entity::Air {
                self.entities.set(self.robot, Entity::Air);
                self.entities.set(new_pos, Entity::Player);
                self.robot = new_pos;
            }
        } else {
            println!("Invalid position: {:?}", new_pos);
        }
    }

    fn move_boxes(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                println!("{:?}", self.get_box_tree());
            }
            Direction::Down => {}
            Direction::Left => {
                let new_pos = (self.robot.0 - 1, self.robot.1);
                if self.entities.get(new_pos) == Some(Entity::BoxRight) {
                    let row = self.entities.row(self.robot.1).unwrap();
                    let airs = row.iter().positions(|e| *e == Entity::Air);
                    if let Some(airs_front) = airs.filter(|p| *p < self.robot.0).next_back() {
                        row.remove(airs_front);
                        self.robot = new_pos;
                        row.insert(new_pos.0 + 1, Entity::Air);
                    }
                }
            }
            Direction::Right => {
                let new_pos = (self.robot.0 + 1, self.robot.1);
                if self.entities.get(new_pos) == Some(Entity::BoxLeft) {
                    let row = self.entities.row(self.robot.1).unwrap();
                    let mut airs = row.iter().positions(|e| *e == Entity::Air);
                    if let Some(airs_front) = airs.find(|p| *p > self.robot.0) {
                        row.remove(airs_front);
                        self.robot = new_pos;
                        row.insert(new_pos.0 + 1, Entity::Air);
                    }
                }
            }
        }
    }

    fn run(&mut self) {
        println!("Initial state:");
        self.print_map();
        for direction in self.moveset.clone() {
            println!("{:?}", direction);
            self.move_boxes(direction);
            self.move_on_clear(direction);
            self.print_map();
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/input2.txt");
    let mut simulation = Simulation::new(INPUT);
    simulation.run();

    // let (sum, count) = simulation.sum_gps();
    // println!("Part 1: {} [{}]", sum, count);
}
