use std::collections::HashMap;
use regex::Regex;
use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

fn parse_line(line: &str) -> Result<(String, String, String)> {
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)")
        .unwrap();
    let caps = re
        .captures(line)
        .ok_or(anyhow!("Invalid line: {}", line))?;
    let id = caps.get(1)
        .ok_or(anyhow!("Regex didn't match against line \"{}\"", line))?
        .as_str()
        .to_string();
    let left = caps.get(2)
        .ok_or(anyhow!("Regex didn't match against line \"{}\"", line))?
        .as_str()
        .to_string();
    let right = caps.get(3)
        .ok_or(anyhow!("Regex didn't match against line \"{}\"", line))?
        .as_str()
        .to_string();
    Ok((id, left, right))
}

fn main() -> Result<()> {
    // let lines = load_input_lines_by_name(file!())?;
    // let lines = vec![
    //     "RL".to_string(),
    //     "".to_string(),
    //     "AAA = (BBB, CCC)".to_string(),
    //     "BBB = (DDD, EEE)".to_string(),
    //     "CCC = (ZZZ, GGG)".to_string(),
    //     "DDD = (DDD, DDD)".to_string(),
    //     "EEE = (EEE, EEE)".to_string(),
    //     "GGG = (GGG, GGG)".to_string(),
    //     "ZZZ = (ZZZ, ZZZ)".to_string(),
    // ];
    let lines = vec![
        "LLR".to_string(),
        "".to_string(),
        "AAA = (BBB, BBB)".to_string(),
        "BBB = (AAA, ZZZ)".to_string(),
        "ZZZ = (ZZZ, ZZZ)".to_string(),
    ];
    let directions = lines.get(0).ok_or(anyhow!("No input"))?;
    let nodes = lines[2..]
        .into_iter()
        .map(|l| parse_line(l))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(HashMap::<String,(String,String)>::new(), |mut acc, (id, left, right)| {
            acc.insert(id, (left, right));
            acc
        });

    let mut node = "AAA".to_string();
    let mut count: usize = 0;
    while node != "ZZZ" {
        let (left, right) = nodes
            .get(&node)
            .ok_or(anyhow!("No node {}", node))?;
        let i = count % 2;
        let direction = directions
            .chars()
            .nth(i)
            .ok_or(anyhow!("No direction at {}", count))?;
        node = match direction {
            'L' => left.to_string(),
            'R' => right.to_string(),
            _ => return Err(anyhow!("Invalid direction {}", direction)),
        };
        count += 1;
    }

    println!("count: {}", count);

    Ok(())
}

