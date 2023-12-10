use anyhow::{anyhow, Context, Result};
use aoc23::load_input_lines_by_name;
use std::collections::HashSet;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn new_us(x: usize, y: usize) -> Result<Self> {
        Ok(Self::new(x.try_into()?, y.try_into()?))
    }

    fn north(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    fn south(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn east(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn west(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            _ => Err(anyhow!("Invalid pipe character: {}", c)),
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
        }
        Pipe::NorthSouth => Some((point.north(), point.south())),
        Pipe::EastWest => Some((point.east(), point.west())),
        Pipe::NorthEast => Some((point.north(), point.east())),
        Pipe::NorthWest => Some((point.north(), point.west())),
        Pipe::SouthEast => Some((point.south(), point.east())),
        Pipe::SouthWest => Some((point.south(), point.west())),
        Pipe::Ground => None,
    }
}

/// Infers the pipe type at `point`, given it's neighbors.
fn infer(grid: &Vec<Vec<Pipe>>, point: &Point) -> Result<Pipe> {
    let (a, b) = get_connections(grid, point)
        .ok_or(anyhow!("Unable to find connections to point {}", point))?;

    // Get the neighbors...
    let north = point.north();
    let south = point.south();
    let east = point.east();
    let west = point.west();

    // Check which sides it connects to...
    let connects_north = a == north || b == north;
    let connects_south = a == south || b == south;
    let connects_east = a == east || b == east;
    let connects_west = a == west || b == west;

    // Check the combos...
    if connects_north && connects_south {
        return Ok(Pipe::NorthSouth);
    }
    if connects_north && connects_east {
        return Ok(Pipe::NorthEast);
    }
    if connects_north && connects_west {
        return Ok(Pipe::NorthWest);
    }
    if connects_south && connects_east {
        return Ok(Pipe::SouthEast);
    }
    if connects_south && connects_west {
        return Ok(Pipe::SouthWest);
    }
    if connects_east && connects_west {
        return Ok(Pipe::EastWest);
    }
    Err(anyhow!("Unable to infer pipe type at {}", point))
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

    // Replace the start pipe with its inferred type...
    let grid = {
        let start_pipe = infer(&grid, &start).context("Failed to infer start pipe type")?;
        let mut grid = grid;
        let row = &mut grid[start.y as usize];
        row[start.x as usize] = start_pipe;
        grid
    };

    // Find the connected points...
    let (next, _) = get_connections(&grid, &start).ok_or(anyhow!("No connections found"))?;

    // Follow the two paths back to the start...
    let path_points: HashSet<Point> = {
        let mut path = vec![start, next];
        let mut prev = start;
        let mut this = next;
        while this != start {
            let (a, b) = get_connections(&grid, &this).ok_or(anyhow!("No connections found"))?;
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
        path.iter().copied().collect()
    };

    // Create sets to track points inside/outside
    // the path...
    let mut points_out = HashSet::<Point>::new();
    let mut points_in = HashSet::<Point>::new();
    // let mut points_unsure = HashSet::<Point>::new();

    // Iterate through the grid...
    //
    // For each row, imagine drawing a line from left
    // to right, starting off the left side, and ending
    // off the right side.
    //
    // As we start, we know we're outside the path and
    // any point we encounter is outside the path. But
    // if we cross the path, then everything flips and
    // any point we encounter is inside the path --
    // until we cross the path again.
    //
    // For example, the following cross section shows
    // how the markers would flip, moving left to right:
    //
    // ```
    // O O   I I   O
    // ↓ ↓   ↓ ↓   ↓
    // . . | . . | .
    // ```
    //
    // But...we need to account for cases where we touch
    // the path but don't *cross* it. For example:
    //
    // ```
    // O       O   I
    // ↓       ↓   ↓
    // . L - J . | .
    // ```
    //
    // If we flipped the marker at each path point, we'd
    // end up with `OIO` instead of `OOI`. Also consider:
    //
    // ```
    // O       I   O
    // ↓       ↓   ↓
    // . L - 7 . | .
    // ```
    //
    // I think the key here is that with the `|` pipe,
    // we're fully crossing the path, so we flip the
    // marker.
    //
    // With the `-` pipe, we're following the path, so
    // we don't do anything.
    //
    // With the `LJ7F` pipes, we're only *half* crossing
    // the path. Depending on the *state* that we're in.
    // If we're fully in or out, the only (valid) half
    // steps we should see are `L` or `F`. If we see one
    // of those, we don't flip the marker yet. Instead
    // track the half-step. From then on, we should see
    // zero or more `-` pipes until we reach the next
    // half-step -- either `J` or `7`. Then, if the two
    // markers *combine* to form a `|` pipe, we flip the
    // marker, otherwise we don't.
    //
    // ```
    // F + J = |
    // L + 7 = |
    // ```
    //
    for (y, row) in grid.iter().enumerate() {
        // Initialize the marker and the half-state...
        let mut mark_in = false;
        let mut half_state: Option<Pipe> = None;
        for (x, pipe) in row.iter().enumerate() {
            // Construct the point...
            let point = Point::new_us(x, y)?;

            // If the point is in the path, check to flip...
            if path_points.contains(&point) {
                match *pipe {
                    Pipe::NorthSouth => {
                        // Flip!
                        mark_in = !mark_in;
                    }
                    Pipe::EastWest => {} // no-op
                    Pipe::NorthEast => {
                        // Mark and continue...
                        half_state = Some(*pipe);
                    }
                    Pipe::SouthEast => {
                        // Mark and continue...
                        half_state = Some(*pipe);
                    }
                    Pipe::NorthWest => {
                        // Check state and maybe flip...
                        let from = half_state.ok_or(anyhow!("No half-state found"))?;
                        match from {
                            Pipe::SouthEast => {
                                // Flip!
                                mark_in = !mark_in;
                            }
                            Pipe::NorthEast => {} // no-op
                            _ => unreachable!("How'd we get here!? Was {:?} is {:?}", from, pipe),
                        }
                    }
                    Pipe::SouthWest => {
                        // Check state and maybe flip...
                        let from = half_state.ok_or(anyhow!("No half-state found"))?;
                        match from {
                            Pipe::NorthEast => {
                                // Flip!
                                mark_in = !mark_in;
                            }
                            Pipe::SouthEast => {} // no-op
                            _ => unreachable!("How'd we get here!? Was {:?} is {:?}", from, pipe),
                        }
                    }
                    _ => unreachable!(
                        "Ground and start pipes should not be in the path: {:?}",
                        pipe
                    ),
                }
                continue;
            }

            // Otherwise, add it to the marked bucket...
            if mark_in {
                points_in.insert(point);
            } else {
                points_out.insert(point);
            }
        }
    }

    // Get the grid size...
    let height = grid.len();
    let width = grid[0].len();
    let n_points_total = width * height;
    println!("Points total = {}", n_points_total);
    println!("Points on path = {}", path_points.len());
    println!("Points outside path = {}", points_out.len());
    println!("Points inside path = {}", points_in.len());

    Ok(())
}
