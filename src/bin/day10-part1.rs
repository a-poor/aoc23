use std::collections::HashMap;
use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

#[derive(Debug,Hash,Clone,Copy,PartialEq,Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn north(&self) -> Self {
        Self::new(self.x, self.y-1)
    }

    fn south(&self) -> Self {
        Self::new(self.x, self.y+1)
    }

    fn east(&self) -> Self {
        Self::new(self.x+1, self.y)
    }

    fn west(&self) -> Self {
        Self::new(self.x-1, self.y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Pipe {
    Start,
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'S' => Ok(Pipe::Start),
            '.' => Ok(Pipe::Ground),
            '|' => Ok(Pipe::NorthSouth),
            '-' => Ok(Pipe::EastWest),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            'F' => Ok(Pipe::SouthEast),
            '7' => Ok(Pipe::SouthWest),
            _ => Err(anyhow!("Invalid pipe character: {}", c))
        }
    }
}

/// Finds the starting point in the grid.
fn find_start(grid: &Vec<Vec<Pipe>>) -> Result<Point> {
    for (y, row) in grid.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if let Pipe::Start = pipe {
                return Ok(Point::new(x.try_into()?, y.try_into()?));
            }
        }
    }
    Err(anyhow!("No start found"))
}

/// Gets the pipe at the given point.
///
/// Returns `None` if the point is out of bounds.
fn get_point(grid: &Vec<Vec<Pipe>>, point: &Point) -> Option<Pipe> {
    if point.y < 0 || point.y >= grid.len() as i32 {
        return None;
    }
    let row = &grid[point.y as usize];
    if point.x < 0 || point.x >= row.len() as i32 {
        return None;
    }
    let val = row[point.x as usize];
    Some(val)
}

/// Checks if the two points touch.
fn touches(grid: &Vec<Vec<Pipe>>, p1: &Point, p2: &Point) -> bool {
    let c1 = get_connections(grid, p1);
    if let Some((c1, c2)) = c1 {
        c1 == *p2 || c2 == *p2
    } else {
        false
    }
}

/// Gets the connections for the given point.
///
/// Returns `None` if the point is out of bounds.
fn get_connections(grid: &Vec<Vec<Pipe>>, point: &Point) -> Option<(Point, Point)> {
    match get_point(grid, point)? {
        Pipe::Start => {
            // Get n/s/e/w points...
            let north = point.north();
            let south = point.south();
            let east = point.east();
            let west = point.west();

            // See which neighbors connect to this point...
            let nt = touches(grid, &north, point);
            let st = touches(grid, &south, point);
            let et = touches(grid, &east, point);
            let wt = touches(grid, &west, point);

            // Return the connections...
            if nt && st {
                return Some((north, south));
            }
            if et && wt {
                return Some((east, west));
            }
            if nt && et {
                return Some((north, east));
            }
            if nt && wt {
                return Some((north, west));
            }
            if st && et {
                return Some((south, east));
            }
            if st && wt {
                return Some((south, west));
            }
            panic!("Start point has no connections");
        },
        Pipe::NorthSouth => Some((
            point.north(),
            point.south(),
        )),
        Pipe::EastWest => Some((
            point.east(),
            point.west(),
        )),
        Pipe::NorthEast => Some((
            point.north(),
            point.east(),
         )),
        Pipe::NorthWest => Some((
            point.north(),
            point.west(),
        )),
        Pipe::SouthEast => Some((
            point.south(),
            point.east(),
        )),
        Pipe::SouthWest => Some((
            point.south(),
            point.west(),
        )),
        Pipe::Ground => None,
    }
}

fn main() -> Result<()> {
    let debug = false;

    // Load the input data and parse it as a grid...
    let input_lines = load_input_lines_by_name(file!())?;
    // let input_lines = vec![
    //     // ".....",
    //     // ".S-7.",
    //     // ".|.|.",
    //     // ".L-J.",
    //     // ".....",
    //     "..F7.",
    //     ".FJ|.",
    //     "SJ.L7",
    //     "|F--J",
    //     "LJ...",
    // ]
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<String>>();

    let grid = input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Pipe::try_from(c))
                .collect::<Result<Vec<Pipe>>>()
        })
        .collect::<Result<Vec<Vec<Pipe>>>>()?;

    // Get the width and height of the grid...
    // let height = grid.len();
    // let width = grid[0].len();
    
    // Find the starting point...
    let start = find_start(&grid)?;
    if debug { println!("Start point: {}", start); }
    
    // Find the connected points...
    let (next1, next2) = get_connections(&grid, &start)
        .ok_or(anyhow!("No connections found"))?;

    // Follow the two paths back to the start...
    let path1 = {
        if debug { println!("Path 1: {} -> {}", start, next1); }
        let mut path = vec![start, next1];
        let mut prev = start;
        let mut this = next1;
        let mut dist = 2; // TODO - Remove me...
        while this != start {
            let (a, b) = get_connections(&grid, &this)
                .ok_or(anyhow!("No connections found"))?;
            if a != prev {
                path.push(a);
                prev = this;
                this = a;
            } else {
                path.push(b);
                prev = this;
                this = b;
            }
            if debug { println!("> {} -> {} :: dist = {:2}", prev, this, dist); }
            dist += 1;
        }
        path
    };
    let path2 = {
        if debug { println!("Path 1: {} -> {}", start, next1); }
        let mut path = vec![start, next2];
        let mut prev = start;
        let mut this = next2;
        let mut dist = 2; // TODO - Remove me...
        while this != start {
            let (a, b) = get_connections(&grid, &this)
                .ok_or(anyhow!("No connections found"))?;
            if a != prev {
                path.push(a);
                prev = this;
                this = a;
            } else {
                path.push(b);
                prev = this;
                this = b;
            }
            if debug { println!("> {} -> {} :: dist = {:2}", prev, this, dist); }
            dist += 1;
        }
        path
    };

    // Convert the paths to distances...
    let dists1 = path1
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .filter(|(p, _)| *p != start)
        .collect::<HashMap<_, _>>();
    let dists2 = path2
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .filter(|(p, _)| *p != start)
        .collect::<HashMap<_, _>>();
    let mut dists = dists1
        .iter()
        .map(|(p, d1)| {
            let d2 = dists2.get(p).expect("Point not found in path 2");
            (*p, *d1.min(d2))
        })
        .collect::<Vec<(_, _)>>();
    dists.sort_by_key(|(_, d)| *d);
   
    if debug { 
        println!("Dists:"); 
        for (p, d) in dists.iter() {
            println!("{} is {:2} steps from the start", p, d);
        }
        println!();
    }

    // Find the furthest point...
    let (furthest_point, furthest_dist) = dists
        .last()
        .ok_or(anyhow!("No points found"))?;
    println!("Furthest point: {} ({} steps)", furthest_point, furthest_dist);
    
    Ok(())
}
