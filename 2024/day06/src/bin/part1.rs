struct Guard {
    pos: (usize, usize),
    dir: (isize, isize),
    is_facing_obstacle: bool,
    pos_visited: Vec<(usize, usize)>,
}

fn main() {
    const INPUT: &str = include_str!("../inputs/input.txt");
    let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut count = 1;
    let mut guard = Guard {
        pos: (48, 85), // (48, 85) or (6, 4)
        dir: (-1, 0),
        is_facing_obstacle: false,
        pos_visited: Vec::new(),
    };

    // (0, 0) is the top left corner
    // x is vertical, increasing from top to bottom
    // y is horizontal, increasing from left to right

    guard.pos_visited.push(guard.pos);

    loop {
        let new_x = (guard.pos.0 as isize + guard.dir.0) as usize;
        let new_y = (guard.pos.1 as isize + guard.dir.1) as usize;

        if (new_x >= grid.len()) || (new_y >= grid[0].len()) {
            break;
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

            if !guard.pos_visited.contains(&guard.pos) {
                guard.pos_visited.push(guard.pos);
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}
