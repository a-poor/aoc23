use anyhow::{anyhow, Result};
use regex::Regex;

/// Loads the input data for the `d`th day and
/// returns it as a single, raw `String`.
pub fn load_input(d: u8) -> Result<String> {
    let p = format!("data/{:02}.txt", d);
    let s = std::fs::read_to_string(p)?;
    Ok(s)
}

pub fn load_input_lines(d: u8) -> Result<Vec<String>> {
    let raw = load_input(d)?;
    let lines = raw.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    Ok(lines)
}

fn parse_filename(name: &str) -> Result<u8> {
    let re = Regex::new(r"day(\d{2})-part\d\.rs")?;
    let caps = re
        .captures(name)
        .ok_or(anyhow::anyhow!("Invalid filename: {}", name))?;
    caps.get(1)
        .ok_or(anyhow!("Regex didn't match against file \"{}\"", name))?
        .as_str()
        .parse::<u8>()
        .map_err(|_| anyhow!("Failed to parse day from filename \"{}\"", name))
}

pub fn load_input_by_name(name: &str) -> Result<String> {
    let d = parse_filename(name)?;
    load_input(d)
}

pub fn load_input_lines_by_name(name: &str) -> Result<Vec<String>> {
    let d = parse_filename(name)?;
    load_input_lines(d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_input() -> Result<()> {
        let data: String = load_input(1)?;
        assert!(data.len() > 0);
        Ok(())
    }

    #[test]
    fn test_load_input_lines() -> Result<()> {
        let data: Vec<String> = load_input_lines(1)?;
        assert!(data.len() > 0);
        Ok(())
    }

    #[test]
    fn test_parse_filename() -> Result<()> {
        assert_eq!(parse_filename("day01-part1.rs")?, 1);
        assert_eq!(parse_filename("day02-part2.rs")?, 2);
        assert_eq!(parse_filename("day25-part1.rs")?, 25);
        assert_eq!(parse_filename("foo/bar/baz/day01-part2.rs")?, 1);
        assert_eq!(parse_filename("/bar/baz/day01-part1.rs")?, 1);
        assert_eq!(parse_filename("./bar/baz/day01-part1.rs")?, 1);
        Ok(())
    }
}
