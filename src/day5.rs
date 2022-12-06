use anyhow::Result;
use std::fs;
use std::collections::LinkedList;
use regex::Regex;

#[derive(Debug)]
struct Stack {
    stacks: Vec<LinkedList<char>>
}

impl Stack {
    fn from_drawing(drawing: &str) -> Stack {
        let matrics: Vec<Vec<char>> = drawing.split("\n")
            .map(|line| line.chars().collect::<Vec<char>>().chunks(4).map(|c| c[1]).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .into_iter()
            .rev()
            .skip(1)
            .collect();
        let n = matrics[0].len();
        let mut stacks: Vec<LinkedList<char>> = vec![LinkedList::new(); n];
        for row in &matrics {
            for i in 0..n {
                if row[i] != ' ' {
                    stacks[i].push_back(row[i]);
                }
            }
        }
        Stack {stacks: stacks}   
    }
}

#[derive(Debug)]
struct Move {
    number: u32,
    from: usize,
    to: usize,
}

impl Move {
    fn parse(instruction: &str) -> Move {
        let re = Regex::new(r"move (?P<number>\d+) from (?P<from>\d+) to (?P<to>\d+)").expect("invalid regex");
        let caps = re.captures(instruction).expect("parse intruction {instruction}");
        Move {
            number: caps["number"].parse::<u32>().unwrap(),
            from: caps["from"].parse::<usize>().unwrap(),
            to: caps["to"].parse::<usize>().unwrap(),
        }
    }

    fn parse_multiple(instructions: &str) -> Vec<Move> {
        instructions.split("\n")
          .map(|instruction| Move::parse(instruction))
          .collect()
    }
}

fn main() -> Result<()> {
    let (mut stack, moves): (Stack, Vec<Move>) = fs::read_to_string("day5.input")?
      .split_once("\n\n")
      .map(|(drawing, moves)| (Stack::from_drawing(drawing), Move::parse_multiple(moves)))
      .expect("invalid input");

    for m in moves {
        for _ in 0..m.number {
            let element: char = stack.stacks[m.from - 1].pop_back().expect("stack has elements");
            stack.stacks[m.to - 1].push_back(element);
        }
    }

    let part1: String = stack.stacks.iter().flat_map(|list| list.back()).collect::<String>();
    println!("{part1}");

    let (mut stack, moves): (Stack, Vec<Move>) = fs::read_to_string("day5.input")?
      .split_once("\n\n")
      .map(|(drawing, moves)| (Stack::from_drawing(drawing), Move::parse_multiple(moves)))
      .expect("input has two parts separating by two newlines");

    for m in moves {
        let mut temp_stack = LinkedList::new();
        for _ in 0..m.number {
            let element: char = stack.stacks[m.from - 1].pop_back().unwrap();
            temp_stack.push_front(element);
        }
        for element in temp_stack {
            stack.stacks[m.to - 1].push_back(element);
        }
    }

    let part2: String = stack.stacks.iter().filter_map(|list| list.back()).collect::<String>();
    println!("{part2}");
    Ok(())
}