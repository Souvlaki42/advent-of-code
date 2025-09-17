use std::collections::{hash_map::Entry, HashMap};

fn add_antinodes(
    grid: &[Vec<char>],
    antinodes: &mut Vec<(usize, usize)>,
    positions: ((usize, usize), (usize, usize)),
) {
    let (x1, y1) = positions.0;
    let (x2, y2) = positions.1;

    antinodes.push((x1, y1));
    antinodes.push((x2, y2));

    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;

    let mut z_x = x1 as i32 - dx;
    let mut z_y = y1 as i32 - dy;

    while z_x >= 0 && z_x < grid.len() as i32 && z_y >= 0 && z_y < grid[0].len() as i32 {
        antinodes.push((z_x as usize, z_y as usize));
        z_x -= dx;
        z_y -= dy;
    }

    let mut w_x = x2 as i32 + dx;
    let mut w_y = y2 as i32 + dy;

    while w_x >= 0 && w_x < grid.len() as i32 && w_y >= 0 && w_y < grid[0].len() as i32 {
        antinodes.push((w_x as usize, w_y as usize));
        w_x += dx;
        w_y += dy;
    }

    if z_x >= 0 && z_x < grid.len() as i32 && z_y >= 0 && z_y < grid[0].len() as i32 {
        antinodes.push((z_x as usize, z_y as usize));
    }
    if w_x >= 0 && w_x < grid.len() as i32 && w_y >= 0 && w_y < grid[0].len() as i32 {
        antinodes.push((w_x as usize, w_y as usize));
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: Vec<(usize, usize)> = Vec::new();

    // (0, 0) is the top left corner
    // x is vertical, increasing from top to bottom
    // y is horizontal, increasing from left to right
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '.' {
                continue;
            }
            if let Entry::Vacant(e) = antennas.entry(grid[x][y]) {
                e.insert(vec![(x, y)]);
            } else {
                antennas.get_mut(&grid[x][y]).unwrap().push((x, y));
            }
        }
    }

    for antenna_positions in antennas.values() {
        for (i, position1) in antenna_positions.iter().enumerate() {
            for position2 in &antenna_positions[i + 1..] {
                add_antinodes(&grid, &mut antinodes, (*position1, *position2));
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();

    println!("Part 2: {:?}", antinodes.len());
}
