use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

fn find_first_digit<I>(chars: I) -> Result<i32> 
where
    I: IntoIterator<Item = char>
{
    for c in chars {
        match c {
            '0'..='9' => match c.to_digit(10) {
                Some(n) => return i32::try_from(n).map_err(|err| err.into()),
                None => return Err(anyhow!("Couldn't convert char '{}' into digit", c)),
            },
            _ => {}
        }
    }
    Err(anyhow!("no digit found in line"))
}

fn parse_line(line: String) -> Result<i32> {
    let first = find_first_digit(line.chars())?;
    let last = find_first_digit(line.chars().rev())?;
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
        let fd = find_first_digit("a12".chars())?;
        assert_eq!(fd, 1);
        
        let fd = find_first_digit("9abc2".chars())?;
        assert_eq!(fd, 9);
        
        let fd = find_first_digit("abcd5".chars())?;
        assert_eq!(fd, 5);
        
        let fd = find_first_digit("9abcd5".chars().rev())?;
        assert_eq!(fd, 5);
       
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

