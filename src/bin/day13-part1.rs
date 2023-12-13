use anyhow::{anyhow, Result};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Square {
    Ash,
    Rock,
}

impl TryFrom<char> for Square {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Square::Ash),
            '#' => Ok(Square::Rock),
            _ => Err(anyhow!("Invalid square: {}", value)),
        }
    }
}

fn compare_sq_vec(a: &Vec<Square>, b: &Vec<Square>) -> bool {
    if a.len() != b.len() {
        return false
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false
        }
    }
    true
}

#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    grid: Vec<Vec<Square>>,
}

impl Pattern {
    fn new(grid: Vec<Vec<Square>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Self { width, height, grid }
    }

    fn parse(input: &Vec<String>) -> Result<Self> {
        let grid = input
            .iter()
            .map(|line| line
                .chars()
                .map(|c| Square::try_from(c))
                .collect::<Result<Vec<_>>>()
            ).collect::<Result<Vec<_>>>()?;
        Ok(Self::new(grid))
    }

    fn col(&self, i: i32) -> Option<Vec<Square>> {
        if i < 0 || i >= self.width as i32 {
            return None
        }
        Some(self.grid
            .iter()
            .map(|row| row[i as usize])
            .collect::<Vec<Square>>())
    }
    
    fn row(&self, i: i32) -> Option<Vec<Square>> {
        if i < 0 || i >= self.height as i32 {
            return None
        }
        Some(self.grid
            .get(i as usize)?
            .iter()
            .map(|&square| square)
            .collect::<Vec<Square>>())
    }

    fn cols_match(&self, i: i32, j: i32) -> bool {
        let ca = self.col(i);
        let cb = self.col(j);
        if ca.is_none() || cb.is_none() {
            return true
        }
        compare_sq_vec(&ca.unwrap(), &cb.unwrap())
    }
    
    fn rows_match(&self, i: i32, j: i32) -> bool {
        let ra = self.row(i);
        let rb = self.row(j);
        if ra.is_none() || rb.is_none() {
            return true
        }
        compare_sq_vec(&ra.unwrap(), &rb.unwrap())
    }

    fn get_col_reflection(&self) -> Option<usize> {
        // Iterate through the mirror match points...
        //
        // Note: We can possibly speed this up by
        // getting matches once, ahead of time.
        //
        // Also, we're doing extra iterations here.
        // I can come back and optimize this later.
        'outer: for i in 0..self.width-1 {
            for j in 0..self.width {
                let i = i as i32;
                let j = j as i32;
                if !self.cols_match(i-j, i+1+j) {
                    continue 'outer
                }
            }
            return Some(i)
        }
        None
    }
    
    fn get_row_reflection(&self) -> Option<usize> {
        // Iterate through the mirror match points...
        //
        // Note: We can possibly speed this up by
        // getting matches once, ahead of time.
        //
        // Also, we're doing extra iterations here.
        // I can come back and optimize this later.
        'outer: for i in 0..self.height-1 {
            for j in 0..self.height {
                let i = i as i32;
                let j = j as i32;
                if !self.rows_match(i-j, i+1+j) {
                    continue 'outer
                }
            }
            return Some(i)
        }
        None
    }

    fn summarize(&self) -> usize {
        let col_sum = self.get_col_reflection()
            .map(|i| i+1)
            .unwrap_or(0);
        let row_sum = self.get_row_reflection()
            .map(|i| i+1)
            .unwrap_or(0);
        col_sum + (row_sum * 100)
    }
}

fn main() -> Result<()> {
    // Load in the input data...
    let input = aoc23::load_input_lines_by_name(file!())?;

    // Split the vec of input lines on empty lines...
    let patterns = input
        .split(|line| line.is_empty())
        .map(|lines| Pattern::parse(&lines.to_vec()))
        .collect::<Result<Vec<_>>>()?;

    // Summarize the patterns...
    let summary = patterns
        .iter()
        .map(|pattern| pattern.summarize())
        .collect::<Vec<_>>();

    // Get the summary sum...
    let sum = summary.iter().sum::<usize>();
    println!("sum: {}", sum);

    Ok(())
}

