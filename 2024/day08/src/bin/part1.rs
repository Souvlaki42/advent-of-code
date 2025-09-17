use std::collections::{hash_map::Entry, HashMap};

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
        for (i, antenna_position1) in antenna_positions.iter().enumerate() {
            for antenna_position2 in &antenna_positions[i + 1..] {
                let (x1, y1) = *antenna_position1;
                let (x2, y2) = *antenna_position2;

                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                let z_x = x1 as i32 - dx;
                let z_y = y1 as i32 - dy;
                let w_x = x2 as i32 + dx;
                let w_y = y2 as i32 + dy;

                if z_x >= 0 && z_x < grid.len() as i32 && z_y >= 0 && z_y < grid[0].len() as i32 {
                    antinodes.push((z_x as usize, z_y as usize));
                }
                if w_x >= 0 && w_x < grid.len() as i32 && w_y >= 0 && w_y < grid[0].len() as i32 {
                    antinodes.push((w_x as usize, w_y as usize));
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();

    println!("Part 1: {:?}", antinodes.len());
}
