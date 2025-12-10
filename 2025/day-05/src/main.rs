use std::fs;

const DATA_PATH: &str = "data/ingredients-ids-input.txt";

fn read_data() -> Result<String, std::io::Error> {
    fs::read_to_string(DATA_PATH)
}
fn _merge_ranges(ranges: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // Sort ranges by their start values
    ranges.sort_by_key(|k| k.0);

    let mut merged: Vec<(usize, usize)> = Vec::new();

    for (start, end) in ranges.iter() {
        // As the ranges are sorted, we can just check the last merged range
        // to see if we need to merge or add a new range
        if let Some(last) = merged.last_mut() {
            if *start <= last.1 + 1 {
                last.1 = last.1.max(*end);
            } else {
                merged.push((*start, *end));
            }
        } else {
            merged.push((*start, *end));
        }
    }

    merged
}

fn part_one(data: &str) -> usize {
    let mut fresh_ingredients_ranges: Vec<(usize, usize)> = Vec::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split('-').collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        fresh_ingredients_ranges.push((start, end));
    }

    let available_ingredients_ids: Vec<&str> = data.lines().skip(fresh_ingredients_ranges.len() + 1).collect();
    fresh_ingredients_ranges = _merge_ranges(&mut fresh_ingredients_ranges);
    let mut fresh_ingredients_count: usize = 0;

    for available_id_str in available_ingredients_ids {
        let available_id: usize = available_id_str.parse().unwrap();
        let mut is_fresh = false;

        for (start, end) in &fresh_ingredients_ranges {
            if available_id >= *start && available_id <= *end {
                is_fresh = true;
                break;
            }
        }

        if is_fresh {
            fresh_ingredients_count += 1;
        }
        
    }
    fresh_ingredients_count
}

fn part_two(data: &str) -> usize {
    let mut fresh_ingredients_ranges: Vec<(usize, usize)> = Vec::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split('-').collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        fresh_ingredients_ranges.push((start, end));
    }

    fresh_ingredients_ranges = _merge_ranges(&mut fresh_ingredients_ranges);
    
    fresh_ingredients_ranges.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_data()?;

    println!("The solution for the first part is: {}", part_one(&input));
    println!("The solution for the second part is: {}", part_two(&input));

    Ok(())
}
