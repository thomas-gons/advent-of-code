use std::fs;

const DATA_PATH: &str = "data/rolls-layout-input.txt";

fn read_data() -> Result<String, std::io::Error> {
    return fs::read_to_string(DATA_PATH)
}

fn part_one(input: &str) -> usize {
    let rolls_layout: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let height = rolls_layout.len();
    let width = rolls_layout[0].len();
        let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut accessible_count = 0;

    for y in 0..height {
        for x in 0..width {
            if !rolls_layout[y][x] { continue;}

            let neighbor_count = directions.iter().filter(|&(dy, dx)| {
                let (nx, ny) = (dx + (x as isize), dy + (y as isize));
                if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                    return false;
                }
                rolls_layout[ny as usize][nx as usize] == true
            }).count();

            if neighbor_count < 4 {
                accessible_count += 1;
            }
        }
    }

    accessible_count
}

fn part_two(input: &str) -> usize {
    let mut rolls_layout: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let height = rolls_layout.len();
    let width = rolls_layout[0].len();
        let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut accessible_locations = Vec::new();
    let mut accessible_count = 0;
    loop {
        for y in 0..height {
            for x in 0..width {
                if !rolls_layout[y][x] { continue; }

                let neighbor_count = directions.iter().filter(|&(dy, dx)| {
                    let (nx, ny) = (dx + (x as isize), dy + (y as isize));
                    if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                        return false;
                    }
                    rolls_layout[ny as usize][nx as usize] == true
                }).count();

                if neighbor_count < 4 {
                    accessible_locations.push((x, y));
                }
            }
        }

        if accessible_locations.is_empty() { break; }

        accessible_count += accessible_locations.len();
        for (x, y) in &accessible_locations {
            rolls_layout[*y][*x] = false;
        }

        accessible_locations.clear();
    }

    accessible_count
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_data()?;
    println!("The solution for the first part is {}", part_one(&input));
    println!("The solution for the second part is {}", part_two(&input));    
    
    Ok(())
}
