#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashSet;
use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug,Clone,PartialEq,Eq)]
enum Space {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitLR,
    SplitUD,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    
    /// Create a new `Pos` that's moved one step in direction `d`.
    ///
    /// If the new position would be off the edge of the board,
    /// returns `None`.
    fn step(&self, d: &Direction, w: usize, h: usize) -> Option<Self> {
        match d {
            Direction::Up => {
                if self.y == 0 {
                    None
                } else {
                    Some(Self::new(self.x, self.y-1))
                }
            },
            Direction::Down => {
                if self.y >= h-1 {
                    None
                } else {
                    Some(Self::new(self.x, self.y+1))
                }
            },
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some(Self::new(self.x-1, self.y))
                }
            },
            Direction::Right => {
                if self.x >= w-1 {
                    None
                } else {
                    Some(Self::new(self.x+1, self.y))
                }
            },
        }
    }
}

impl TryFrom<char> for Space {
    type Error = anyhow::Error;
    
    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Space::Empty),
            '/' => Ok(Space::MirrorForward),
            '\\' => Ok(Space::MirrorBackward),
            '-' => Ok(Space::SplitLR),
            '|' => Ok(Space::SplitUD),
            _ => Err(anyhow!("Unknown character '{}'", c)),
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Beam {
    pos: Pos,
    dir: Direction,
}

impl Beam {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        let pos = Pos::new(x, y);
        Self { pos, dir }
    }

    fn step(&self, s: &Space, w: usize, h: usize) -> Vec<Beam> {
        match s {
            // Empyy space continues in the same direction
            // as long as it doesn't fall off the edge...
            Space::Empty => {
                if let Some(p) = self.pos.step(&self.dir, w, h) {
                    vec![Self::new(p.x, p.y, self.dir.clone())]
                } else {
                    vec![]
                }
            },

            // Redirects at a 90 degree angle based on the
            // direction it's coming from (like mirror back).
            //
            // right -> up
            // down -> left
            // left -> down
            // up -> right
            Space::MirrorForward => {
                match self.dir {
                    Direction::Right => {
                        if let Some(p) = self.pos.step(&Direction::Up, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Up)]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Down => {
                        if let Some(p) = self.pos.step(&Direction::Left, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Left)]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Left => {
                        if let Some(p) = self.pos.step(&Direction::Down, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Down)]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Up => {
                        if let Some(p) = self.pos.step(&Direction::Right, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Right)]
                        } else {
                            vec![]
                        }
                    },
                }
            },
            
            // Redirects at a 90 degree angle based on the
            // direction it's coming from (like mirror fwd).
            //
            // right -> down
            // down -> right
            // left -> up
            // up -> left
            Space::MirrorBackward => {
                match self.dir {
                    Direction::Right => {
                        if let Some(p) = self.pos.step(&Direction::Down, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Down)]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Down => {
                        if let Some(p) = self.pos.step(&Direction::Right, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Right)]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Left => {
                        if let Some(p) = self.pos.step(&Direction::Up, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Up)]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Up => {
                        if let Some(p) = self.pos.step(&Direction::Left, w, h) {
                            vec![Self::new(p.x, p.y, Direction::Left)]
                        } else {
                            vec![]
                        }
                    },
                }
            },

            // If it's moving left/right, treat like an empty
            // space. Otherwise, if moving  up/down, create
            // two beams: one left, one right.
            Space::SplitLR => {
                match self.dir {
                    Direction::Left | Direction::Right => {
                        // NOTE: This is the same as `Empty` above...
                        if let Some(p) = self.pos.step(&self.dir, w, h) {
                            vec![Self::new(p.x, p.y, self.dir.clone())]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Up | Direction::Down => {
                        let mut nexts = Vec::new();
                        if let Some(p) = self.pos.step(&Direction::Left, w, h) {
                            nexts.push(Self::new(p.x, p.y, Direction::Left));
                        }
                        if let Some(p) = self.pos.step(&Direction::Right, w, h) {
                            nexts.push(Self::new(p.x, p.y, Direction::Right));
                        }
                        nexts
                    },
                }
            },

            // If it's moving up/down, treat like an empty
            // space. Otherwise, if moving  left/right, create
            // two beams: one up, one down.
            Space::SplitUD => {
                match self.dir {
                    Direction::Up | Direction::Down => {
                        // NOTE: This is the same as `Empty` above...
                        if let Some(p) = self.pos.step(&self.dir, w, h) {
                            vec![Self::new(p.x, p.y, self.dir.clone())]
                        } else {
                            vec![]
                        }
                    },
                    Direction::Left | Direction::Right => {
                        let mut nexts = Vec::new();
                        if let Some(p) = self.pos.step(&Direction::Up, w, h) {
                            nexts.push(Self::new(p.x, p.y, Direction::Up));
                        }
                        if let Some(p) = self.pos.step(&Direction::Down, w, h) {
                            nexts.push(Self::new(p.x, p.y, Direction::Down));
                        }
                        nexts
                    },
                }
            },
        }
    }
}

fn main() -> Result<()> {
    // Parse the input as a grid of spaces...
    let input = load_input_lines_by_name(file!())?;
    // let input = vec![
    //     ".|...\\....".to_string(),
    //     "|.-.\\.....".to_string(),
    //     ".....|-...".to_string(),
    //     "........|.".to_string(),
    //     "..........".to_string(),
    //     ".........\\".to_string(),
    //     "..../.\\\\..".to_string(),
    //     ".-.-/..|..".to_string(),
    //     ".|....-|.\\".to_string(),
    //     "..//.|....".to_string(),
    // ];
    let grid = input
        .iter()
        .map(|line| line
            .chars()
            .map(|c| Space::try_from(c))
            .collect::<Result<Vec<Space>>>()
        )
        .collect::<Result<Vec<_>>>()?;

    // Get the grid size...
    let height = grid.len();
    let width = grid[0].len();

    // Create a vector of beams...
    let mut beams = vec![
        Beam::new(0, 0, Direction::Right),
    ];

    // Track the energized spaces...
    let mut energized_spaces: HashSet<Pos> = HashSet::new();

    // Track the beam (position, direction) pairs that
    // have already been seen...
    let mut seen_beams: HashSet<Beam> = HashSet::new();

    // Loop until all of the beams are gone... 
    while beams.len() > 0 {
        // Make sure all the current beams are in the
        // energized set...
        for beam in &beams {
            energized_spaces.insert(beam.pos.clone());
            seen_beams.insert(beam.clone());
        }

        // Step each of the beams forward once...
        let next = beams
            .iter()
            .flat_map(|b| {
                // Find the grid spot at that position
                // (Note: They should all be in range)...
                let s = &grid[b.pos.y][b.pos.x];

                // Step the beam...
                b.step(s, width, height)
            })
            .collect::<Vec<_>>();

        // The next beams from this round become the beams
        // for the next round...
        beams = next
            .into_iter()
            .filter(|b| !seen_beams.contains(b))
            .collect::<Vec<_>>();
    }

    // Debug print the state of the board after the fact...
    for (i, row) in grid.iter().enumerate() {
        for (j, space) in row.iter().enumerate() {
             if energized_spaces.contains(&Pos::new(j, i)) {
                print!("#");
                continue
             }
             match space {
                Space::Empty => print!("."),
                Space::MirrorForward => print!("/"),
                Space::MirrorBackward => print!("\\"),
                Space::SplitLR => print!("-"),
                Space::SplitUD => print!("|"),
             }
        }
        println!();
    }

    // How many spots were energized?
    let count = energized_spaces.len();
    println!("There are {} energized spaces", count);
    println!("There are {} remaining beams", beams.len());

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_step() {
        let width = 10;
        let height = 10;
        let cases = vec![
            (
                Pos::new(0, 0),
                Direction::Left,
                None,
            ),
            (
                Pos::new(0, 0),
                Direction::Right,
                Some(Pos::new(1, 0)),
            ),
            (
                Pos::new(0, 0),
                Direction::Down,
                Some(Pos::new(0, 1)),
            ),
            (
                Pos::new(0, 0),
                Direction::Up,
                None,
            ),
            (
                Pos::new(1, 1),
                Direction::Left,
                Some(Pos::new(0, 1)),
            ),
            (
                Pos::new(9, 4),
                Direction::Right,
                None,
            ),
        ];
        for (i, (p, d, expect)) in cases.into_iter().enumerate() {
            let res = p.step(&d, width, height);
            assert_eq!(
                res, expect, 
                "Case {} failed. Expected ({:?})+({:?})=({:?}). Got ({:?})", 
                i, p, d, expect, res,
            );
        }
    }
    
    #[test]
    fn test_beam_step() {
        let width = 10;
        let height = 10;
        let cases = vec![
            (
                Beam::new(0, 0, Direction::Right),
                Space::Empty,
                vec![
                    Beam::new(1, 0, Direction::Right),
                ],
            ),
            (
                Beam::new(0, 0, Direction::Left),
                Space::Empty,
                vec![],
            ),
            (
                Beam::new(0, 0, Direction::Right),
                Space::MirrorForward,
                vec![],
            ),
            (
                Beam::new(0, 0, Direction::Right),
                Space::MirrorBackward,
                vec![
                    Beam::new(0, 1, Direction::Down),
                ],
            ),
        ];
        for (i, (b, s, expect)) in cases.into_iter().enumerate() {
            let res = b.step(&s, width, height);
            assert_eq!(
                res, expect, 
                "Case {} failed. Expected ({:?})+({:?})=({:?}). Got ({:?})", 
                i, b, s, expect, res,
            );
        }
    }
}

