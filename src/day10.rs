use anyhow::Result;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = parse_input(fs::read_to_string("day10.input")?.as_str());
    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.split("\n")
      .map(|line| parse_line(line))
      .collect()
}

fn parse_line(line: &str) -> Instruction {
    match line.split_once(" ") {
        Some((_, x)) => Instruction::AddX(x.parse::<i32>().unwrap()),
        None => Instruction::Noop,
    }
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let register: Vec<i32> = instructions.iter()
      .fold(
        vec![1], 
        |mut acc, instruction| {
            match instruction {
                Instruction::AddX(x) => {
                    let last = acc.last().unwrap().clone();
                    acc.push(last);
                    acc.push(last + x);
                    acc
                },
                Instruction::Noop => {acc.push(acc.last().unwrap().clone()); acc}
            }
        });
    let position: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let signal_strenth: i32 = position.iter()
      .map(|x| x.clone())
      .map(|x| (x as i32) * register[x - 1])
      .sum();
    signal_strenth
}

fn part2(instructions: &Vec<Instruction>) -> String {
    let register: Vec<i32> = instructions.iter()
    .fold(
      vec![1], 
      |mut acc, instruction| {
          match instruction {
              Instruction::AddX(x) => {
                  let last = acc.last().unwrap().clone();
                  acc.push(last);
                  acc.push(last + x);
                  acc
              },
              Instruction::Noop => {acc.push(acc.last().unwrap().clone()); acc}
          }
      });
    let result: String = register.iter()
      .enumerate()
      .map(|(index, value)| ((index % 40) as i32, value))
      .map(|(position, value)| {
        let character: String = 
            if position >= value - 1 && position <= value + 1 {
                String::from("#")
            } else {
                String::from(".")
            };
        if position == 39 {
            character + "\n"
        } else {
            character
        }
      })
      .collect();
    result
}


// cargo test --bin day10 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(INPUT));
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(INPUT));
        println!("{result}");
        assert_eq!(result, "");
    }

    const INPUT: &str = "addx 15\n\
                        addx -11\n\
                        addx 6\n\
                        addx -3\n\
                        addx 5\n\
                        addx -1\n\
                        addx -8\n\
                        addx 13\n\
                        addx 4\n\
                        noop\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx -35\n\
                        addx 1\n\
                        addx 24\n\
                        addx -19\n\
                        addx 1\n\
                        addx 16\n\
                        addx -11\n\
                        noop\n\
                        noop\n\
                        addx 21\n\
                        addx -15\n\
                        noop\n\
                        noop\n\
                        addx -3\n\
                        addx 9\n\
                        addx 1\n\
                        addx -3\n\
                        addx 8\n\
                        addx 1\n\
                        addx 5\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -36\n\
                        noop\n\
                        addx 1\n\
                        addx 7\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 2\n\
                        addx 6\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        addx 7\n\
                        addx 1\n\
                        noop\n\
                        addx -13\n\
                        addx 13\n\
                        addx 7\n\
                        noop\n\
                        addx 1\n\
                        addx -33\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 2\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 8\n\
                        noop\n\
                        addx -1\n\
                        addx 2\n\
                        addx 1\n\
                        noop\n\
                        addx 17\n\
                        addx -9\n\
                        addx 1\n\
                        addx 1\n\
                        addx -3\n\
                        addx 11\n\
                        noop\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        addx -13\n\
                        addx -19\n\
                        addx 1\n\
                        addx 3\n\
                        addx 26\n\
                        addx -30\n\
                        addx 12\n\
                        addx -1\n\
                        addx 3\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -9\n\
                        addx 18\n\
                        addx 1\n\
                        addx 2\n\
                        noop\n\
                        noop\n\
                        addx 9\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -1\n\
                        addx 2\n\
                        addx -37\n\
                        addx 1\n\
                        addx 3\n\
                        noop\n\
                        addx 15\n\
                        addx -21\n\
                        addx 22\n\
                        addx -6\n\
                        addx 1\n\
                        noop\n\
                        addx 2\n\
                        addx 1\n\
                        noop\n\
                        addx -10\n\
                        noop\n\
                        noop\n\
                        addx 20\n\
                        addx 1\n\
                        addx 2\n\
                        addx 2\n\
                        addx -6\n\
                        addx -11\n\
                        noop\n\
                        noop\n\
                        noop";
}