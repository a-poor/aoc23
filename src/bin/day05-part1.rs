#![allow(dead_code)]

use regex::Regex;
use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

fn parse_seeds(line: &str) -> Result<Vec<usize>> {
    let re = Regex::new(r"seeds: ([0-9 ]+)$").unwrap();
    let caps = re.captures(line)
        .ok_or(anyhow!("No capture match for line \"{}\"", line))?;
    caps
        .get(1)
        .ok_or(anyhow!("No captures found"))?
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<usize>()
            .map_err(|err| anyhow::Error::from(err)
                .context(format!("Failed to parse seed \"{}\"", s))
            )
        )
        .collect::<Result<Vec<_>>>()
}

fn parse_map_type(line: &str) -> Result<(String, String)> {
    let re = Regex::new(r"(.*)-to-(.*) map").unwrap();
    let caps = re.captures(line).ok_or(anyhow!("No captures found"))?;
    let src = caps
        .get(1)
        .ok_or(anyhow!("No captures found"))?
        .as_str()
        .to_string();
    let dest = caps
        .get(2)
        .ok_or(anyhow!("No captures found"))?
        .as_str()
        .to_string();
    Ok((src, dest))
}

#[derive(Debug)]
struct MapRule {
    dest_start: usize,
    src_start: usize,
    width: usize,
}

impl MapRule {
    fn parse(line: &str) -> Result<Self> {
        let parts = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()?;
        let dest_start = *parts.get(0).ok_or(anyhow!("No captures found (1)"))?;
        let src_start = *parts.get(1).ok_or(anyhow!("No captures found (2)"))?;
        let width = *parts.get(2).ok_or(anyhow!("No captures found (3)"))?;
        Ok(Self {        
            dest_start,
            src_start,
            width,
        })
    }

    fn is_in(&self, pos: usize) -> bool {
        pos >= self.src_start && pos < self.src_start + self.width
    }

    fn map(&self, pos: usize) -> usize {
        if !self.is_in(pos) {
            return pos;
        }
        let offset = pos - self.src_start;
        self.dest_start + offset
    }
}

#[derive(Debug)]
struct Mapping {
    src: String,
    dest: String,
    rules: Vec<MapRule>,
}

impl Mapping {
    fn parse(lines: &Vec<String>) -> Result<Self> {
        let (src, dest) = parse_map_type(&lines[0])?;
        let rules = lines[1..]
            .iter()
            .map(|line| MapRule::parse(line))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            src,
            dest,
            rules,
        })
    }

    fn map(&self, pos: usize) -> usize {
        for rule in &self.rules {
            if rule.is_in(pos) {
                return rule.map(pos);
            }
        }
        pos
    }
}

fn main() -> Result<()> {
    // Load and group the input lines...
    let input_groups = load_input_lines_by_name(file!())?
        .into_iter()
        .fold(Vec::new(), |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else if acc.is_empty() {
                acc.push(vec![line]);
            } else {
                let last = acc.last_mut().unwrap();
                last.push(line);
            }
            acc
        });

    // Parse the seeds...
    let seeds = parse_seeds(&input_groups[0][0])?;

    // Parse the mappings...
    let mappings = input_groups[1..]
        .iter()
        .map(|lines| Mapping::parse(lines))
        .collect::<Result<Vec<_>>>()?;

    let seeds = seeds
        .into_iter()
        .map(|seed| {
            let mut seed = seed;
            for mapping in &mappings {
                seed = mapping.map(seed);
            }
            seed
        })
        .collect::<Vec<_>>();
    let res = seeds.iter().min();
    println!("res: {:?}", res);

    Ok(())
}
