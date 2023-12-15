use anyhow::{anyhow,Result};
use aoc23::load_input_by_name;

#[derive(Debug,Clone,PartialEq,Eq)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl Lens {
    fn new(label: &str, focal_length: u32) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }

    fn parse(s: &str) -> Result<Self> {
        let parts = s
            .split("=")
            .collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid lens: {}", s));
        }
        let label = parts[0].trim();
        let focal_length = parts[1]
            .trim()
            .parse::<u32>()?;
        Ok(Self::new(label, focal_length))
    }

    fn get_hash(&self) -> u32 {
        hash_string(&self.label)
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
enum Operation {
    Set(Lens),
    Unset(String),
}

impl Operation {
    fn parse(s: &str) -> Result<Self> {
        // If it's a set command, it will contain an
        // equal sign. Parse it with the Lens parser
        // and return...
        if s.contains('=') {
            let lens = Lens::parse(s)?;
            return Ok(Self::Set(lens))
        }
        
        // Otherwise, it must be an unset command
        // (meaning it ends with an '-'). Get the
        // characters up to the dash and return...
        let label = s
            .split("-")
            .next()
            .ok_or_else(|| anyhow!("Invalid unset command: {}", s))?;
        Ok(Self::Unset(label.to_string()))
    }

    fn get_hash(&self) -> u32 {
        match self {
            Self::Set(lens) => lens.get_hash(),
            Self::Unset(label) => hash_string(label),
        }
    }
}

fn get_ascii_code(c: char) -> u32 {
    c as u32
}

fn hash_char(state: u32, c: char) -> u32 {
    let ascii_code = get_ascii_code(c);
    let next = state + ascii_code;
    let next = next * 17;
    let next = next % 256;
    next
}

fn hash_string(s: &str) -> u32 {
    let mut state = 0;
    for c in s.chars() {
        state = hash_char(state, c);
    }
    state
}

fn init_state() -> Vec<Vec<Lens>> {
    (0..256)
        .map(|_| Vec::<Lens>::new())
        .collect::<Vec<_>>()
}

fn update_state(state: &mut Vec<Vec<Lens>>, c: &Operation) {
    let box_idx = c.get_hash() as usize;
    match c {
        Operation::Unset(label) => {
            state[box_idx] = state[box_idx]
                .iter()
                .filter(|l| l.label != *label)
                .cloned()
                .collect::<Vec<_>>();
        },
        Operation::Set(lens) => {
            if !state[box_idx].iter().any(|l| l.label == lens.label) {
                state[box_idx].push(lens.clone());
            } else {
                state[box_idx] = state[box_idx]
                    .iter()
                    .map(|l| {
                        if l.label == lens.label {
                            lens.clone()
                        } else {
                            l.clone()
                        }
                    })
                    .collect::<Vec<_>>();
            }
        },
    }
}

fn score_state(state: &Vec<Vec<Lens>>) -> u32 {
    state
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, l)| {
                    let a = (i + 1) as u32;
                    let b = (j + 1) as u32;
                    let c = l.focal_length;
                    a * b * c
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn main() -> Result<()> {
    let input = load_input_by_name(file!())?;
    let parts = input
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    println!("There are {} parts", parts.len());
    
    // Parse the commands...
    let commands = parts
        .iter()
        .map(|s| Operation::parse(s))
        .collect::<Result<Vec<_>>>()?;
    
    // Initialize the state...
    let mut state = init_state();

    // Loop through the commands...
    for c in commands {
        update_state(&mut state, &c);
    }

    // Score the result...
    let score = score_state(&state);
    println!("Score: {}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ascii_code() {
        assert_eq!(get_ascii_code('a'), 97);
        assert_eq!(get_ascii_code('A'), 65);
        assert_eq!(get_ascii_code('1'), 49);
        assert_eq!(get_ascii_code('!'), 33);
    }
    
    #[test]
    fn test_hash_char() {
        assert_eq!(hash_char(  0, 'H'), 200);
        assert_eq!(hash_char(200, 'A'), 153);
        assert_eq!(hash_char(153, 'S'), 172);
        assert_eq!(hash_char(172, 'H'), 52);
    }
    
    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("rn=1"), 30);
        assert_eq!(hash_string("cm-"), 253);
        assert_eq!(hash_string("qp=3"), 97);
    }

    #[test]
    fn test_update_state() {
        let mut state = init_state();
        let c = Operation::parse("rn=1").unwrap();
        update_state(&mut state, &c);
        assert_eq!(state[0], vec![Lens::new("rn", 1)]);
        assert_eq!(state.iter().filter(|b| !b.is_empty()).count(), 1);

        let c = Operation::parse("cm-").unwrap();
        update_state(&mut state, &c);
        assert_eq!(state[0], vec![Lens::new("rn", 1)]);
        assert_eq!(state.iter().filter(|b| !b.is_empty()).count(), 1);

        let c = Operation::parse("qp=3").unwrap();
        update_state(&mut state, &c);
        assert_eq!(state[0], vec![Lens::new("rn", 1)]);
        assert_eq!(state[1], vec![Lens::new("qp", 3)]);
        assert_eq!(state.iter().filter(|b| !b.is_empty()).count(), 2);

        let c = Operation::parse("cm=2").unwrap();
        update_state(&mut state, &c);
        assert_eq!(state[0], vec![Lens::new("rn", 1),Lens::new("cm", 2)]);
        assert_eq!(state[1], vec![Lens::new("qp", 3)]);
        assert_eq!(state.iter().filter(|b| !b.is_empty()).count(), 2);

        let c = Operation::parse("qp-").unwrap();
        update_state(&mut state, &c);
        assert_eq!(state[0], vec![Lens::new("rn", 1),Lens::new("cm", 2)]);
        assert_eq!(state[1], vec![]);
        assert_eq!(state.iter().filter(|b| !b.is_empty()).count(), 1);

        let c = Operation::parse("pc=4").unwrap();
        update_state(&mut state, &c);
        assert_eq!(state[0], vec![Lens::new("rn", 1),Lens::new("cm", 2)]);
        assert_eq!(state[3], vec![Lens::new("pc", 4)]);
        assert_eq!(state.iter().filter(|b| !b.is_empty()).count(), 2);
    }

    #[test]
    fn test_score_state() {
        let state = vec![
            vec![
                Lens::new("rn", 1),
                Lens::new("cm", 2),
            ],
            vec![],
            vec![],
            vec![
                Lens::new("ot", 7),
                Lens::new("ab", 5),
                Lens::new("pc", 6),
            ],
        ];
        assert_eq!(score_state(&state), 145);
    }
}

