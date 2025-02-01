use std::{collections::HashMap, fmt::Debug};

#[derive(PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Finds connected region of the same plant type / character using the depth-first search (DFS) algorithm
fn traverse_region(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    target: char,
    visited: &mut Vec<Vec<bool>>,
    region: &mut Vec<Position>,
) {
    if x >= grid.len() || y >= grid[0].len() || visited[x][y] || grid[x][y] != target {
        return;
    }

    visited[x][y] = true;
    region.push(Position { x, y });

    traverse_region(grid, x - 1, y, target, visited, region);
    traverse_region(grid, x + 1, y, target, visited, region);
    traverse_region(grid, x, y - 1, target, visited, region);
    traverse_region(grid, x, y + 1, target, visited, region);
}

fn find_regions(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Vec<Position>>> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut regions: HashMap<char, Vec<Vec<Position>>> = HashMap::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !visited[x][y] {
                let mut region = Vec::new();
                traverse_region(grid, x, y, grid[x][y], &mut visited, &mut region);
                regions.entry(grid[x][y]).or_default().push(region);
            }
        }
    }

    regions
}

fn count_sides(region: &[Position], pos: &Position) -> usize {
    let mut sides = 0;
    let x = pos.x;
    let y = pos.y;

    // Check top-left (convex corner)
    if !region.contains(&Position { x: x - 1, y }) && !region.contains(&Position { x, y: y - 1 }) {
        sides += 1;
    }

    // Check bottom-left (convex corner)
    if !region.contains(&Position { x: x + 1, y }) && !region.contains(&Position { x, y: y - 1 }) {
        sides += 1;
    }

    // Check top-right (convex corner)
    if !region.contains(&Position { x: x - 1, y }) && !region.contains(&Position { x, y: y + 1 }) {
        sides += 1;
    }

    // Check bottom-right (convex corner)
    if !region.contains(&Position { x: x + 1, y }) && !region.contains(&Position { x, y: y + 1 }) {
        sides += 1;
    }

    // Check top-left (concave corner)
    if region.contains(&Position { x: x - 1, y })
        && region.contains(&Position { x, y: y - 1 })
        && !region.contains(&Position { x: x - 1, y: y - 1 })
    {
        sides += 1;
    }

    // Check bottom-left (concave corner)
    if region.contains(&Position { x: x + 1, y })
        && region.contains(&Position { x, y: y - 1 })
        && !region.contains(&Position { x: x + 1, y: y - 1 })
    {
        sides += 1;
    }

    // Check top-right (concave corner)
    if region.contains(&Position { x: x - 1, y })
        && region.contains(&Position { x, y: y + 1 })
        && !region.contains(&Position { x: x - 1, y: y + 1 })
    {
        sides += 1;
    }

    // Check bottom-right (concave corner)
    if region.contains(&Position { x: x + 1, y })
        && region.contains(&Position { x, y: y + 1 })
        && !region.contains(&Position { x: x + 1, y: y + 1 })
    {
        sides += 1;
    }

    sides
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let region_lists = find_regions(&grid);

    let mut sum = 0;
    for region_list in region_lists.values() {
        for region in region_list {
            let mut side_count = 0;
            for pos in region {
                side_count += count_sides(region, pos);
            }
            sum += side_count * region.len();
        }
    }

    println!("Part 2: {}", sum);
}
