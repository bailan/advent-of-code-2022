use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let snafu: Vec<String> = parse_input(fs::read_to_string("day25.input")?.as_str());
    println!("{}", part1(&snafu));
    Ok(())
}

fn parse_input(input: &str) -> Vec<String> {
    input.split("\n").map(|s| s.to_string()).collect()
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut decimal = 0;
    for c in snafu.chars() {
        decimal = decimal * 5 + match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!()
        }
    }
    decimal
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut snafu: Vec<char> = Vec::new();
    let mut number = decimal;
    while number != 0 {
        snafu.push(match number % 5 {
            4 => {number += 1; '-'},
            3 => {number += 2; '='},
            2 => {number -= 2; '2'},
            1 => {number -= 1; '1'},
            0 => '0',
            _ => panic!(),
        });
        number /= 5;
    }
    snafu.iter().rev().collect()
}

fn part1(snafu: &Vec<String>) -> String {
    decimal_to_snafu(snafu.iter().map(|s| snafu_to_decimal(s)).sum())
}

// cargo test --bin day25 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, "2=-1=0");
    }

    const INPUT: &str ="1=-0-2\n\
                        12111\n\
                        2=0=\n\
                        21\n\
                        2=01\n\
                        111\n\
                        20012\n\
                        112\n\
                        1=-1=\n\
                        1-12\n\
                        12\n\
                        1=\n\
                        122";                
}