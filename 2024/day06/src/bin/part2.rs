use rayon::prelude::*;

struct Guard {
    pos: (usize, usize),
    dir: (isize, isize),
    is_facing_obstacle: bool,
    pos_visited: Vec<(usize, usize, usize)>,
}

fn check_pos(old_grid: &[Vec<char>], pos: (usize, usize)) -> bool {
    let mut guard = Guard {
        pos: (48, 85), // (48, 85) or (6, 4)
        dir: (-1, 0),
        is_facing_obstacle: false,
        pos_visited: Vec::new(),
    };

    // (0, 0) is the top left corner
    // x is vertical, increasing from top to bottom
    // y is horizontal, increasing from left to right
    if guard.pos == pos {
        return false;
    }

    let mut grid = old_grid.to_vec();
    grid[pos.0][pos.1] = '#';

    guard.pos_visited.push((guard.pos.0, guard.pos.1, 1));

    loop {
        let new_x = (guard.pos.0 as isize + guard.dir.0) as usize;
        let new_y = (guard.pos.1 as isize + guard.dir.1) as usize;

        if (new_x >= grid.len()) || (new_y >= grid[0].len()) {
            return false;
        }

        guard.is_facing_obstacle = grid[new_x][new_y] == '#';

        if guard.is_facing_obstacle {
            guard.dir = match guard.dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => guard.dir,
            };
        } else {
            guard.pos = (new_x, new_y);

            let visited = guard
                .pos_visited
                .iter()
                .position(|(x, y, _)| *x == guard.pos.0 && *y == guard.pos.1);

            if let Some(visited) = visited {
                guard.pos_visited[visited].2 += 1;
                if guard.pos_visited[visited].2 == 4 {
                    return true;
                }
            } else {
                guard.pos_visited.push((guard.pos.0, guard.pos.1, 1));
            }
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/input.txt");
    let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    let count: usize = (0..grid.len())
        .into_par_iter()
        .map(|x| {
            (0..grid[0].len())
                .filter_map(|y| {
                    if grid[x][y] == '#' {
                        None
                    } else if check_pos(&grid, (x, y)) {
                        // println!("X: {}, Y: {}", x, y);
                        Some(1)
                    } else {
                        Some(0)
                    }
                })
                .sum::<usize>()
        })
        .sum();

    println!("Part 2: {}", count);
}
