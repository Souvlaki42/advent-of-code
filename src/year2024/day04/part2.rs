fn create_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn strict_equal(grid: &[Vec<char>], a: (usize, usize), b: (isize, isize), letter: char) -> bool {
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

fn check_diagonals(grid: &[Vec<char>], letter: (usize, usize)) -> bool {
    let diagonal1 = strict_equal(grid, letter, (-1, -1), 'M')
        && strict_equal(grid, letter, (0, 0), 'A')
        && strict_equal(grid, letter, (1, 1), 'S');

    let diagonal2 = strict_equal(grid, letter, (1, -1), 'M')
        && strict_equal(grid, letter, (0, 0), 'A')
        && strict_equal(grid, letter, (-1, 1), 'S');

    let diagonal1_reversed = strict_equal(grid, letter, (-1, -1), 'S')
        && strict_equal(grid, letter, (0, 0), 'A')
        && strict_equal(grid, letter, (1, 1), 'M');

    let diagonal2_reversed = strict_equal(grid, letter, (1, -1), 'S')
        && strict_equal(grid, letter, (0, 0), 'A')
        && strict_equal(grid, letter, (-1, 1), 'M');

    (diagonal1 || diagonal1_reversed) && (diagonal2 || diagonal2_reversed)
}

inventory::submit! {
    crate::Solution { year: 2024, day: 4, part: 2, run: run }
}

fn run() {
    const INPUT: &str = include_str!("input.txt");
    let grid = create_grid(INPUT);
    let mut count = 0;

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if check_diagonals(&grid, (x, y)) {
                count += 1;
            }
        }
    }

    println!("Part 2: {}", count);
}
