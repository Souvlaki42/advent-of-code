fn find_character_around(grid: &[Vec<char>], x: usize, y: usize, c: char) -> Vec<(usize, usize)> {
    let mut trails: Vec<(usize, usize)> = Vec::new();
    let directions = [
        (-1, 0), // UP
        (0, 1),  // RIGHT
        (1, 0),  // DOWN
        (0, -1), // LEFT
    ];

    for dir in directions.iter() {
        let new_x = (x as isize + dir.0) as usize;
        let new_y = (y as isize + dir.1) as usize;

        if (new_x >= grid.len()) || (new_y >= grid[0].len()) {
            continue;
        }

        if grid[new_x][new_y] == c {
            trails.push((new_x, new_y));
        }
    }
    trails
}

fn find_trailhead_rating(grid: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut rating = 0;
    let found_one = find_character_around(grid, x, y, '1');
    for one in found_one.iter() {
        let found_two = find_character_around(grid, one.0, one.1, '2');
        for two in found_two.iter() {
            let found_three = find_character_around(grid, two.0, two.1, '3');
            for three in found_three.iter() {
                let found_four = find_character_around(grid, three.0, three.1, '4');
                for four in found_four.iter() {
                    let found_five = find_character_around(grid, four.0, four.1, '5');
                    for five in found_five.iter() {
                        let found_six = find_character_around(grid, five.0, five.1, '6');
                        for six in found_six.iter() {
                            let found_seven = find_character_around(grid, six.0, six.1, '7');
                            for seven in found_seven.iter() {
                                let found_eight =
                                    find_character_around(grid, seven.0, seven.1, '8');
                                for eight in found_eight.iter() {
                                    let found_nine =
                                        find_character_around(grid, eight.0, eight.1, '9');
                                    rating += found_nine.len();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    rating
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut sum = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '0' {
                sum += find_trailhead_rating(&grid, x, y);
            }
        }
    }

    println!("Part 2: {}", sum);
}
