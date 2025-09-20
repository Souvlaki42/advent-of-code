use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Piece {
    x: i32,
    y: i32,
    c: char, // '@', 'O', '#'
}

type Grid = Vec<Piece>;

fn movements_map(ch: char) -> (i32, i32) {
    match ch {
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        '^' => (0, -1),
        _ => (0, 0),
    }
}

// Helper: find index of a piece at (x,y) if present
fn find_at(grid: &Grid, x: i32, y: i32) -> Option<usize> {
    grid.iter().position(|p| p.x == x && p.y == y)
}

// part1 move: recursively push chain in direction if possible; walls block.
fn move_part1(grid: &mut Grid, piece_idx: usize, dx: i32, dy: i32) -> bool {
    let (nx, ny) = {
        let p = grid[piece_idx];
        (p.x + dx, p.y + dy)
    };
    if let Some(ob_idx) = find_at(grid, nx, ny) {
        if grid[ob_idx].c == '#' {
            return false;
        }
        if move_part1(grid, ob_idx, dx, dy) {
            grid[piece_idx].x = nx;
            grid[piece_idx].y = ny;
            return true;
        }
        false
    } else {
        grid[piece_idx].x = nx;
        grid[piece_idx].y = ny;
        true
    }
}

// part2 move2: collect all pieces that must move, fail if any '#' involved, then move if all allowed.
fn move_part2(grid: &mut Grid, piece_idx: usize, dx: i32, dy: i32) {
    // pieces_to_move holds (index, allowed)
    let mut pieces_to_move: Vec<(usize, bool)> = Vec::new();

    fn already_added(list: &[(usize, bool)], idx: usize) -> bool {
        list.iter().any(|(i, _)| *i == idx)
    }

    fn can_move_chain(
        grid: &Grid,
        piece_idx: usize,
        dx: i32,
        dy: i32,
        acc: &mut Vec<(usize, bool)>,
    ) {
        if already_added(acc, piece_idx) {
            return;
        }
        let p = grid[piece_idx];
        let nx = p.x + dx;
        let ny = p.y + dy;

        // For size: 0 for '@' else 1 (matches Python's size usage)
        let size = if p.c == '@' { 0 } else { 1 };

        // Check three prospective collision positions like the Python:
        // (nx,ny), (nx-1,ny), (nx+1,ny) must be free to mark movable w/o obstacles
        let mut free_direct = true;
        for (tx, ty) in [(nx, ny), (nx - 1, ny), (nx + 1, ny)] {
            if find_at(grid, tx, ty).is_some() {
                free_direct = false;
                break;
            }
        }
        if free_direct {
            acc.push((piece_idx, true));
            return;
        }

        // Otherwise gather obstacles with the same y = ny and x aligned with nx for single width
        // Python collected:
        // v where (v.x in [nx, nx + size] or v.x + 1 in [nx, nx + size]) and v.y == ny and v != piece
        let mut obstacles: Vec<usize> = Vec::new();
        for (i, v) in grid.iter().enumerate() {
            if i == piece_idx {
                continue;
            }
            if v.y == ny {
                let cond_x =
                    (v.x == nx) || (v.x == nx + size) || (v.x + 1 == nx) || (v.x + 1 == nx + size);
                if cond_x {
                    obstacles.push(i);
                }
            }
        }

        // If any obstacle is '#', mark current as not allowed; still add it so the caller sees result.
        if obstacles.iter().any(|&i| grid[i].c == '#') {
            acc.push((piece_idx, false));
            // Still record True per Python? Python appended [piece, False] then appended [piece, True] too.
            // It adds both; but the final decision uses all p[1] (bool) and if any False, no movement.
            // To match behavior, add both entries: False and True. The final "all true" will fail.
            // However, duplicating entries is awkward; instead, add False once and continue discovering others.
            // To mirror the Python flow better, push a True entry too (it does both), but "all true" will still fail due to the False.
            acc.push((piece_idx, true));
        } else {
            acc.push((piece_idx, true));
        }

        // Recurse into obstacles
        for oi in obstacles {
            can_move_chain(grid, oi, dx, dy, acc);
        }
    }

    can_move_chain(grid, piece_idx, dx, dy, &mut pieces_to_move);

    if pieces_to_move.is_empty() {
        pieces_to_move.push((piece_idx, true));
    }
    let all_ok = pieces_to_move.iter().all(|(_, ok)| *ok);

    if all_ok {
        // Move each unique piece once; direction is uniform
        // Preserve original order similar to Python’s append traversal
        // Ensure each piece moved exactly once
        let mut seen = Vec::new();
        for (idx, _) in pieces_to_move {
            if seen.contains(&idx) {
                continue;
            }
            seen.push(idx);
            grid[idx].x += dx;
            grid[idx].y += dy;
        }
    }
}

// part1: move robot by each instruction and sum x+100*y for 'O'
fn part1(mut grid: Grid, moves: &[char]) -> i64 {
    let mut robot_idx = grid.iter().position(|p| p.c == '@').unwrap();
    for &m in moves {
        let (dx, dy) = movements_map(m);
        // robot might change index after moves; re-find by coordinates
        // But move_part1 moves in place and does not change vector ordering, so index stays valid.
        let _ = move_part1(&mut grid, robot_idx, dx, dy);
        robot_idx = grid.iter().position(|p| p.c == '@').unwrap();
    }
    grid.iter()
        .filter(|p| p.c == 'O')
        .map(|p| p.x as i64 + 100 * p.y as i64)
        .sum()
}

// Pretty-print grid similar to Python’s print_grid for debugging
#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    let max_y = grid.iter().map(|p| p.y).max().unwrap_or(0);
    let max_x = grid.iter().map(|p| p.x).max().unwrap_or(0);
    for y in 0..=max_y {
        let mut x = 0;
        let mut line = String::new();
        while x <= max_x {
            if let Some(pi) = find_at(grid, x, y) {
                let c = grid[pi].c;
                if c == '@' {
                    line.push(c);
                    x += 1;
                } else if c == 'O' {
                    line.push_str("[]");
                    x += 2;
                } else if c == '#' {
                    line.push_str("##");
                    x += 2;
                } else {
                    line.push('.');
                    x += 1;
                }
            } else {
                line.push('.');
                x += 1;
            }
        }
        println!("{line}");
    }
    println!();
}

// part2: re-find robot each step, use move_part2, sum like part1
fn part2(mut grid: Grid, moves: &[char]) -> i64 {
    for &m in moves {
        let robot_idx = grid.iter().position(|p| p.c == '@').unwrap();
        let (dx, dy) = movements_map(m);
        move_part2(&mut grid, robot_idx, dx, dy);
    }
    grid.iter()
        .filter(|p| p.c == 'O')
        .map(|p| p.x as i64 + 100 * p.y as i64)
        .sum()
}

fn parse_input(input: &str) -> (Grid, Vec<char>) {
    let mut parts = input.split("\n\n");
    let map = parts.next().unwrap_or("");
    let moves_block = parts.next().unwrap_or("");

    // base grid: skip dots
    let mut grid: Grid = Vec::new();
    for (y, line) in map.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                grid.push(Piece {
                    x: x as i32,
                    y: y as i32,
                    c: ch,
                });
            }
        }
    }

    // movements: all characters in the second block
    let moves: Vec<char> = moves_block.lines().flat_map(|l| l.chars()).collect();

    (grid, moves)
}

fn expand_large_grid(input: &str) -> Grid {
    let mut large: Grid = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            // The Python code appends the same triple for '@', '#', 'O' but doubles x: x*2
            large.push(Piece {
                x: (x as i32) * 2,
                y: y as i32,
                c: ch,
            });
        }
    }
    large
}

fn solve(puzzle_input: &str) -> (i64, i64) {
    let (grid, moves) = parse_input(puzzle_input);
    let p1 = part1(grid.clone(), &moves);

    // Build large_grid based on the first block of input (same as Python)
    let map_block = puzzle_input.split("\n\n").next().unwrap_or("");
    let large_grid = expand_large_grid(map_block);
    let p2 = part2(large_grid, &moves);
    (p1, p2)
}

// Original Python code: https://github.com/Gautzilla/AdventOfCode/blob/main/python_AoC/2024/15/2024_15.py
fn main() {
    // Adjust the input path as needed
    const INPUT: &str = include_str!("../inputs/input.txt");
    let t_start = Instant::now();
    let (p1, p2) = solve(INPUT);
    let elapsed = t_start.elapsed().as_secs_f64();

    println!("{:<20}{:>10}", "Part one", p1);
    println!("{:<20}{:>10}", "Part two", p2);
    println!(
        "\nSolved in {} second{}.",
        elapsed,
        if elapsed >= 2.0 { "s" } else { "" }
    );
}
