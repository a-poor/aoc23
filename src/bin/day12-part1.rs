use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

#[derive(Debug,Clone,PartialEq,Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for SpringState {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(SpringState::Operational),
            '#' => Ok(SpringState::Damaged),
            '?' => Ok(SpringState::Unknown),
            _ => Err(anyhow!("I have *literally* no idea what the char '{}' means", c))
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
struct Line {
    springs: Vec<SpringState>,
    damage_counts: Vec<usize>,
}

impl Line {
    fn new(springs: Vec<SpringState>, damage_counts: Vec<usize>) -> Self {
        Self { springs, damage_counts }
    }

    fn parse(line: &str) -> Result<Self> {
        let line = line.trim();
        let parts = line.split(" ").collect::<Vec<_>>();

        let springs = parts
            .get(0)
            .ok_or(anyhow!("No springs found in line: {}", line))?
            .chars()
            .map(|c| SpringState::try_from(c))
            .collect::<Result<Vec<_>>>()?;
        let damage_counts = parts
            .get(1)
            .ok_or(anyhow!("No damage counts found in line: {}", line))?
            .split(",")
            .map(|s| s
                .parse::<usize>()
                .map_err(anyhow::Error::msg)
            )
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(
            springs,
            damage_counts,
        ))
    }

    fn get_real_damage_counts(&self) -> Option<Vec<usize>> {
        let mut found = Vec::<usize>::new();
        let mut start: Option<usize> = None;

        for (i, s) in self.springs.iter().enumerate() {
            match *s {
                SpringState::Operational => {
                    if let Some(start_pos) = start {
                        found.push(i - start_pos);
                        start = None;
                    }
                },
                SpringState::Damaged => {
                    if start.is_none() {
                        start = Some(i);
                    }
                },
                SpringState::Unknown => {
                    return None;
                },
            }
        }
        if let Some(start_pos) = start {
            found.push(self.springs.len() - start_pos);
        }
        Some(found)
    }
    
    fn matches(&self, other: &Self) -> Option<bool> {
        let real_damage_counts = self.get_real_damage_counts()?;
        if real_damage_counts.len() != other.damage_counts.len() {
            return Some(false);
        }
        if real_damage_counts.iter().sum::<usize>() != other.damage_counts.iter().sum::<usize>() {
            return Some(false);
        }
        for (i, count) in real_damage_counts.iter().enumerate() {
            let other_count = other.damage_counts.get(i);
            if other_count.is_none() {
                return Some(false);
            }
            let other_count = other_count.unwrap();
            if *count != *other_count {
                return Some(false);
            }
        }
        Some(true)
    }

    fn with_set_next_spring(&self, state: SpringState) -> Self {
        let mut springs = self.springs.clone();
        for i in 0..springs.len() {
            if springs[i] == SpringState::Unknown {
                springs[i] = state;
                break;
            }
        }
        let mut line = self.clone();
        line.springs = springs;
        line
    }

    fn arrangement_count(&self) -> usize {
        let n_spaces = self.springs
            .iter()
            .filter(|s| **s == SpringState::Unknown)
            .count();
        let mut possible_routes = vec![self.clone()];
        for _ in 0..n_spaces {
            possible_routes = possible_routes
                .iter()
                .flat_map(|line| {
                    vec![
                        line.with_set_next_spring(SpringState::Operational),
                        line.with_set_next_spring(SpringState::Damaged),
                    ]
                })
                .collect::<Vec<_>>();
        }
        possible_routes
            .iter()
            .filter(|line| {
                let m = line.matches(self);
                if let Some(m) = m {
                    m
                } else {
                    false
                }
            })
            .count()
    }
}

fn main() -> Result<()> {
    let input = load_input_lines_by_name(file!())?;
    let lines = input
        .iter()
        .map(|line| Line::parse(line))
        .collect::<Result<Vec<_>>>()?;

    let total = lines
        .iter()
        .map(|line| line.arrangement_count())
        .sum::<usize>();
    
    println!("Total arrangements: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parse() {
        let cases = vec![
            (
                "???.### 1,1,3",
                Line::new(
                    vec![
                        SpringState::Unknown,
                        SpringState::Unknown,
                        SpringState::Unknown,
                        SpringState::Operational,
                        SpringState::Damaged,
                        SpringState::Damaged,
                        SpringState::Damaged,
                    ],
                    vec![1,1,3],
                )
            ),
            (
                "?#?#?#?#?#?#?#? 1,3,1,6",
                Line::new(
                    vec![
                        SpringState::Unknown,
                        SpringState::Damaged,
                        SpringState::Unknown,
                        SpringState::Damaged,
                        SpringState::Unknown,
                        SpringState::Damaged,
                        SpringState::Unknown,
                        SpringState::Damaged,
                        SpringState::Unknown,
                        SpringState::Damaged,
                        SpringState::Unknown,
                        SpringState::Damaged,
                        SpringState::Unknown,
                        SpringState::Damaged,
                        SpringState::Unknown,
                    ],
                    vec![1,3,1,6],
                )
            ),
        ];
        for (input, expected) in cases {
            let actual = Line::parse(input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_line_parse_count() {
        let cases = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];
        for (i, (input, expected)) in cases.into_iter().enumerate() {
            let line = Line::parse(input).expect(format!("Failed to parse line {}", i).as_str());
            let count = line.arrangement_count();
            assert_eq!(count, expected, "Expected {}, got {}", expected, count);
        }
    }
}

