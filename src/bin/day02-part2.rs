use anyhow::{anyhow, Result, Context};
use aoc23::load_input_lines_by_name;

#[derive(Debug, Default)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn parse(s: &str) -> Result<CubeSet> {
        s.split(",")
            .map(|part| {
                // Use a regex capture to get the number and color...
                let re = regex::Regex::new(r"(\d+) (red|green|blue)").unwrap();
                let caps = re.captures(part).ok_or(anyhow!("no match"))?;
                
                // Get the number...
                let count = caps.get(1).ok_or(anyhow!("no number found"))?.as_str().parse::<usize>()?;
                
                // Get the color...
                let color = caps.get(2).ok_or(anyhow!("no color found"))?.as_str();

                // Return the result...
                match color {
                    "red" => Ok(CubeSet { red: count, ..CubeSet::default() }),
                    "green" => Ok(CubeSet { green: count, ..CubeSet::default() }),
                    "blue" => Ok(CubeSet { blue: count, ..CubeSet::default() }),
                    _ => Err(anyhow!("invalid color '{}'", color)),
                }
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .reduce(|acc, part| acc.add(&part))
            .ok_or(anyhow!("no result found"))
    }

    fn add(&self, other: &CubeSet) -> CubeSet {
        CubeSet {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }

    fn max(self, other: CubeSet) -> CubeSet {
        CubeSet {
            red: std::cmp::max(self.red, other.red),
            green: std::cmp::max(self.green, other.green),
            blue: std::cmp::max(self.blue, other.blue),
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct GameRes {
    id: usize,
    sets: Vec<CubeSet>,
}

impl GameRes {
    fn parse(s: &str) -> Result<GameRes> {
        // Create a regex to capture the game's data...
        let re = regex::Regex::new(r"^Game (\d+): (.*)$").unwrap();
        let caps = re.captures(s)
            .ok_or(anyhow!("no match"))?;

        // Extract the ID and the sets...
        let id = caps
            .get(1)
            .ok_or(anyhow!("no id found"))?
            .as_str()
            .parse::<usize>()
            .with_context(|| format!("Failed to parse id with text=\"{}\", n-caps={}, caps={:?}", s, caps.len(), caps.get(1)))?;
        let sets = caps
            .get(2)
            .ok_or(anyhow!("no sets found"))?
            .as_str()
            .split(";")
            .map(CubeSet::parse)
            .collect::<Result<Vec<_>>>()?;

        // Return the result...
        Ok(GameRes { id, sets })
    }
}

fn main() -> Result<()> {
    // Load the input data...
    let input_lines = load_input_lines_by_name(file!())?;

    // Parse the lines as game results...
    let game_results = input_lines
        .into_iter()
        .map(|line| GameRes::parse(&line))
        .collect::<Result<Vec<_>>>()?;

    // 1) Find the minimum number of cubes needed for each game.
    // 2) Calculate the power for each of those mins.
    // 3) Sum the power. 
    let res = game_results
        .into_iter()
        .map(|gr| gr.sets.into_iter().reduce(|left, right| left.max(right)).ok_or(anyhow!("no result found")))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|cs| cs.power())
        .fold(0, |acc, p| acc + p);

    // Output the results...
    println!("Done. Sum = {:?}", res);

    // Done!
    Ok(())
}

