use std::cmp::Ordering;
use std::collections::HashSet;
use anyhow::Result;
use aoc23::load_input_lines_by_name;


// const EXPANSION_FACTOR: u64 = 10 as u64;
// const EXPANSION_FACTOR: u64 = 100 as u64;
const EXPANSION_FACTOR: u64 = 1e6 as u64;

#[derive(Debug,Hash,Eq,PartialEq,Clone,Copy,Ord)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
    
    fn newu(x: usize, y: usize) -> Self {
        Self::new(x as u64, y as u64)
    }

    fn dist(&self, other: &Self) -> u64 {
        let dx = if self.x > other.x { self.x - other.x } else { other.x - self.x };
        let dy = if self.y > other.y { self.y - other.y } else { other.y - self.y };
        dx + dy
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.y.cmp(&other.y) {
            Ordering::Equal => Some(self.x.cmp(&other.x)),
            o => Some(o),
        }
    }
}

fn main() -> Result<()> {
    // Load the input data and parse the points...
    let input = load_input_lines_by_name(file!())?;
    // let input = vec![
    //     "...#......".to_string(),
    //     ".......#..".to_string(),
    //     "#.........".to_string(),
    //     "..........".to_string(),
    //     "......#...".to_string(),
    //     ".#........".to_string(),
    //     ".........#".to_string(),
    //     "..........".to_string(),
    //     ".......#..".to_string(),
    //     "#...#.....".to_string(),
    // ]; 

    let points = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Point::new(x as u64, y as u64))
        })
        .collect::<HashSet<_>>();

    // Shift the points based on empty rows/columns...
    let points = {
        // Calculate the width and height of the grid...
        let height = input.len();
        let width = input[0].len();

        // Find the empty rows/columns...
        let empty_rows = (0..height)
            .filter(|y| (0..width).all(|x| !points.contains(&Point::newu(x, *y))))
            .map(|n| n as u64)
            .collect::<HashSet<_>>();
        let empty_cols = (0..width)
            .filter(|x| (0..height).all(|y| !points.contains(&Point::newu(*x, y))))
            .map(|n| n as u64)
            .collect::<HashSet<_>>();
        
        // Update the points by shifting them...
        points
            .iter()
            .map(|p| {
                let dx = empty_cols.iter().filter(|x| **x < p.x).count();
                let dx = dx as u64 * (EXPANSION_FACTOR - 1);
                let dy = empty_rows.iter().filter(|y| **y < p.y).count();
                let dy = dy as u64 * (EXPANSION_FACTOR - 1);
                Point::new(p.x + dx, p.y + dy)
            })
            .collect::<HashSet<_>>()
    };

    // Continue here...
    let dist_total = points
        .iter()
        .flat_map(|p| points
            .iter()
            .map(|q| (*p, *q))
        )
        .filter(|(p, q)| *p < *q)
        .map(|(p, q)| p.dist(&q))
        .sum::<u64>();
   
    println!("expansion_factor = {}", EXPANSION_FACTOR);
    println!("dist_total = {}", dist_total);
    Ok(())
}

