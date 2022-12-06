use anyhow::Result;
use std::fs;
use std::collections::HashSet;

fn main() -> Result<()> {
    let datastream: Vec<char> = fs::read_to_string("day6.input")?
        .chars()
        .collect();

    let part1 = datastream
        .windows(4)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<&char>>().len() == 4)
        .map(|(i, _)| i + 4)
        .expect("index");
    println!("{part1}");

    let part2 = datastream
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<&char>>().len() == 14)
        .map(|(i, _)| i + 14)
        .expect("index");
    println!("{part2}");

    Ok(())
}