use anyhow::{anyhow, Context, Result};
use aoc23::load_input_lines_by_name;
use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
struct Card {
    id: usize,
    win_count: usize,
}

impl Card {
    fn parse(line: &str) -> Result<Self> {
        // Build a regex to parse the line...
        let re = Regex::new(r"^Card\s+(\d+): ([0-9 ]+) \| ([0-9 ]+)$").unwrap();
        let caps = re.captures(line).ok_or(anyhow!("Invalid line"))?;

        // Parse the id, winners, and my nums...
        let id = caps
            .get(1)
            .ok_or(anyhow!("Invalid line - capture group 1 not found"))?
            .as_str()
            .parse::<usize>()?;
        let winning_numbers = caps
            .get(2)
            .ok_or(anyhow!("Invalid line - capture group 2 not found"))?
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<usize>().map_err(anyhow::Error::msg))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(HashMap::<usize, usize>::new(), |mut acc, n| {
                let prev = acc.get(&n).unwrap_or(&0);
                acc.insert(n, *prev + 1);
                acc
            });
        let my_numbers = caps
            .get(3)
            .ok_or(anyhow!("Invalid line - capture group 3 not found"))?
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<usize>().map_err(anyhow::Error::msg))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(HashMap::<usize, usize>::new(), |mut acc, n| {
                let prev = acc.get(&n).unwrap_or(&0);
                acc.insert(n, *prev + 1);
                acc
            });

        let win_count = winning_numbers
            .iter()
            .map(|(k, _)| my_numbers.contains_key(k))
            .fold(0, |acc, b| if b { acc + 1 } else { acc });

        // Return the new card...
        Ok(Self { id, win_count })
    }
}

fn main() -> Result<()> {
    // Load the input data...
    let input_lines = load_input_lines_by_name(file!())?;

    // Parse the input data...
    let cards = input_lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| Card::parse(&line).context(format!("Error parsing line {}", i)))
        .collect::<Result<Vec<_>>>()?;

    // Create a queue of cards...
    let mut counts = cards.iter().map(|_| 1_usize).collect::<Vec<_>>();

    // Now start looping through the cards...
    for (i, c) in cards.into_iter().enumerate() {
        // Now, for each n coppies of the i-th card,
        // add n copies to the next m cards, where m
        // is the number of winners for this card...
        for j in 1..=c.win_count {
            counts[i + j] += counts[i];
        }
    }

    // Count the total number of cards...
    let res = counts.into_iter().sum::<usize>();

    // Print out the result...
    println!("Done. Result: {}", res);
    Ok(())
}
