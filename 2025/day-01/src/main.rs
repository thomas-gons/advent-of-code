const DATA_PATH: &str = "day-01/data/dial-input.txt";
const INIT_NUMBER: i32 = 50;
const NUMBERS: i32 = 100;


fn part_one(input: &str) -> u32 {
    let mut curr: i32 = INIT_NUMBER;
    let mut zeros: u32 = 0;
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() { continue; }

        let (dir_str, n_str) = line.split_at(1);
        let dir = dir_str.chars().next().unwrap_or(' '); 
        
        let n: i32 = match n_str.parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Warning: Skipping invalid line format: {}", line);
                continue; 
            }
        };

        curr += if dir == 'R' {n} else {-n};

        curr = curr.rem_euclid(NUMBERS);
        if curr == 0 {
            zeros += 1;
        }
    }

    zeros
}

fn part_two(input: &str) -> i32 {
    let mut curr: i32 = INIT_NUMBER;
    let mut zeros: i32 = 0;
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() { continue; }

        let (dir_str, n_str) = line.split_at(1);
        let dir = dir_str.chars().next().unwrap_or(' '); 
        
        let n: i32 = match n_str.parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Warning: Skipping invalid line format: {}", line);
                continue; 
            }
        };

        // count each click as assumed by method 0x434C49434B
        for _ in 1..=n {
            curr += if dir == 'R' {1} else {-1};
            if curr == NUMBERS {
                curr = 0;
            } else if curr == -1 {
                curr = NUMBERS -1;
            }

            if curr == 0 {
                zeros += 1;
            }
        }

    }

    zeros
}

fn main() -> anyhow::Result<()> {
    let input = aoc::read_input(DATA_PATH)?;
    aoc::run(part_one, part_two, &input);    
    
    Ok(())
}
