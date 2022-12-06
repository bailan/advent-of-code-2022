use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let mut calories: Vec<u32> = fs::read_to_string("day1.input")?
      .split("\n\n")
      .map(|lines| lines.split("\n").flat_map(|line| line.parse::<u32>()).sum())
      .collect();
    calories.sort();
    calories.reverse();
    println!("{}", calories.iter().take(1).sum::<u32>());
    println!("{}", calories.iter().take(3).sum::<u32>());
    Ok(())
}