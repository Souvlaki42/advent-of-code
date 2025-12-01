use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
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

fn perimeter(positions: &[Position], pos: &Position) -> usize {
    let mut perimeter = 0;
    let x = pos.x;
    let y = pos.y;

    // Check top
    if !positions.contains(&Position { x: x - 1, y }) {
        perimeter += 1;
    }

    // Check bottom
    if !positions.contains(&Position { x: x + 1, y }) {
        perimeter += 1;
    }

    // Check left
    if !positions.contains(&Position { x, y: y - 1 }) {
        perimeter += 1;
    }

    // Check right
    if !positions.contains(&Position { x, y: y + 1 }) {
        perimeter += 1;
    }

    perimeter
}

inventory::submit! {
    crate::Solution { year: 2024, day: 12, part: 1, run: run }
}

fn run() {
    const INPUT: &str = include_str!("input.txt");
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let regions = find_regions(&grid);

    let mut sum = 0;
    for (_, value) in regions.iter() {
        for region in value.iter() {
            let mut perimeters = 0;
            for pos in region.iter() {
                perimeters += perimeter(region, pos);
            }
            sum += perimeters * region.len();
        }
    }

    println!("Part 1: {}", sum);
}
