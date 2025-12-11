
const DATA_PATH: &str = "day-02/data/ids-input.txt";

fn part_one(input: &str) -> u64 {
    let mut invalid_ids_sum = 0u64;

    for range in input.split(',') {
        let mut bounds = range.split('-');
        let min: u64 = bounds.next().unwrap().trim().parse().unwrap();
        let max: u64 = bounds.next().unwrap().trim().parse().unwrap();

        // Determine the length of numbers in the range
        let min_len = min.to_string().len();
        let max_len = max.to_string().len();

        // Only even lengths can be repeated sequences
        for len in min_len..=max_len {
            if len % 2 != 0 {
                continue;
            }

            let half_len = len / 2;
            let start = 10u64.pow((half_len - 1) as u32);
            let end = 10u64.pow(half_len as u32) - 1;

            for n in start..=end {
                // Construct the repeated number: "n" repeated twice
                let repeated = n * 10u64.pow(half_len as u32) + n;
                if repeated >= min && repeated <= max {
                    invalid_ids_sum += repeated;
                }
            }
        }
    }

    invalid_ids_sum
}

use std::collections::HashSet;

fn part_two(input: &str) -> u64 {
    // Use a HashSet to avoid double counting (e.g. "1212" and "12" both generate 121212)
    let mut invalid_ids = HashSet::new();

    for range in input.split(',') {
        let mut bounds = range.split('-');
        let min_str = bounds.next().unwrap().trim();
        let max_str = bounds.next().unwrap().trim();
        
        let min: u64 = min_str.parse().unwrap();
        let max: u64 = max_str.parse().unwrap();

        let max_len = max_str.len();

        // Pattern lengths from 1 up to half of max digits
        for pattern_len in 1..=max_len / 2 {
            let start = 10u64.pow((pattern_len - 1) as u32);
            let end = 10u64.pow(pattern_len as u32) - 1;

            for n in start..=end {
                let mut repeated = 0u64;
                let mut digits = 0;

                // Repeat n until we exceed max
                while repeated <= max {
                    // handle eventual overflow
                    repeated = repeated.saturating_mul(10u64.pow(pattern_len as u32)) + n;
                    digits += pattern_len;
                    
                    if repeated > max {
                        break;
                    }
                    
                    if digits >= 2 * pattern_len && repeated >= min {
                        invalid_ids.insert(repeated);
                    }
                }
            }
        }
    }

    invalid_ids.iter().sum()
}

fn main() -> anyhow::Result<()> {
    let input = aoc::read_input(DATA_PATH)?;
    aoc::run(part_one, part_two, &input);

    Ok(())
}
