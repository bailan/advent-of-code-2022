use anyhow::Result;
use std::fs;
use std::cmp::min;
use std::cmp::max;

use State::*;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Empty,
    Stay,
    Move(u16),
}

fn main() -> Result<()> {
    println!("{}", part1(&mut parse_input(fs::read_to_string("day23.input")?.as_str())));
    println!("{}", part2(&mut parse_input(fs::read_to_string("day23.input")?.as_str())));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<State>> {
    let raw_map: Vec<Vec<char>> = input.split("\n")
        .map(|s| s.chars().collect::<Vec<char>>()).collect();
    let n = raw_map[0].len();
    let m = raw_map.len();
    let mut map = vec![vec![Empty; n + 2]; m + 2];
    for i in 0..m {
        for j in 0..n {
            if raw_map[i][j] == '#' {
                map[i + 1][j + 1] = Stay;
            }
        }
    }
    map
}

fn part1(map: &Vec<Vec<State>>) -> usize {
    for round in 0..10 {
        let m = map.len();
        let n = map[0].len();
        let mut current_map = next_map;

        for i in 1..(m-1) {
            for j in 1..(n-1) {
                match current_map[i][j] {
                    Empty, Move(_) => (),
                    Stay => {
                        if !(all_directions_empty(current_map, i, j)) {
                            for k in 0..4 {
                                if direction_empty(current_map, i, j, (round + k) % 4) {
                                    current_map[i][j] = Move((round + k) % 4);
                                    break;
                                }
                            }
                        }
                    },
                }
            }
        }

        for i in 1..(m-1) {
            for j in 1..(n-1) {
                match current_map[i][j] {
                    Empty => {
                        let mut candidates: Vec<(usize, usize)> = Vec::new();
                        if current_map[i + 1][j] == Move(0) {
                            candidates.push((i + 1, j));
                        }
                        if current_map[i - 1][j] == Move(1) {
                            candidates.push((i - 1, j));
                        }
                        if current_map[i][j + 1] == Move(2) {
                            candidates.push((i, j + 1));
                        }
                        if current_map[i][j - 1] == Move(3) {
                            candidates.push((i, j - 1));
                        }
                        if candidates.len() == 1 {
                            map[i][j] = Stay;
                            map[candidates[0].0][candidates[0].1] = Empty;
                        } else if candidates.len() > 1 {
                            for candidate in candidates {
                                current_map[candidate.0][candidate.1] = Stay;
                            }
                        }
                    },
                    Stay, Move(_) => (),
                }
            }
        }
        next = resize(&current_map);
    }
    score(map)
}

fn all_directions_empty(map: &Vec<Vec<State>>, i: usize, j: usize) -> bool {
    map[i - 1][j + 1] == Empty && map[i][j + 1] == Empty && map[i + 1][j + 1] == Empty
        && map[i - 1][j] == Empty && map[i + 1][j] == Empty
        && map[i - 1][j - 1] == Empty && map[i][j - 1] == Empty && map[i + 1][j - 1] == Empty
}

fn direction_empty(map: &Vec<Vec<State>>, i: usize, j: usize, d: u16) -> bool {
    match d {
        0 => map[i - 1][j - 1] == Empty && map[i - 1][j] == Empty && map[i - 1][j + 1] == Empty,
        1 => map[i + 1][j - 1] == Empty && map[i + 1][j] == Empty && map[i + 1][j + 1] == Empty,
        2 => map[i - 1][j - 1] == Empty && map[i][j - 1] == Empty && map[i + 1][j - 1] == Empty,
        3 => map[i - 1][j + 1] == Empty && map[i][j + 1] == Empty && map[i + 1][j + 1] == Empty,
        _ => panic!()
    }
}

fn resize(map: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let m = map.len();
    let n = map[0].len();

    let mut min_i = m;
    let mut max_i = 0;
    let mut min_j = n;
    let mut max_j = 0;
    let mut count = 0; 
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                Empty => (),
                Stay => {
                    min_i = min(min_i, i);
                    max_i = max(max_i, i);
                    min_j = min(min_j, j);
                    max_j = max(max_j, j);
                    count += 1;
                },
                Move(_) => (),
            };
        }
    }
    (max_i - min_i + 1) * (max_j - min_j + 1) - count
}

fn part2(map: &mut Vec<Vec<State>>) -> usize {
    let m = map.len();
    let n = map[0].len();
    let mut round = 0;
    loop {
        // println!("round start");
        for i in 1..(m-1) {
            for j in 1..(n-1) {
                match map[i][j] {
                    Empty => (),
                    Stay => {
                        if !(all_directions_empty(map, i, j)) {
                            for k in 0..4 {
                                if direction_empty(map, i, j, (round + k) % 4) {
                                    map[i][j] = Move((round + k) % 4);
                                    break;
                                }
                            }
                        }
                    },
                    Move(_) => (),
                }
            }
        }

        round += 1;
        let mut moved = false;
        for i in 1..(m-1) {
            for j in 1..(n-1) {
                match map[i][j] {
                    Empty => {
                        let mut candidates: Vec<(usize, usize)> = Vec::new();
                        if map[i + 1][j] == Move(0) {
                            candidates.push((i + 1, j));
                        }
                        if map[i - 1][j] == Move(1) {
                            candidates.push((i - 1, j));
                        }
                        if map[i][j + 1] == Move(2) {
                            candidates.push((i, j + 1));
                        }
                        if map[i][j - 1] == Move(3) {
                            candidates.push((i, j - 1));
                        }
                        if candidates.len() == 1 {
                            map[i][j] = Stay;
                            map[candidates[0].0][candidates[0].1] = Empty;
                            moved = true;
                        } else if candidates.len() > 1 {
                            for candidate in candidates {
                                map[candidate.0][candidate.1] = Stay;
                            }
                        }
                    },
                    Stay => (),
                    Move(_) => (),
                }
            }
        }
        if !moved {
            break;
        }
    }
    print_map(map);
    round as usize
}

fn print_map(map: &Vec<Vec<State>>) -> () {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                Empty => print!("."),
                Stay => print!("#", ),
                Move(i) => print!("{}", 4),
            };
        }
        println!();
    }
}


// cargo test --bin 21 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test1() {
        let result = part1(&mut parse_input(&INPUT));
        assert_eq!(result, 110);
    }

    #[test]
    fn test2() {
        let result = part2(&mut parse_input(&INPUT));
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