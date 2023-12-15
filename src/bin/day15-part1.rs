use anyhow::{anyhow,Result};
use aoc23::load_input_by_name;

fn get_ascii_code(c: char) -> u32 {
    c as u32
}

fn hash_char(state: u32, c: char) -> u32 {
    let ascii_code = get_ascii_code(c);
    let next = state + ascii_code;
    let next = next * 17;
    let next = next % 256;
    next
}

fn hash_string(s: &str) -> u32 {
    let mut state = 0;
    for c in s.chars() {
        state = hash_char(state, c);
    }
    state
}

fn main() -> Result<()> {
    let input = load_input_by_name(file!())?;
    let parts = input
        .split(",")
        .collect::<Vec<_>>();
    let scores = parts
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| hash_string(s))
        .collect::<Vec<_>>();
    let total = scores
        .iter()
        .sum::<u32>();
    println!("Total: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ascii_code() {
        assert_eq!(get_ascii_code('a'), 97);
        assert_eq!(get_ascii_code('A'), 65);
        assert_eq!(get_ascii_code('1'), 49);
        assert_eq!(get_ascii_code('!'), 33);
    }
    
    #[test]
    fn test_hash_char() {
        assert_eq!(hash_char(  0, 'H'), 200);
        assert_eq!(hash_char(200, 'A'), 153);
        assert_eq!(hash_char(153, 'S'), 172);
        assert_eq!(hash_char(172, 'H'), 52);
    }
    
    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("rn=1"), 30);
        assert_eq!(hash_string("cm-"), 253);
        assert_eq!(hash_string("qp=3"), 97);
    }
}

