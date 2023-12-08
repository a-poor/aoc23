use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;
use regex::Regex;
use std::collections::HashMap;

fn parse_line(line: &str) -> Result<(String, String, String)> {
    let re = Regex::new(r"([A-Za-z0-9]+) = \(([A-Za-z0-9]+), ([A-Za-z0-9]+)\)").unwrap();
    let caps = re.captures(line).ok_or(anyhow!("Invalid line: {}", line))?;
    let id = caps
        .get(1)
        .ok_or(anyhow!("Regex didn't match against line \"{}\"", line))?
        .as_str()
        .to_string();
    let left = caps
        .get(2)
        .ok_or(anyhow!("Regex didn't match against line \"{}\"", line))?
        .as_str()
        .to_string();
    let right = caps
        .get(3)
        .ok_or(anyhow!("Regex didn't match against line \"{}\"", line))?
        .as_str()
        .to_string();
    Ok((id, left, right))
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(dists: Vec<usize>) -> usize {
    let mut lcm = dists[0];
    for i in 1..dists.len() {
        lcm = (lcm * dists[i]) / gcd(lcm, dists[i]);
    }
    lcm
}

fn main() -> Result<()> {
    // Parse the input data...
    let lines = load_input_lines_by_name(file!())?;
    // let lines = vec![
    //     "LR".to_string(),
    //     "".to_string(),
    //     "11A = (11B, XXX)".to_string(),
    //     "11B = (XXX, 11Z)".to_string(),
    //     "11Z = (11B, XXX)".to_string(),
    //     "22A = (22B, XXX)".to_string(),
    //     "22B = (22C, 22C)".to_string(),
    //     "22C = (22Z, 22Z)".to_string(),
    //     "22Z = (22B, 22B)".to_string(),
    //     "XXX = (XXX, XXX)".to_string(),
    // ];
    let directions = lines.get(0).ok_or(anyhow!("No input"))?;
    let nodes = lines[2..]
        .into_iter()
        .map(|l| parse_line(l))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(
            HashMap::<String, (String, String)>::new(),
            |mut acc, (id, left, right)| {
                acc.insert(id, (left, right));
                acc
            },
        );

    // Find the starting points (nodes that end
    // with an "A")...
    let starting_points: Vec<String> = nodes
        .iter()
        .filter(|(id, _)| id.ends_with("A"))
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>();
    println!("Found {} starting points", starting_points.len());

    // For each of those starting points, find out how
    // many steps it takes to get to the end (Z)...
    let dists = starting_points
        .iter()
        .map(|start| {
            // Create a running node...
            let mut node = start.clone();

            // Still define the shared state variables...
            let mut i: usize = 0;
            let mut count: usize = 0;

            // Now, start the loop for all tracks simultaneously.
            // End the loop when all tracks end with a "Z"
            // (simultaneously).
            while !node.ends_with('Z') {
                // Get the next direction...
                let direction = directions
                    .chars()
                    .nth(i)
                    .ok_or(anyhow!("No direction at {}", count))
                    .unwrap();

                // Move the nodes in each track...
                let (left, right) = nodes.get(&node).ok_or(anyhow!("No node {}", node)).unwrap();
                node = match direction {
                    'L' => left.clone(),
                    'R' => right.clone(),
                    _ => panic!("Invalid direction {}", direction),
                };

                // Increment the counters...
                count += 1;
                i = if i == directions.len() - 1 { 0 } else { i + 1 };
            }
            count
        })
        .collect::<Vec<_>>();
    println!("Distances calculated.");

    // Find the LCM of all the distances...
    let dist_lcm = lcm(dists);
    let dist_fmt = {
        let mut n = dist_lcm;
        let mut parts = Vec::new();
        while n > 0 {
            parts.push(format!("{:03}", n % 1000));
            n /= 1000;
        }
        parts.reverse();
        parts.join(",").trim_start_matches('0').to_string()
    };
    println!("LCM = {}", dist_fmt);
    Ok(())
}
