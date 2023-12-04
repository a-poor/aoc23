use regex::Regex;
use anyhow::{anyhow, Result, Context};
use std::collections::HashMap;
use aoc23::load_input_lines_by_name;

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Card {
    id: usize,
    winning_numbers: HashMap<usize, usize>,
    my_numbers: HashMap<usize, usize>,
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
                let prev = acc
                    .get(&n)
                    .unwrap_or(&0);
                acc.insert(n, *prev + 1);
                acc
            })
            ;
        let my_numbers = caps
            .get(3)
            .ok_or(anyhow!("Invalid line - capture group 3 not found"))?
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<usize>().map_err(anyhow::Error::msg))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(HashMap::<usize, usize>::new(), |mut acc, n| {
                let prev = acc
                    .get(&n)
                    .unwrap_or(&0);
                acc.insert(n, *prev + 1);
                acc
            })
            ;
        
        // Return the new card...
        Ok(Self {
            id,
            winning_numbers,
            my_numbers,
        })
    }

    fn score(&self) -> usize {
        let count = self.winning_numbers
            .iter()
            .map(|(k, _)| self.my_numbers.contains_key(k))
            .fold(0, |acc, b| if b { acc + 1 } else { acc })
            ;
        if count == 0 { 0 } else { 2_usize.pow(count-1) }
    }
}

fn main() -> Result<()> {
    // Load the input data...
    let input_lines = load_input_lines_by_name(file!())?;

    let res = input_lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            Card::parse(&line)
                .context(format!("Error parsing line {}", i))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|card| card.score())
        .sum::<usize>();
    println!("Done. Result: {}", res);

    Ok(())
}
