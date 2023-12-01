use anyhow::{anyhow, Result};
use aoc23::load_input_lines;

fn parse_line(line: String) -> Result<i32> {
    todo!();
}

fn main() -> Result<()> {
    // Load the input data...
    let input_lines = load_input_lines(1)?;

    // Create a place to store the final count...
    let sum = input_lines
        .into_iter()
        .map(parse_line)
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .reduce(|a, b| a + b);

    // Print out the count!
    println!("Done. Sum = {:?}", sum);

    // Done!
    Ok(())
}
