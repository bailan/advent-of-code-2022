use anyhow::Result;
use std::fs;
use std::collections::HashMap;

fn main() -> Result<()> {
    let part1: usize = fs::read_to_string("day3.input")?
        .split("\n")
        .map(|line| line.split_at(line.len()/2))
        .map(|(first, second)| find_common(first, second))
        .map(|common| priority(&common))
        .sum();
    println!("{part1}");

    let part2: usize = fs::read_to_string("day3.input")?
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunks| find_common3(chunks[0], chunks[1], chunks[2]))
        .map(|common| priority(&common))
        .sum();
    println!("{part2}");
    Ok(())
}

fn find_common(first: &str, second: &str) -> char {
    first.chars().find(|c| second.contains(*c)).expect("common item between {first} and {second}")
}

fn find_common3(first: &str, second: &str, third: &str) -> char {
    first.chars().find(|c| second.contains(*c) && third.contains(*c)).expect("common item between {first}, {second}, and {third}")
}

fn priority(item: &char) -> usize {
    let priority_map: HashMap<char, usize> = ('a'..='z').chain('A'..='Z')
        .enumerate()
        .map(|(index, character)| (character, index+1))
        .collect();
    *priority_map.get(item).expect("item {item}")
}