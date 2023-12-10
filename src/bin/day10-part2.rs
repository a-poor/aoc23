use std::collections::HashSet;
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
    
    fn new_us(x: usize, y: usize) -> Result<Self> {
        Ok(Self::new(
            x.try_into()?, 
            y.try_into()?,
        ))
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
    // Load the input data and parse it as a grid...
    let input_lines = load_input_lines_by_name(file!())?;
    // let input_lines = vec![
    //     "FF7FSF7F7F7F7F7F---7".to_string(),
    //     "L|LJ||||||||||||F--J".to_string(),
    //     "FL-7LJLJ||||||LJL-77".to_string(),
    //     "F--JF--7||LJLJ7F7FJ-".to_string(),
    //     "L---JF-JLJ.||-FJLJJ7".to_string(),
    //     "|F|F-JF---7F7-L7L|7|".to_string(),
    //     "|FFJF7L7F-JF7|JL---7".to_string(),
    //     "7-L-JL7||F7|L7F-7F7|".to_string(),
    //     "L.L7LFJ|||||FJL7||LJ".to_string(),
    //     "L7JLJL-JLJLJL--JLJ.L".to_string(),
    // ];

    let grid = input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Pipe::try_from(c))
                .collect::<Result<Vec<Pipe>>>()
        })
        .collect::<Result<Vec<Vec<Pipe>>>>()?;
 
    // Find the starting point...
    let start = find_start(&grid)?;
    
    // Find the connected points...
    let (next, _) = get_connections(&grid, &start)
        .ok_or(anyhow!("No connections found"))?;

    // Follow the two paths back to the start...
    let path_points: HashSet<Point> = {
        let mut path = vec![start, next];
        let mut prev = start;
        let mut this = next;
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
        }
        path
            .iter()
            .copied()
            .collect()
    };

    // Find the points that are not contained 
    // by the path. Start by seeding the in/out
    // sets with points on the edge of the map
    // and points on the line...
    let mut points_out = HashSet::<Point>::new();
    // let mut points_in = HashSet::<Point>::new();
    let mut points_unsure = HashSet::<Point>::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            // Construct the point...
            let point = Point::new_us(x, y)?;

            // If the point is in the path, skip it... 
            if path_points.contains(&point) {
                continue;
            }

            // If it's on an edge, add it to the
            // points out...
            if x == 0 || y == 0 || x == row.len()-1 || y == grid.len()-1 {
                points_out.insert(point);
                continue;
            }
            
            // If we get here, the point is inside the path...
            points_unsure.insert(point);
        }
    }
    
    // Now, using that as a starting point,
    // iterate through the points we're unsure
    // of, and check their neighbors...
    // for i in 0..20 {
    // while points_unsure.len() > 0 {
    loop {
        println!("{:6} points unsure", points_unsure.len());

        let queue = points_unsure;
        points_unsure = HashSet::new();
        
        for point in queue.iter() {
            // Are any of it's neighbors outside the path?
            // (Note: No neighbors should be on the edge or
            // within the path -- from previous checks --
            // so we don't need to check for that.)
            if points_out.contains(&point.north()) 
                || points_out.contains(&point.south()) 
                || points_out.contains(&point.east()) 
                || points_out.contains(&point.west()) 
            {
                points_out.insert(*point);
                continue;
            }

            // Otherwise, we're still unsure...
            points_unsure.insert(*point);
        }

        // If the number of points is unchanged, we're done...
        if points_unsure.len() == queue.len() {
            break;
        }
    }

    // Okay, now to find the points that ar
    
    
    // Get the grid size...
    let height = grid.len();
    let width = grid[0].len();
    let n_points_total = width * height;

    // Get the number of points inside the path...
    let count = n_points_total - points_out.len() - path_points.len();
    println!("{} points inside the path", count);

    Ok(())
}
