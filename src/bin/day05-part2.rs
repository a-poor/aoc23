use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;
use regex::Regex;

struct SeedRange {
    start: usize,
    width: usize,
}

impl SeedRange {
    fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.start + self.width
    }
}

fn parse_seeds(line: &str) -> Result<Vec<SeedRange>> {
    let re = Regex::new(r"seeds: ([0-9 ]+)$").unwrap();
    let caps = re
        .captures(line)
        .ok_or(anyhow!("No capture match for line \"{}\"", line))?;
    let mut res = caps
        .get(1)
        .ok_or(anyhow!("No captures found"))?
        .as_str()
        .split_whitespace()
        .map(|s| {
            s.parse::<usize>().map_err(|err| {
                anyhow::Error::from(err).context(format!("Failed to parse seed \"{}\"", s))
            })
        })
        .collect::<Result<Vec<_>>>()?
        .chunks(2)
        .enumerate()
        .map(|(i, chunk)| {
            let start = *chunk
                .get(0)
                .ok_or(anyhow!("Error getting 1st num in {}th chunk of 2", i))?;
            let width = *chunk
                .get(1)
                .ok_or(anyhow!("Error getting 2nd num in {}th chunk of 2", i))?;
            Ok(SeedRange { start, width })
        })
        .collect::<Result<Vec<_>>>()?;
    res.sort_by(|a, b| a.start.cmp(&b.start));
    Ok(res)
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

    fn was_in(&self, pos: usize) -> bool {
        pos >= self.dest_start && pos < self.dest_start + self.width
    }

    fn unmap(&self, pos: usize) -> usize {
        if !self.was_in(pos) {
            return pos;
        }
        let offset = pos - self.dest_start;
        self.src_start + offset
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Mapping {
    src: String,
    dest: String,
    rules: Vec<MapRule>,
}

impl Mapping {
    fn parse(lines: &Vec<String>, sort_src: bool) -> Result<Self> {
        let (src, dest) = parse_map_type(&lines[0])?;
        let mut rules = lines[1..]
            .iter()
            .map(|line| MapRule::parse(line))
            .collect::<Result<Vec<_>>>()?;
        if sort_src {
            rules.sort_by(|a, b| a.src_start.cmp(&b.src_start));
        } else {
            rules.sort_by(|a, b| a.dest_start.cmp(&b.dest_start));
        }
        Ok(Self { src, dest, rules })
    }

    fn unmap(&self, pos: usize) -> usize {
        for rule in &self.rules {
            if rule.was_in(pos) {
                return rule.unmap(pos);
            }
        }
        pos
    }
}

fn main() -> Result<()> {
    // Load and group the input lines...
    let input_groups =
        load_input_lines_by_name(file!())?
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
        .map(|lines| Mapping::parse(lines, false))
        .collect::<Result<Vec<_>>>()?;

    // Now, in part 2, since we're operating
    // over a list of ranges, the simple version
    // would be to walk the full ranges but that's
    // probably going to take a lot of walking
    // (I think that's around 15 billion) numbers.
    //
    // Instead, maybe we can walk backwards.
    //
    // Starting from the final mapping, we can walk
    // from the ideal ending point (aka the lowest
    // destination value -- I think that's 0) and
    // try to walk it backwards to the starting point.
    //
    // One other question, though, is whether we can
    // work with ranges of values? Or do we need to
    // work with individual values?
    //
    // Let's start with the single loop and see how
    // slow that actually is.

    // Get the stopping point...
    let stop = seeds
        .iter()
        .map(|seed| seed.start + seed.width)
        .max()
        .ok_or(anyhow!("No seeds found"))?;

    // Start looping...
    for i in 0..stop {
        // Unmap it to the start...
        let out = mappings
            .iter()
            .rev()
            .fold(i, |acc, mapping| mapping.unmap(acc));

        // Check if it's in the initial seeds ranges...
        if seeds.iter().any(|seed| seed.contains(out)) {
            println!("Found a match: {}", i);
            return Ok(());
        }
    }
    
    println!("Oh no! Got to the end without finding a match!");
    Ok(())
}
