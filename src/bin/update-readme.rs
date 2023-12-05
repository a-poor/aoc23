use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs;

const GEN_BEGIN_COMMENT: &str = "<!-- GENERATED BEGIN -->";
const GEN_END_COMMENT: &str = "<!-- GENERATED END -->";

fn main() -> Result<()> {
    // Get the file names...
    let mut gen_lines = fs::read_dir("./src/bin")?
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if file!().ends_with(entry.file_name().to_str()?) {
                    None
                } else {
                    Some(entry.file_name())
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
                None
            }
        })
        .map(|name| {
            let re = Regex::new(r"day(\d+)-part(1|2).rs")?;
            let name = name.to_str().ok_or(anyhow!("not a name"))?;
            let cap = re
                .captures(name)
                .ok_or_else(|| anyhow!("no captures returned"))?;
            let day = cap[1].parse::<u32>().map_err(|_| anyhow!("invalid day"))?;
            let part = cap[2].parse::<u32>().map_err(|_| anyhow!("invalid part"))?;
            Ok((day, part))
        })
        .collect::<Result<Vec<_>>>()?;

    // Sort the vector...
    gen_lines.sort_by(|(day1, part1), (day2, part2)| day1.cmp(day2).then(part1.cmp(part2)));

    // Convert to lines...
    let mut gen_lines = gen_lines
        .into_iter()
        .fold(Vec::new(), |mut acc, (day, part)| {
            // If this is part 1, add the top-level day...
            if part == 1 {
                acc.push(format!("- Day {}:", day));
                acc.push(format!("  - [Input Data](/data/{:02}.txt)", day));
            }
            acc.push(format!(
                "  - [Part {}](/src/bin/day{:02}-part{}.rs)",
                part, day, part
            ));
            acc
        });

    // Read in the readme...
    let mut skipping = false;
    let readme = fs::read_to_string("./README.md")?;
    let readme = readme
        .lines()
        .filter(|line| match (line.trim(), skipping) {
            (GEN_BEGIN_COMMENT, false) => {
                skipping = true;
                true
            }
            (GEN_END_COMMENT, true) => {
                skipping = false;
                true
            }
            (_, true) => false,
            _ => true,
        })
        .collect::<Vec<_>>();

    // Iterate over the lines...
    let mut out = Vec::<String>::new();
    for line in readme {
        // If we're at the place, add the generated lines...
        if line == GEN_BEGIN_COMMENT {
            out.push(line.to_string());
            out.append(&mut gen_lines);
        }
        // Otherwise, add the line...
        else {
            out.push(line.to_string());
        }
    }

    // Write the readme...
    let out = out.join("\n");
    fs::write("./README.md", out)?;

    Ok(())
}
