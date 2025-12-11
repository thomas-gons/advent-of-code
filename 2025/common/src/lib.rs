use std::fs;
use anyhow::Result;

pub fn read_input(file_name: &str) -> Result<String> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

pub fn not_implemented() -> usize {
    eprintln!("\x1b[33mWarning: this part is not implemented yet!\x1b[0m");
    0
}

pub fn run<F1, F2, T1, T2>(part_one: F1, part_two: F2, input: &str)
where
    F1: Fn(&str) -> T1,
    F2: Fn(&str) -> T2,
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    let start = std::time::Instant::now();
    let res1 = part_one(input);
    println!("The solution for the first part is : {} (took {:?})\n", res1, start.elapsed());

    let start = std::time::Instant::now();
    let res2 = part_two(input);
    println!("The solution for the second part is : {} (took {:?})", res2, start.elapsed());
}
