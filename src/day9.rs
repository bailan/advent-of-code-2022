use anyhow::Result;
use std::fs;
use std::iter;
use std::collections::LinkedList;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn of(x: i32, y: i32) -> Position {
        Position{ x, y }
    }

    fn origin() -> Position {
        Position::of(0, 0)
    }

    fn add(&self, another: &Position) -> Position {
        Position::of(self.x + another.x, self.y + another.y)
    }
}

#[derive(Debug)]
struct State {
    head: Position,
    tails: LinkedList<Position>,
}

#[derive(Clone)]
#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn main() -> Result<()> {
    let directions: Vec<Direction> = parse_input(fs::read_to_string("day9.input")?.as_str());
    println!("{}", part1(&directions));
    println!("{}", part2(&directions));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.split("\n")
      .filter_map(|line| line.split_once(" "))
      .flat_map(|(dir, times)| iter::repeat(parse_direction(dir)).take(times.parse::<usize>().unwrap()))
      .collect()
}

fn parse_direction(dir: &str) -> Direction {
    match dir {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        d => panic!("unrecognized direction {d}")
    }
}

fn moves(state: &State, direction: &Direction) -> State {
    let new_head: Position = move_head(&state.head, direction);
    let new_tails: LinkedList<Position> = state.tails
      .iter()
      .fold(LinkedList::from([new_head]), |mut acc, tail| {acc.push_back(move_tail(acc.back().unwrap(), tail)); acc})
      .into_iter()
      .skip(1)
      .collect();
    State {head: new_head, tails: new_tails}
}

fn move_head(head: &Position, direction: &Direction) -> Position {
    let offset = match direction {
        Direction::Right => Position::of(1, 0),
        Direction::Left => Position::of(-1, 0),
        Direction::Up => Position::of(0, 1),
        Direction::Down => Position::of(0, -1),
    };
    head.add(&offset)
}

fn move_tail(head: &Position, tail: &Position) -> Position {
    let offset = match (head.x - tail.x, head.y - tail.y) {
        (0, 0) | (0, 1) | (1, 0) | (0, -1) | (-1, 0) | (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => Position::of(0, 0),
        (2, 0) => Position::of(1, 0),
        (0, 2) => Position::of(0, 1),
        (-2, 0) => Position::of(-1, 0),
        (0, -2) => Position::of(0, -1),
        (2, 1) | (1, 2) | (2, 2) => Position::of(1, 1),
        (2, -1) | (1, -2) | (2, -2) => Position::of(1, -1),
        (-2, 1) | (-1, 2) | (-2, 2) => Position::of(-1, 1),
        (-2, -1) | (-1, -2) | (-2, -2) => Position::of(-1, -1),
        (x, y) => panic!("head tail distance ({x},{y})")
    };
    tail.add(&offset)
}

fn part1(directions: &Vec<Direction>) -> usize {
    let identity = LinkedList::from([State { head: Position::origin(), tails: LinkedList::from([Position::origin()]) }]);
    directions.iter()
      .fold(identity, |mut acc, direction| {acc.push_back(moves(acc.back().unwrap(), direction)); acc})
      .iter()
      .map(|state| state.tails.back().unwrap().clone())
      .collect::<HashSet<Position>>()
      .len()
}

fn part2(directions: &Vec<Direction>) -> usize {
    let identity = LinkedList::from([State { head: Position::origin(), tails: LinkedList::from([Position::origin(); 9]) }]);
    directions.iter()
      .fold(identity, |mut acc, direction| {acc.push_back(moves(acc.back().unwrap(), direction)); acc})
      .iter()
      .map(|state| state.tails.back().unwrap().clone())
      .collect::<HashSet<Position>>()
      .len()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4\n\
                         U 4\n\
                         L 3\n\
                         D 1\n\
                         R 4\n\
                         D 1\n\
                         L 5\n\
                         R 2";

    #[test]
    fn test1() {
        let result: usize = part1(&parse_input(INPUT));
        assert_eq!(result, 13);
    }

    #[test]
    fn test2() {
        let result: usize = part2(&parse_input(INPUT));
        assert_eq!(result, 1);
    }
}