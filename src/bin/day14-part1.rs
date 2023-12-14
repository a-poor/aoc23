use anyhow::{anyhow,Result};
use aoc23::load_input_lines_by_name;

#[derive(Clone,Copy,PartialEq)]
enum Space {
    RoundRock,
    SquareRock,
    Empty,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::RoundRock => write!(f, "O"),
            Space::SquareRock => write!(f, "#"),
            Space::Empty => write!(f, "."),
        }
    }
}

impl TryFrom<char> for Space {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Space::Empty),
            'O' => Ok(Space::RoundRock),
            '#' => Ok(Space::SquareRock),
            _ => Err(anyhow!("Unknown space type")),
        }
    }
}

struct Grid {
    spaces: Vec<Vec<Space>>,
}

impl Grid {
    fn parse(input: &Vec<String>) -> Result<Self> {
        let spaces = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| Space::try_from(c))
                    .collect::<Result<Vec<Space>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Grid { spaces })
    }

    fn shift_north_once(&mut self) -> Result<bool> {
        // Track if anything has moved...
        let mut shifted = false;

        // Get the open spaces in the first line...
        let mut open_spaces = self.spaces[0]
            .iter()
            .enumerate()
            .filter(|(_, space)| **space == Space::Empty)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        // Now, loop through the lines...
        for i in 1..self.spaces.len() {
            // Get the indexes in this row with rocks that can move.
            //
            // These will be spaces that contain a round rock and where
            // the space above (grid[i-1][j]) is empty.
            //
            // The `open_spaces` vec contains the empty spaces (by idx)
            // in the previous row.
            let moves = self.spaces[i]
                .iter()
                .enumerate()
                .filter(|(j, space)| (
                        **space == Space::RoundRock
                        && open_spaces.binary_search(&j).is_ok()
                ))
                .map(|(j, _)| j)
                .collect::<Vec<_>>();

            // Update shifted, if necessary...
            shifted = shifted || moves.len() > 0;

            // Update the spaces in the current and preceding row...
            for j in moves {
                self.spaces[i][j] = Space::Empty;
                self.spaces[i - 1][j] = Space::RoundRock;
            }
           
            // Update the open spaces...
            open_spaces = self.spaces[i]
                .iter()
                .enumerate()
                .filter(|(_, space)| **space == Space::Empty)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
        }
        
        // Return the shifted flag...
        Ok(shifted)
    }

    fn shift_north_until_settled(&mut self) -> Result<()> {
        while self.shift_north_once()? {}
        Ok(())
    }

    fn get_load(&self) -> usize {
        let n = self.spaces.len();
        self.spaces
            .iter()
            .enumerate()
            .map(|(i, row)| {
                (n - i) * row.iter()
                    .filter(|space| **space == Space::RoundRock)
                    .count()
            })
            .sum::<usize>()
    }
}

fn main() -> Result<()> {
    // Load the input liles...
    let input = load_input_lines_by_name(file!())?;

    // Parse it as a grid...
    let mut grid = Grid::parse(&input)?;

    // Shift all the round grid stones north until they settle...
    grid.shift_north_until_settled()?;

    // Get the load...
    let load = grid.get_load();

    // Print the load...
    println!("The load is: {}", load);

    Ok(())
}

