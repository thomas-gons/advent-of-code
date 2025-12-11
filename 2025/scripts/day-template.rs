const DATA_PATH: &str = "day-XX/data/input.txt";

fn part_one(_input: &str) -> usize {
    aoc::not_implemented()
}

fn part_two(_input: &str) -> usize {
    aoc::not_implemented()
}

fn main() -> anyhow::Result<()> {
    let input = aoc::read_input(DATA_PATH)?;
    aoc::run(part_one, part_two, &input);
    Ok(())
}