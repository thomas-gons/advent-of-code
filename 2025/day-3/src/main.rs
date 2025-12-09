use std::fs;

const DATA_PATH: &str = "data/batteries-input.txt";
const PART_2_BATTERY_TO_USE: usize = 5;

fn read_data() -> Result<String, std::io::Error> {
    fs::read_to_string(DATA_PATH)
}

fn part_one(input: &str) -> usize {
    let mut overall_joltage = 0usize;

    for line in input.lines() {
        // s0: first digit of the best two-digit number so far (tens place)
        // s1: second digit of the best two-digit number so far (ones place)
        let (mut s0, mut s1) = (0u8, 0u8);

        // Iterate over each digit in the battery bank
        for char in line.chars() {
            let n: u8 = char.to_digit(10).unwrap() as u8;

            // If the current ones-digit is larger than the tens-digit,
            // the optimal sequence becomes s1 followed by the new digit
            if s1 > s0 {
                s0 = s1;
                s1 = n;
            } 
            // Otherwise, if the new digit is larger than the current ones-digit,
            // it should replace s1 to maximize the two-digit number
            else if n > s1 {
                s1 = n;
            }
        }

        // Compute the value of the best two-digit number and add it to the total
        overall_joltage += (s0 as usize) * (s1 as usize);
    }

    overall_joltage
}

fn part_two(input: &str) -> usize {
    let mut joltage: usize = 0;

    for line in input.lines() {
        let mut stack: Vec<u8> = Vec::new();
        let mut to_remove = line.len() - PART_2_BATTERY_TO_USE;

        // Greedily build the largest number by removing smaller digits when possible (size constraint)
        for c in line.chars() {
            let d = c.to_digit(10).unwrap() as u8;

            // Remove smaller digits from the stack if we still can remove digits
            while !stack.is_empty()
                && to_remove > 0
                && *stack.last().unwrap() < d
            {
                stack.pop();
                to_remove -= 1;
            }

            stack.push(d);
        }

        // In case the stack is still larger than needed
        stack.truncate(PART_2_BATTERY_TO_USE);

        let mut value: usize = 0;
        for &digit in &stack {
            value = value * 10 + digit as usize;
        }

        joltage += value;
    }

    joltage
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_data()?;

    println!("The solution for the first part is {}", part_one(&input));
    println!("The solution for the second part is {}", part_two(&input));
    
    Ok(())
}
