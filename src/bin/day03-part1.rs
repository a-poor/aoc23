use anyhow::{anyhow, Result, Context};
use aoc23::load_input_lines;

#[derive(Debug, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self{x, y}
    }
}

#[derive(Debug, Default)]
struct Number {
    start: Point,
    end: Point,
    num: usize,
}

impl Number {
    fn new(y: usize, start: usize, end: usize, num: usize) -> Self {
        Self {
            start: Point::new(start, y),
            end: Point::new(end, y),
            num,
        }
    }

    fn bound_tl(&self) -> Point {
        let x = if self.start.x > 0 { self.start.x - 1 } else { 0 };
        let y = if self.start.y > 0 { self.start.y - 1 } else { 0 };
        Point::new(x, y)
    }

    fn bound_br(&self) -> Point {
        Point::new(
            self.end.x+1,
            self.end.y+1,
        )
    }

    fn is_adjacent(&self, symbol: &Point) -> bool {
        let btl = self.bound_tl(); 
        let bbr = self.bound_br();
        symbol.x >= btl.x
            && symbol.x <= bbr.x
            && symbol.y >= btl.y
            && symbol.y <= bbr.y
    }
}

fn main() -> Result<()> {
    // Load the input data...
    let input_lines = load_input_lines(3)?;

    let mut numbers = Vec::<Number>::new();
    let mut symbols = Vec::<Point>::new();

    // Loop through the lines
    for (i, line) in input_lines.into_iter().enumerate() {
        // Have we seen the start of a number already? 
        let mut running_num: Option<usize> = None;
        
        // Start looping through the line...
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    // If a number was started, store it.
                    if let Some(start) = running_num {
                        let end = j - 1;
                        let num = line[start..j]
                            .parse::<usize>()
                            .context(format!(
                                "Failed to parse usize on line {} [{},{}] = \"{}\"",
                                i, start, end, &line[start..j], 
                            ))?;
                        numbers.push(Number::new(i, start, end, num));
                        running_num = None;
                    }
                },
                '0'..='9' => {
                    // If we haven't started a number yet,
                    // mark this as the starting point.
                    if running_num.is_none() {
                        running_num = Some(j);
                    }
                },
                _ => {
                    // Otherwise, this must be a symbol (right?)
                    
                    // Add this symbol to the vec
                    symbols.push(Point::new(j, i));

                    // Also, if a number was started, store it.
                    if let Some(start) = running_num {
                        let end = j - 1;
                        let num = line[start..j]
                            .parse::<usize>()
                            .context(format!(
                                "Failed to parse usize on line {} [{},{}] = \"{}\"",
                                i, start, end, &line[start..j], 
                            ))?;
                        numbers.push(Number::new(i, start, end, num));
                        running_num = None;
                    }
                },
            }
        }
        
        // Check if a number was started but not finished.
        // If so, store it.
        if let Some(start) = running_num {
            let j = line.len();
            let end = j - 1;
            let num = line[start..j]
                .parse::<usize>()
                .context(format!(
                    "Failed to parse usize on line {} [{},{}] = \"{}\"",
                    i, start, end, &line[start..j], 
                ))?;
            numbers.push(Number::new(i, start, end, num));
        }
    }

    // Now  we have numbers and symbols...
    //
    // Filter down to the numbers adjacent to symbols
    // and sum them.
    //
    // Note: There's going to be a lot of room for
    // improvement here but let's just start with 
    // something simple. Also, I haven't seen part 2
    // yet but I have a feeling that this might simplify
    // that part, too.
    let res = numbers
        .into_iter()
        .filter(|num| symbols.iter().any(|sym| num.is_adjacent(&sym)))
        .fold(0, |acc, next| acc + next.num)
        ;

    println!("Done. Sum = {}", res);

    // Done!
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() -> Result<()> {
        Ok(())
    }
}

