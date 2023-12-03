use anyhow::{Result, Context};
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

#[derive(Debug)]
struct Symbol {
    pos: Point,
    sym: char,
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
    let mut symbols = Vec::<Symbol>::new();

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
                    symbols.push(Symbol{
                        pos: Point::new(j, i),
                        sym: c,
                    });

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
    // - Filter down to just the '*' symbols.
    // - Filter down to the symbols with exactly
    //   two adjacent numbers (and capture those)
    // - Calculate the "gear ratio" using the nums
    // - Sum the "gear ratio"
    let res = symbols
        .into_iter()
        .filter(|sym| sym.sym == '*')
        .filter_map(|sym| {
            // Find the adjacent numbers...
            let adj = numbers
                .iter()
                .filter_map(|num| {
                    if !num.is_adjacent(&sym.pos) {
                        return None
                    }
                    Some(num.num)
                })
                .collect::<Vec<_>>();
            
            // If there aren't *excatly* two, stop...
            if adj.len() != 2 {
                return None;
            }

            // Return the gear ratio...
            adj.into_iter().reduce(|acc, n| acc * n)
        })
        .fold(0, |acc, gr| acc + gr)
        ;


    println!("Done. Sum = {}", res);

    // Done!
    Ok(())
}


