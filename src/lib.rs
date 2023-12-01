use anyhow::Result;

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
}
