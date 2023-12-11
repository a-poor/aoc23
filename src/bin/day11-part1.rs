use std::cmp::Ordering;
use std::collections::HashSet;
use anyhow::Result;
use aoc23::load_input_lines_by_name;

#[derive(Debug,Hash,Eq,PartialEq,Clone,Copy,Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Self) -> i32 {
        let dx = self.x as i32 - other.x as i32;
        let dy = self.y as i32 - other.y as i32;
        dx.abs() + dy.abs()
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
                .map(move |(x, _)| Point::new(x, y))
        })
        .collect::<HashSet<_>>();
    // println!("Input points:");
    // for i in 0..input.len() {
    //     for j in 0..input[i].len() {
    //         print!("{}", if points.contains(&Point::new(j, i)) { '#' } else { '.' });
    //     }
    //     println!();
    // }
    // println!();

    // Shift the points based on empty rows/columns...
    let points = {
        // Calculate the width and height of the grid...
        let height = input.len();
        let width = input[0].len();

        // Find the empty rows/columns...
        let empty_rows = (0..height)
            .filter(|y| (0..width).all(|x| !points.contains(&Point::new(x, *y))))
            .collect::<HashSet<_>>();
        let empty_cols = (0..width)
            .filter(|x| (0..height).all(|y| !points.contains(&Point::new(*x, y))))
            .collect::<HashSet<_>>();
        
        // Update the points by shifting them...
        points
            .iter()
            .map(|p| {
                Point::new(
                    p.x + empty_cols.iter().filter(|x| **x < p.x).count(),
                    p.y + empty_rows.iter().filter(|y| **y < p.y).count(),
                )
            })
            .collect::<HashSet<_>>()
    };
    // println!("Transformed input:");
    // for i in 0..=points.iter().map(|p| p.y).max().unwrap() {
    //     for j in 0..=points.iter().map(|p| p.x).max().unwrap() {
    //         print!("{}", if points.contains(&Point::new(j, i)) { '#' } else { '.' });
    //     }
    //     println!();
    // }
    // println!();

    // Continue here...
    let dist_total = points
        .iter()
        .flat_map(|p| points
            .iter()
            .map(|q| (*p, *q))
        )
        .filter(|(p, q)| *p < *q)
        .map(|(p, q)| p.dist(&q))
        .sum::<i32>();
    
    println!("dist_total = {}", dist_total);
    Ok(())
}

