use std::collections::HashSet;

const DATA_PATH: &str = "day-07/data/tachyon-manifold-diagram-input.txt";

fn part_one(input: &str) -> usize {
    let mut beam_splits = 0usize;
    let mut beam_locations = HashSet::<u16>::new();
    for line in input.lines() {
        if line.contains("S") {
            beam_locations.insert(line.find('S').unwrap() as u16);
        } else {
            let mut next_beam_locations = beam_locations.clone();
            for (i, c) in line.chars().enumerate() {
                let position = i as u16;
                if c == '^' && beam_locations.contains(&position) {
                    if i > 0 {
                        next_beam_locations.insert(position - 1);
                    }
                    if i < line.len() - 1 {
                        next_beam_locations.insert(position + 1);
                    }
                    beam_splits += 1;
                    next_beam_locations.remove(&position);
                }
            }
            beam_locations = next_beam_locations;
        }
            
    }
    beam_splits
}


fn simulate_timeline(
    row: usize,
    col: usize,
    line_size: usize,
    diagram: &Vec<Vec<usize>>,
    memo: &mut Vec<Vec<isize>>
) -> usize {
    // If we're past the bottom → exactly 1 final timeline survives
    if row >= diagram.len() {
        return 1;
    }

    if memo[row][col] != -1 {
        return memo[row][col] as usize;
    }

    let mut total = 0;

    if diagram[row].contains(&col) {
        // LEFT branch
        if col > 0 {
            total += simulate_timeline(row + 1, col - 1, line_size, diagram, memo);
        }
        // RIGHT branch
        if col < line_size - 1 {
            total += simulate_timeline(row + 1, col + 1, line_size, diagram, memo);
        }
    } else {
        // No splitter → go straight down
        total += simulate_timeline(row + 1, col, line_size, diagram, memo);
    }

    memo[row][col] = total as isize;
    total
}


fn part_two(input: &str) -> usize {
    let line_size = input.lines().next().unwrap().len();

    let mut start_pos = 0;
    let mut diagram = Vec::<Vec<usize>>::new();

    for line in input.lines() {
        if let Some(idx) = line.find('S') {
            start_pos = idx as usize;
            continue;
        }

        let mut row = Vec::<usize>::new();
        for (i, c) in line.chars().enumerate() {
            if c == '^' {
                row.push(i as usize);
            }
        }
        if !row.is_empty() {
            diagram.push(row);
        }
    }

    let mut memo = vec![vec![-1; line_size as usize]; diagram.len()];
    simulate_timeline(0, start_pos, line_size, &diagram, &mut memo)
}



fn main() -> anyhow::Result<()> {
    let input = aoc::read_input(DATA_PATH)?;
    aoc::run(part_one, part_two, &input);
    
    Ok(())
}
