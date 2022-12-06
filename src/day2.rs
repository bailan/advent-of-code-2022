use anyhow::Result;
use std::fs;
use HandShape::*;

#[derive(Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors
}

fn main() -> Result<()> {
    let part1: u32 = fs::read_to_string("day2.input")?
        .split("\n")
        .flat_map(|line| line.split_once(" "))
        .map(|(opponent, elf)| (parse_hand_shape(opponent), parse_hand_shape(elf)))
        .map(|(opponent, elf)| score(opponent, elf))
        .sum();
    println!("{}", part1);

    let part2: u32 = fs::read_to_string("day2.input")?
        .split("\n")
        .flat_map(|line| line.split_once(" "))
        .map(|(opponent, elf)| (parse_hand_shape(opponent), derive_hand_shape(opponent, elf)))
        .map(|(opponent, elf)| score(opponent, elf))
        .sum();
    println!("{}", part2);

    Ok(())
}

fn parse_hand_shape(string: &str) -> HandShape {
    match string {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        _ => panic!()
    }
}

fn derive_hand_shape(opponent: &str, elf: &str) -> HandShape {
    match (opponent, elf) {
        ("A", "Y") | ("B", "X") | ("C", "Z") => Rock,
        ("A", "Z") | ("B", "Y") | ("C", "X") => Paper,
        ("A", "X") | ("B", "Z") | ("C", "Y") => Scissors,
        _ => panic!()
    }
}

fn score(opponent: HandShape, elf: HandShape) -> u32 {
    let shape_score = match elf {
        Rock => 1,
        Paper => 2,
        Scissors => 3, 
    };
    let round_score = match (opponent, elf) {
        (Rock, Scissors) | (Paper, Rock)| (Scissors, Paper) => 0,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
    };
    shape_score + round_score
}