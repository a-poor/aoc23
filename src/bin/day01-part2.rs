use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

const NUM_WORDS: [&'static str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight", 
    "nine",
];

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn find_first_digit(line: &str, rev: bool) -> Result<i32> {
    for (i, c) in line.chars().enumerate() {
        match c {
            '0'..='9' => match c.to_digit(10) {
                Some(n) => return i32::try_from(n).map_err(|err| err.into()),
                None => return Err(anyhow!("Couldn't convert char '{}' into digit", c)),
            },
            _ => {
                for (j, w) in NUM_WORDS.iter().enumerate() {
                    if (
                        !rev && line[..=i].ends_with(w)
                    ) || (
                        rev && line[..=i].ends_with(&reverse_string(w))
                    ) {
                        return i32::try_from(j + 1).map_err(|err| err.into());
                    }
                }
            },
        }
    }
    Err(anyhow!("no digit found in line"))
}

fn parse_line(line: String) -> Result<i32> {
    let first = find_first_digit(&line, false)?;
    let last = find_first_digit(&reverse_string(&line), true)?;
    Ok(10 * first + last)
}

fn parse_file(lines: Vec<String>) -> Result<i32> {
    lines
        .into_iter()
        .map(parse_line)
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .reduce(|a, b| a + b)
        .ok_or(anyhow!("no result found"))
}

fn main() -> Result<()> {
    // Load the input data...
    let input_lines = load_input_lines_by_name(file!())?;

    // Create a place to store the final count...
    let sum = parse_file(input_lines)?;

    // Print out the count!
    println!("Done. Sum = {:?}", sum);

    // Done!
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_digit() -> Result<()> {
        let s = "a1two2";
        let fd = find_first_digit(s, false)?;
        assert_eq!(fd, 1, "Failed with string: \"{}\"", s);
        
        let s = "owt1a";
        let fd = find_first_digit(s, true)?;
        assert_eq!(fd, 2, "Failed with string: \"{}\"", s);
        
        let s = "9abc2";
        let fd = find_first_digit(s, false)?;
        assert_eq!(fd, 9, "Failed with string: \"{}\"", s);
        
        let s = "abonecd5";
        let fd = find_first_digit(s, false)?;
        assert_eq!(fd, 1, "Failed with string: \"{}\"", s);
        
        let s = "ab5enocd";
        let fd = find_first_digit(s, true)?;
        assert_eq!(fd, 5, "Failed with string: \"{}\"", s);
        
        let s = "abcdeight";
        let fd = find_first_digit(s, false)?;
        assert_eq!(fd, 8, "Failed with string: \"{}\"", s);
       
        Ok(())
    }

    #[test]
    fn test_parse_line() -> Result<()> {
        let res = parse_line("a1bcde2f".to_string())?;
        assert_eq!(res, 12);

        Ok(())
    }

    #[test]
    fn test_example() -> Result<()> {
        let example_lines: Vec<String> = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ]
            .into_iter()
            .map(|s: &str| s.to_string())
            .collect();
        let res = parse_file(example_lines)?;
        assert_eq!(res, 142);
        Ok(())
    }
}

