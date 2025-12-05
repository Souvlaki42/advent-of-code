use std::collections::{HashMap, HashSet};

inventory::submit! {
    crate::Solution { year: 2025, day: 4, part: 1, run: run }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Vector2<V> {
    x: V,
    y: V,
}

impl Vector2<usize> {
    fn unsigned_add(&self, other: &Vector2<isize>) -> Option<Vector2<usize>> {
        let new_x = self.x.checked_add_signed(other.x);
        let new_y = self.y.checked_add_signed(other.y);

        if let Some(x) = new_x
            && let Some(y) = new_y
        {
            return Some(Self { x: x, y: y });
        }

        return None;
    }
}

fn run() {
    let contents = include_str!("example.txt");
    let mut map = HashMap::new();
    let mut free_rolls: HashSet<Vector2<usize>> = HashSet::new();
    let mut _accessible_count = 0;

    for (y, row) in contents.lines().enumerate() {
        for (x, column) in row.chars().enumerate() {
            if column == '@' {
                map.insert(Vector2 { x: x, y: y }, true);
            } else {
                map.insert(Vector2 { x: x, y: y }, false);
            }
        }
    }
    for (pos, _) in &map {
        let directions: [Option<Vector2<usize>>; 8] = [
            pos.unsigned_add(&Vector2 { x: 0, y: -1 }),
            pos.unsigned_add(&Vector2 { x: 1, y: -1 }),
            pos.unsigned_add(&Vector2 { x: -1, y: -1 }),
            pos.unsigned_add(&Vector2 { x: 1, y: 0 }),
            pos.unsigned_add(&Vector2 { x: -1, y: 0 }),
            pos.unsigned_add(&Vector2 { x: 0, y: 1 }),
            pos.unsigned_add(&Vector2 { x: 1, y: 1 }),
            pos.unsigned_add(&Vector2 { x: -1, y: 1 }),
        ];

        let neighboring_rolls: Vec<Vector2<usize>> = directions
            .iter()
            .filter_map(|dir| *dir)
            .filter(|dir| *map.get(&dir).unwrap_or(&false))
            .collect();

        if neighboring_rolls.len() < 4 {
            free_rolls.insert(*pos);
        }
    }

    for roll in free_rolls {
        println!("Roll: {:?}", roll);
    }

    println!("Accessible paper roll count is {}", _accessible_count);
}
