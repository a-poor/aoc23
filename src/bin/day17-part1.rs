#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Self) -> usize {
        let dx = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        dx + dy
    }

    fn step(&self, dir: &Direction, w: usize, h: usize) -> Option<Self> {
        match dir {
            Direction::Left => {
                if self.x > 0 {
                    Some(Self::new(self.x - 1, self.y))
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < w - 1 {
                    Some(Self::new(self.x + 1, self.y))
                } else {
                    None
                }
            }
            Direction::Up => {
                if self.y > 0 {
                    Some(Self::new(self.x, self.y - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < h - 1 {
                    Some(Self::new(self.x, self.y + 1))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Path {
    points: Vec<Point>,
    cost: usize,
    dir: Option<Direction>,
    n_straight: usize,
}

impl Path {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            cost: 0,
            dir: None,
            n_straight: 0,
        }
    }

    fn est_remaining_cost(&self, end: &Point) -> usize {
        let last_point = self
            .last_point()
            .expect("Can't estimate remaining cost for empty path");
        last_point.dist(end)
    }

    fn est_total_cost(&self, end: &Point) -> usize {
        self.cost + self.est_remaining_cost(end)
    }

    fn last_point(&self) -> Option<Point> {
        self.points
            .last()
            .map(|p| p.clone())
    }

    fn with_next(&self, next: &Point, cost: usize, dir: Direction) -> Self {
        let mut new_path = self.clone();
        new_path.points.push(next.clone());
        new_path.cost += cost;
        new_path.dir = Some(dir);
        if new_path.dir == self.dir {
            new_path.n_straight += 1;
        } else {
            new_path.n_straight = 0;
        }
        new_path
    }

    /// Gets the neighbor paths to the current path.
    ///
    /// Note that a neighbor must follow the following rules:
    /// - Can't go off the edge of the board
    /// - Can't return to the square it just left
    /// - Can't go straight for more than 3 squares in a row before turning
    fn get_neighbors(&self, grid: &Vec<Vec<usize>>) -> Vec<Path> {
        let mut neighbors = Vec::new();
        let last_point = self.last_point()
            .expect("Can't get neighbors for empty path");
        
        // Get the grid dimensions...
        let height = grid.len();
        let width = grid[0].len();

        // Add "up" neighbor...
        if last_point.y > 0 
            && self.dir != Some(Direction::Down) 
            && !(
                self.dir == Some(Direction::Up)
                && self.n_straight > 2
            )
        {
            let next_point = Point::new(last_point.x, last_point.y - 1);
            let next_square = grid[next_point.y][next_point.x];
            let next_path = self.with_next(
                &next_point, 
                next_square, 
                Direction::Up,
            );
            neighbors.push(next_path);
        }

        // Add "down" neighbor...
        if last_point.y < height-1
            && self.dir != Some(Direction::Up) 
            && !(
                self.dir == Some(Direction::Down)
                && self.n_straight > 2
            )
        {
            let next_point = Point::new(last_point.x, last_point.y + 1);
            let next_square = grid[next_point.y][next_point.x];
            let next_path = self.with_next(
                &next_point,
                next_square,
                Direction::Down,
            );
            neighbors.push(next_path);
        }

        // Add "left" neighbor...
        if last_point.x > 0 
            && self.dir != Some(Direction::Right) 
            && !(
                self.dir == Some(Direction::Left)
                && self.n_straight > 2
            )
        {
            let next_point = Point::new(last_point.x - 1, last_point.y);
            let next_square = grid[next_point.y][next_point.x];
            let next_path = self.with_next(
                &next_point,
                next_square,
                Direction::Left,
            );
            neighbors.push(next_path);
        }

        // Add "right" neighbor...
        if last_point.x < width-1
            && self.dir != Some(Direction::Left) 
            && !(
                self.dir == Some(Direction::Right)
                && self.n_straight > 2
            )
        {
            let next_point = Point::new(last_point.x + 1, last_point.y);
            let next_square = grid[next_point.y][next_point.x];
            let next_path = self.with_next(
                &next_point,
                next_square,
                Direction::Right,
            );
            neighbors.push(next_path);
        }

        // Return the neighbors...
        neighbors
    }
}

fn pop_best_path(paths: &mut HashSet<Path>, end: &Point) -> Option<Path> {
    let mut best_path = None;
    let mut best_cost = usize::MAX;
    for path in paths.iter() {
        let cost = path.est_total_cost(end);
        if cost < best_cost {
            best_path = Some(path.clone());
            best_cost = cost;
        }
    }
    if best_path.is_none() {
        return None;
    }
    let p = best_path?;
    paths.remove(&p);
    Some(p.clone())
}

fn find_best_route(grid: &Vec<Vec<usize>>, start: &Point, end: &Point) -> Option<Path> {
    // Create an open list with the starting point... 
    let mut open_set: HashSet<Path> = vec![Path {
        points: vec![start.clone()],
        cost: 0,
        dir: None,
        n_straight: 0,
    }]
        .into_iter()
        .collect();

    // println!("Starting loop...");

    // Start to loop...
    while open_set.len() > 0 {
        // Get the current best path in the open set
        //
        // (Note: Shouldn't be `None` since the loop checks
        // that `open_set` isn't empty)...
        let best_path = pop_best_path(&mut open_set, end)?;

        // Get the last point in the path.
        //
        // If that's the destination, we're done...
        let last_point = best_path.last_point()?;
        // println!("Path's last point: {:?}", last_point);
        if last_point == *end {
            return Some(best_path);
        }

        // Otherwise, get the neighbors...
        let neighbors = best_path.get_neighbors(grid);

        // Add the neighbors to the open set?
        for neighbor in neighbors {
            // println!("> Checking neighbor: {:?}", neighbor.last_point());

            // Get the neighbor's last point...
            let neighbor_last_point = neighbor.last_point();
            if neighbor_last_point.is_none() {
                // println!(">> WARN: Neighbor has no last point");
                continue;
            }
            let neighbor_last_point = neighbor_last_point.unwrap();

            // Is the neighbor's last point already in the path?
            let other_with_this_last_point = open_set
                .iter()
                .filter(|p| p.last_point() == Some(neighbor_last_point.clone()))
                .next();
            if let Some(other) = other_with_this_last_point {
                // println!(">> found neighbor with the same last point");
                // Is this neighbor's path better than the other path's?
                if neighbor.cost >= other.cost {
                    // println!(">> Neighbor already had a better cost. Not adding new.");
                    continue;
                }
            }

            // Otherwise, add the neighbor to the open set...
            // println!(">> Adding neighbor to open set");
            open_set.insert(neighbor);
        }
    }
    
    // If we've gotten here, we didn't find a path...
    None
}

fn main() -> Result<()> {
    let input = load_input_lines_by_name(file!())?;
    // let input = vec![
    //     "2413432311323".to_string(),
    //     "3215453535623".to_string(),
    //     "3255245654254".to_string(),
    //     "3446585845452".to_string(),
    //     "4546657867536".to_string(),
    //     "1438598798454".to_string(),
    //     "4457876987766".to_string(),
    //     "3637877979653".to_string(),
    //     "4654967986887".to_string(),
    //     "4564679986453".to_string(),
    //     "1224686865563".to_string(),
    //     "2546548887735".to_string(),
    //     "4322674655533".to_string(),
    // ];

    // Parse the input into a grid...
    let grid = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                    .ok_or(anyhow!("Can't convert char {} to a digit", c))
                    .map(|d| d as usize)
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    // Find the width and height...
    let height = grid.len();
    let width = grid[0].len();

    // Define the start and end points...
    let start_point = Point::new(0, 0);
    let end_point = Point::new(width - 1, height - 1);

    // Find the best route...
    let best_route = find_best_route(
        &grid, 
        &start_point,
        &end_point,
    );
    
    // Print out the best route...
    if let Some(p) = &best_route {
        println!("\nBest route:");
        for point in p.points.iter() {
            println!("  {:?}", point);
        }
        println!();
    }

    if let Some(p) = best_route {
        println!("Best route cost: {}", p.cost);
    } else {
        println!("No route found");
    }

    Ok(())
}
