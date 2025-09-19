fn find_free_space(
    blocks: &[String],
    length: usize,
    last_position: Option<usize>,
) -> Option<usize> {
    let end_position = if let Some(last_position) = last_position {
        last_position
    } else {
        blocks.len()
    };

    let mut free_space: usize = 0;
    let mut position: Option<usize> = None;
    for (i, block) in blocks.iter().enumerate().take(end_position) {
        if block == "." {
            if free_space == 0 {
                position = Some(i);
            }
            free_space += 1;
            if free_space >= length {
                return position;
            }
        } else {
            free_space = 0;
        }
    }
    None
}

fn main() {
    const INPUT: &str = include_str!("../inputs/input.txt");
    let mut blocks: Vec<String> = Vec::new();
    let mut id = 0;
    for (i, char) in INPUT.chars().enumerate() {
        let num = char.to_digit(10).unwrap();
        if i == 0 || i % 2 == 0 {
            for _ in 0..num {
                blocks.push(id.to_string());
            }
            id += 1;
        } else {
            for _ in 0..num {
                blocks.push(".".to_string());
            }
        }
    }

    let mut files: Vec<Vec<String>> = Vec::new();
    for i in (0..blocks.len()).rev() {
        if blocks[i] == "." {
            continue;
        }
        if files.is_empty() || files.last().unwrap()[0] != blocks[i] {
            files.push(vec![blocks[i].clone()]);
        } else if blocks[i] == files.last().unwrap()[0] {
            files.last_mut().unwrap().push(blocks[i].clone());
        }
    }

    for file in files {
        let mut original_position = blocks.iter().position(|x| *x == file[0]);
        let free_space = find_free_space(&blocks, file.len(), original_position);
        if free_space.is_none() {
            continue;
        }
        let free_space = free_space.unwrap();

        while original_position.is_some() {
            blocks[original_position.unwrap()] = ".".to_string();
            original_position = blocks.iter().position(|x| *x == file[0]);
        }

        blocks[free_space..(file.len() + free_space)].clone_from_slice(&file[..]);
    }

    let mut sum = 0;
    for (i, block) in blocks.iter().enumerate() {
        if block == "." {
            continue;
        }
        let num = block.parse::<u128>().unwrap();
        sum += (i as u128) * num;
    }
    println!("Part 2: {}", sum);
}
