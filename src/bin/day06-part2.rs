use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

#[derive(Debug)]
struct RaceInfo {
    time: usize,
    record: usize,
}

fn parse_input(lines: &Vec<String>) -> Result<RaceInfo> {
    let time = lines
        .get(0)
        .ok_or(anyhow!("Input didn't have a first line"))?
        .split(":")
        .collect::<Vec<_>>()
        .get(1)
        .ok_or(anyhow!("Couldn't get the 2nd part of the first line"))?
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .map_err(anyhow::Error::from)?;
    let record = lines
        .get(1)
        .ok_or(anyhow!("Input didn't have a second line"))?
        .split(":")
        .collect::<Vec<_>>()
        .get(1)
        .ok_or(anyhow!("Couldn't get the 2nd part of the second line"))?
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .map_err(anyhow::Error::from)?;
    Ok(RaceInfo { time, record })
}

fn get_dist(charge_time: usize, total_time: usize) -> usize {
    let rem = if total_time > charge_time {
        total_time - charge_time
    } else {
        0
    };
    charge_time * rem
}

fn get_first_winner(ri: &RaceInfo) -> Result<usize> {
    for i in 1..=ri.time {
        let dist = get_dist(i, ri.time);
        if dist > ri.record {
            return Ok(i);
        }
    }
    Err(anyhow!("No winner found"))
}

fn get_last_winner(ri: &RaceInfo) -> Result<usize> {
    for i in (1..=ri.time).rev() {
        let dist = get_dist(i, ri.time);
        if dist > ri.record {
            return Ok(i);
        }
    }
    Err(anyhow!("No winner found"))
}

fn get_win_margin(ri: &RaceInfo) -> Result<usize> {
    let first = get_first_winner(ri)?;
    let last = get_last_winner(ri)?;
    Ok(last - first + 1)
}

fn main() -> Result<()> {
    // Parse the input data...
    let input_lines = load_input_lines_by_name(file!())?;
    let ri = parse_input(&input_lines)?;

    // Calculate each race's win margin...
    let wm = get_win_margin(&ri)?;

    // Multiply all the win margins together...
    println!("Result: {}", wm);

    Ok(())
}
