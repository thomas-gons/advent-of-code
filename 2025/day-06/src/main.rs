use std::fs;
use regex::Regex;

const DATA_PATH: &str = "data/cephalopod-math-pb-input.txt";

fn read_data() -> Result<String, std::io::Error> {
    fs::read_to_string(DATA_PATH)
}


fn part_one(input: &str) -> usize {
    let mut operands: Vec<Vec<usize>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in input.lines() {
        let values = line.split_whitespace();
        for (i, value) in values.enumerate() {
            if let Ok(num) = value.parse::<usize>() {
                if operands.len() <= i {
                    operands.push(Vec::new());
                }
                operands[i].push(num);
            } else if value.len() == 1 {
                let op = value.chars().next().unwrap();
                if "+*".contains(op) {
                    operators.push(op);
                } else {
                    panic!("unknown operator: {op}")
                }
            }
        }
    }
    let mut grand_total = 0usize;
    let operands_per_operation = operands[0].len();
    for (i, operator) in operators.iter().enumerate() {
        match operator {
            '+' => {
                let mut total = 0usize;
                for j in 0..operands_per_operation {
                    total += operands[i][j];
                }
                grand_total += total;
            }
            '*' => {
                let mut total = 1usize;
                for j in 0..operands_per_operation {
                    total *= operands[i][j];
                }
                grand_total += total;
            }
            _ => panic!("Bad operator: {operator}")
        }

    }
    grand_total
}

fn part_two(input: &str) -> usize {
    let mut operands_str: Vec<Vec<&str>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    let mut pos = 0;
    let last_line = input.lines().last().unwrap();
    let re = Regex::new(r"^[*+]\s*").unwrap();
    while pos < last_line.len() {
        let slice = &last_line[pos..];

        // the operator is always place at the first position of the column
        // so we can use it to determine the column size
        let column_size = re
            .find(slice)
            .map(|mat| {
                let mut size = mat.end() - mat.start();
                if pos + size < last_line.len() {
                    size -= 1;
                }
                size
            }).unwrap();

        let mut values = Vec::<&str>::new();
        for line in input.lines() {
            let value = &line[pos..pos+column_size];
            let trimmed = value.trim();

            // determine if it's a number or an operator robustly
            if trimmed.parse::<usize>().is_ok() {
                values.push(value);
            } else if trimmed.len() == 1 {
                let op = trimmed.chars().next().unwrap();
                if "+*".contains(op) {
                    operators.push(op);
                } else {
                    panic!("unknown operator: {op}");
                }
            }
        }
        operands_str.push(values);

        // skip number and next whitespace
        pos += column_size + 1;
    }


    let mut grand_total = 0usize;
    let order = operands_str[0].len();
    for (i, operator) in operators.iter().enumerate() {
        // reconstruct operands from strings using cephalopod math
        // each digit in the operand strings represents a different operand
        // so we need to read them column by column from top to bottom ignoring spaces
        //
        // for example:
        // 64 
        // 23 
        // 314
        // +  
        // 
        // becomes 623 + 431 + 4 = 1058
        
        let mut operands: Vec<usize> = Vec::new();
        let curr_operands_str = operands_str.get(i).unwrap();
        let operands_per_operation = curr_operands_str[0].len();
        for j in (0..operands_per_operation).rev() {
            let mut value = 0usize;
            for k in 0..order {
                let digit_str = curr_operands_str[k].chars().nth(j).unwrap().to_string();
                if let Ok(digit) = digit_str.parse::<usize>() {
                    value = value * 10 + digit;
                }
            }
            operands.push(value);
        }

        match operator {
            '+' => {
                let mut total = 0usize;
                for j in 0..operands_per_operation {
                    total += operands[j];
                }
                grand_total += total;
            }
            '*' => {
                let mut total = 1usize;
                for j in 0..operands_per_operation {
                    total *= operands[j];
                }
                grand_total += total;
            }
            _ => panic!("Bad operator: {operator}")
        }

    }
    
    grand_total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_data()?;

    println!("The solution for the first part is: {}", part_one(&input));
    println!("The solution for the second part is: {}", part_two(&input));

    Ok(())
}
