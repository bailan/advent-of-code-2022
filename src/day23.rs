use anyhow::Result;
use std::fs;
use std::collections::VecDeque;
use std::cmp::min;
use std::cmp::max;

#[derive(Clone, PartialEq)]
enum State {
    Empty,
    Elf,
}

#[derive(Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, PartialEq)]
enum Propose {
    Empty,
    Stay,
    Move(Direction),
}

fn main() -> Result<()> {
    let map = parse_input(fs::read_to_string("day23.input")?.as_str());
    println!("{}", part1(&map));
    println!("{}", part2(&map));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<State>> {
    let raw_map: Vec<Vec<char>> = input.split("\n")
        .map(|s| s.chars().collect::<Vec<char>>()).collect();
    let m = raw_map.len();
    let n = raw_map[0].len();
    let mut map = vec![vec![State::Empty; n + 2]; m + 2];
    for i in 0..m {
        for j in 0..n {
            if raw_map[i][j] == '#' {
                map[i + 1][j + 1] = State::Elf;
            }
        }
    }
    map
}

fn all_directions_empty(map: &Vec<Vec<State>>, i: usize, j: usize) -> bool {
    map[i - 1][j + 1] == State::Empty 
        && map[i][j + 1] == State::Empty 
        && map[i + 1][j + 1] == State::Empty
        && map[i - 1][j] == State::Empty 
        && map[i + 1][j] == State::Empty
        && map[i - 1][j - 1] == State::Empty 
        && map[i][j - 1] == State::Empty 
        && map[i + 1][j - 1] == State::Empty
}

fn direction_empty(map: &Vec<Vec<State>>, i: usize, j: usize, dir: &Direction) -> bool {
    match dir {
        Direction::North => map[i - 1][j - 1] == State::Empty && map[i - 1][j] == State::Empty && map[i - 1][j + 1] == State::Empty,
        Direction::South => map[i + 1][j - 1] == State::Empty && map[i + 1][j] == State::Empty && map[i + 1][j + 1] == State::Empty,
        Direction::West => map[i - 1][j - 1] == State::Empty && map[i][j - 1] == State::Empty && map[i + 1][j - 1] == State::Empty,
        Direction::East => map[i - 1][j + 1] == State::Empty && map[i][j + 1] == State::Empty && map[i + 1][j + 1] == State::Empty,
    }
}

fn propose(map: &Vec<Vec<State>>, directions: &VecDeque<Direction>) -> Vec<Vec<Propose>> {
    let m = map.len();
    let n = map[0].len();
    let mut proposed_map = vec![vec![Propose::Empty; n]; m];
    for i in 1..(m-1) {
        for j in 1..(n-1) {
            match map[i][j] {
                State::Empty => (),
                State::Elf => {
                    if all_directions_empty(map, i, j) {
                        proposed_map[i][j] = Propose::Stay;
                    } else {
                        proposed_map[i][j] = Propose::Stay;
                        for dir in directions {
                            if direction_empty(map, i, j, dir) {
                                proposed_map[i][j] = Propose::Move(dir.clone());
                                break;
                            }
                        }
                    }
                },
            }
        }
    }
    proposed_map
}

fn moves(map: &Vec<Vec<Propose>>) -> Vec<Vec<State>> {
    let m = map.len();
    let n = map[0].len();
    let mut moved_map = vec![vec![State::Empty; n]; m];
    for i in 0..m {
        for j in 0..n {
            match map[i][j] {
                Propose::Empty => {
                    let mut candidates: Vec<(usize, usize)> = Vec::new();
                    if i + 1 < m && map[i + 1][j] == Propose::Move(Direction::North) {
                        candidates.push((i + 1, j));
                    }
                    if i > 0 && map[i - 1][j] == Propose::Move(Direction::South) {
                        candidates.push((i - 1, j));
                    }
                    if j + 1 < n && map[i][j + 1] == Propose::Move(Direction::West) {
                        candidates.push((i, j + 1));
                    }
                    if j > 0 && map[i][j - 1] == Propose::Move(Direction::East) {
                        candidates.push((i, j - 1));
                    }
                    if candidates.len() == 1 {
                        moved_map[i][j] = State::Elf;
                    } else if candidates.len() > 1 {
                        for candidate in candidates {
                            moved_map[candidate.0][candidate.1] = State::Elf;
                        }
                    }
                },
                Propose::Stay => moved_map[i][j] = State::Elf,
                Propose::Move(_) => (),
            }
        }
    }
    moved_map
}

fn resize(map: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let m = map.len();
    let n = map[0].len();
    let mut min_i = m;
    let mut max_i = 0;
    let mut min_j = n;
    let mut max_j = 0;
    for i in 0..m {
        for j in 0..n {
            match map[i][j] {
                State::Empty => (),
                State::Elf => {
                    min_i = min(min_i, i);
                    max_i = max(max_i, i);
                    min_j = min(min_j, j);
                    max_j = max(max_j, j);
                },
            };
        }
    }

    let mut resized_map = vec![vec![State::Empty; max_j - min_j + 3]; max_i - min_i + 3];
    for i in min_i..(max_i + 1) {
        for j in min_j..(max_j + 1) {
            resized_map[i + 1 - min_i][j + 1 - min_j] = map[i][j].clone();
        }
    }
    resized_map
}

fn score(map: &Vec<Vec<State>>) -> usize {
    let height = map.len() - 2;
    let width = map[0].len() - 2;
    let number_elves: usize = map.iter().map(|row| row.iter().filter(|state| **state == State::Elf).count()).sum();
    height * width - number_elves
}

fn part1(map: &Vec<Vec<State>>) -> usize {
    let mut directions = VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]); 
    let mut current_map = map.clone();
    for _ in 0..10 {
        let proposed_map = propose(&current_map, &directions);
        let moved_map = moves(&proposed_map);
        current_map = resize(&moved_map);
        directions.rotate_left(1);
    }
    score(&current_map)
}

fn same(map1: &Vec<Vec<State>>, map2: &Vec<Vec<State>>) -> bool {
    if map1.len() != map2.len() {
        return false;
    }
    if map1[0].len() != map2[0].len() {
        return false;
    }
    for i in 0..map1.len() {
        for j in 0..map1[i].len() {
            if map1[i][j] != map2[i][j] {
                return false;
            }
        }
    }
    true
}

fn part2(map: &Vec<Vec<State>>) -> usize {
    let mut round = 0;
    let mut directions = VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]); 
    let mut current_map = map.clone();
    loop {
        round += 1;
        let proposed_map = propose(&current_map, &directions);
        let moved_map = moves(&proposed_map);
        let final_map = resize(&moved_map);
        if same(&current_map, &final_map) {
            break;
        }
        current_map = final_map;
        directions.rotate_left(1);
    }
    round
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<State>>) -> () {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                State::Empty => print!("."),
                State::Elf => print!("#"),
            };
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_proposed_map(map: &Vec<Vec<Propose>>) -> () {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match &map[i][j] {
                Propose::Empty => print!("."),
                Propose::Stay => print!("#"),
                Propose::Move(dir) => match dir {
                    Direction::North => print!("^"),
                    Direction::South => print!("v"),
                    Direction::West => print!("<"),
                    Direction::East => print!(">"),
                },
            };
        }
        println!();
    }
}

// cargo test --bin day23 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 110);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(&INPUT));
        assert_eq!(result, 20);
    }

    const INPUT: &str ="....#..\n\
                        ..###.#\n\
                        #...#.#\n\
                        .#...##\n\
                        #.###..\n\
                        ##.#.##\n\
                        .#..#..";                
}