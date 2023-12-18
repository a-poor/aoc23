use anyhow::Result;
use aoc23::load_input_lines_by_name;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    fn parse_hex(s: &str) -> Result<Self> {
        let red = u8::from_str_radix(&s[1..3], 16)
            .map_err(anyhow::Error::msg)?;
        let green = u8::from_str_radix(&s[3..5], 16)
            .map_err(anyhow::Error::msg)?;
        let blue = u8::from_str_radix(&s[5..7], 16)
            .map_err(anyhow::Error::msg)?;
        Ok(Self::new(red, green, blue))
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
    color: String,
}

impl Instruction {
    fn new(direction: Direction, distance: i32, color: String) -> Self {
        Self { direction, distance, color }
    }

    fn parse(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let direction = parts.next().ok_or_else(|| anyhow::Error::msg("missing direction"))?;
        let distance = parts.next().ok_or_else(|| anyhow::Error::msg("missing distance"))?;
        let color = parts.next().ok_or_else(|| anyhow::Error::msg("missing color"))?;
        Ok(Self::new(
            direction.chars().next().ok_or_else(|| anyhow::Error::msg("missing direction"))?,
            distance.parse::<i32>().map_err(anyhow::Error::msg)?,
            color.to_string(),
        ))
    }
}

fn main() -> Result<()> {
    // let _input = load_input_lines_by_name(file!())?;
    let input = vec![
        "R 6 (#70c710)".to_string(),
        "D 5 (#0dc571)".to_string(),
        "L 2 (#5713f0)".to_string(),
        "D 2 (#d2c081)".to_string(),
        "R 2 (#59c680)".to_string(),
        "D 2 (#411b91)".to_string(),
        "L 5 (#8ceee2)".to_string(),
        "U 2 (#caa173)".to_string(),
        "L 1 (#1b58a2)".to_string(),
        "U 2 (#caa171)".to_string(),
        "R 2 (#7807d2)".to_string(),
        "U 3 (#a77fa3)".to_string(),
        "L 2 (#015232)".to_string(),
        "U 2 (#7a21e3)".to_string(),
    ];

    Ok(())
}

