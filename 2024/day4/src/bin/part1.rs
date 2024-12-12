fn create_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn strict_equal_char(
    grid: &[Vec<char>],
    a: (usize, usize),
    b: (isize, isize),
    letter: char,
) -> bool {
    let row = grid.get(((a.0 as isize) + b.0) as usize);
    if row.is_none() {
        return false;
    }
    let row = row.unwrap();
    let col = row.get(((a.1 as isize) + b.1) as usize);
    if col.is_none() {
        return false;
    }
    col.unwrap() == &letter
}

fn strict_equal(
    grid: &[Vec<char>],
    a: (usize, usize),
    b: (isize, isize),
    letters: &[char],
) -> bool {
    for (i, c) in letters.iter().enumerate() {
        let offset = (b.0 * (i as isize), b.1 * (i as isize));
        if !strict_equal_char(grid, a, offset, *c) {
            return false;
        }
    }
    true
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
    let grid = create_grid(INPUT);
    let mut count = 0;

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if strict_equal(&grid, (x, y), (0, 1), &WORD) {
                // UP
                count += 1;
            }
            if strict_equal(&grid, (x, y), (0, -1), &WORD) {
                // DOWN
                count += 1;
            }
            if strict_equal(&grid, (x, y), (1, 0), &WORD) {
                // RIGHT
                count += 1;
            }
            if strict_equal(&grid, (x, y), (-1, 0), &WORD) {
                // LEFT
                count += 1;
            }
            if strict_equal(&grid, (x, y), (1, 1), &WORD) {
                // UP RIGHT
                count += 1;
            }
            if strict_equal(&grid, (x, y), (-1, -1), &WORD) {
                // DOWN LEFT
                count += 1;
            }
            if strict_equal(&grid, (x, y), (1, -1), &WORD) {
                // UP LEFT
                count += 1;
            }
            if strict_equal(&grid, (x, y), (-1, 1), &WORD) {
                // DOWN RIGHT
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}
