use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

fn parse_input_line(line: &str) -> Result<Vec<i32>> {
    line.trim()
        .split_whitespace()
        .map(|s| {
            s.parse::<i32>()
                .map_err(|e| anyhow!("Failed to parse input: {}", e))
        })
        .collect::<Result<Vec<_>>>()
}

fn find_dists(line: &Vec<i32>) -> Vec<i32> {
     let mut dists = Vec::new();
    for i in 1..line.len() {
        dists.push(line[i] - line[i - 1]);
    }
    dists
}

fn find_next_value(line: &Vec<i32>) -> i32 {
    // Find the initial distances...
    let mut last_vals = vec![line.last().unwrap().clone()];
    let mut dists = find_dists(line);
    last_vals.push(dists.last().unwrap().clone());
    while !dists.iter().all(|d| *d == 0) {
        dists = find_dists(&dists);
        last_vals.push(dists.last().unwrap().clone());
    }
    last_vals.iter().sum()
}

fn main() -> Result<()> {
    let data = load_input_lines_by_name(file!())?
        .iter()
        .map(|line| parse_input_line(line))
        .collect::<Result<Vec<_>>>()?;
    // let data = vec![
    //     vec![ 0, 3, 6, 9,12,15],
    //     vec![ 1, 3, 6,10,15,21],
    //     vec![10,13,16,21,30,45],
    // ];

    let results = data
        .iter()
        .map(|line| line.iter().rev().map(|n| *n).collect::<Vec<_>>())
        .map(|line| find_next_value(&line))
        .collect::<Vec<_>>();
    // for (line, result) in data.iter().zip(results.iter()) {
    //     println!("{:3} <- {:?}", result, line);
    // }
    let sum = results.iter().sum::<i32>();
    println!("Sum: {}", sum);

    Ok(())
}
