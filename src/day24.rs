use anyhow::Result;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone)]
enum Ground {
    Blizzard(Vec<Direction>),
    Wall,
}

impl Ground {
    fn contains(&self, dir: &Direction) -> bool {
        match self {
            Ground::Blizzard(dirs) => dirs.contains(dir),
            Ground::Wall => false,
        }
    }

    fn is_clear(&self) -> bool {
        match self {
            Ground::Blizzard(dirs) => dirs.is_empty(),
            Ground::Wall => false,
        }  
    }
}

fn main() -> Result<()> {
    let map: Vec<Vec<Ground>> = parse_input(fs::read_to_string("day24.input")?.as_str());
    println!("{}", part1(&map));
    println!("{}", part2(&map));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<Ground>> {
    input.split("\n")
        .map(|s| s.chars()
            .map(|c| match c {
                '#' => Ground::Wall,
                '.' => Ground::Blizzard(vec![]),
                '>' => Ground::Blizzard(vec![Direction::Right]),
                '<' => Ground::Blizzard(vec![Direction::Left]),
                '^' => Ground::Blizzard(vec![Direction::Up]),
                'v' => Ground::Blizzard(vec![Direction::Down]),
                _ => panic!(),
            })
            .collect())
        .collect()
}

#[warn(dead_code)]
fn print_map(map: &Vec<Vec<Ground>>) -> () {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", match &map[i][j] {
                Ground::Wall => '#',
                Ground::Blizzard(dirs) => {
                    if dirs.len() == 0 {
                        '.'
                    } else if dirs.len() == 1 {
                        match dirs[0] {
                            Direction::Left => '<',
                            Direction::Right => '>',
                            Direction::Up => '^',
                            Direction::Down => 'v',
                        }
                    } else {
                        char::from_digit(dirs.len() as u32, 10).unwrap() 
                    }
                },
            });
        }
        println!();
    }
}

fn next_map(map: &Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    let m = map.len();
    let n = map[0].len();
    let mut new_map = vec![vec![Ground::Wall; n]; m];
    new_map[0][1] = Ground::Blizzard(vec![]);
    new_map[m - 1][n - 2] = Ground::Blizzard(vec![]);
    for i in 1..(m - 1) {
        for j in 1..(n - 1) {
            let mut dirs: Vec<Direction> = Vec::new();
            if map[i - 1][j].contains(&Direction::Down) || (map[i - 1][j] == Ground::Wall && map[m - 2][j].contains(&Direction::Down)) {
                dirs.push(Direction::Down);
            }
            if map[i + 1][j].contains(&Direction::Up) || (map[i + 1][j] == Ground::Wall && map[1][j].contains(&Direction::Up)) {
                dirs.push(Direction::Up);
            }
            if map[i][j - 1].contains(&Direction::Right) || (map[i][j - 1] == Ground::Wall && map[i][n - 2].contains(&Direction::Right)) {
                dirs.push(Direction::Right);
            }
            if map[i][j + 1].contains(&Direction::Left) || (map[i][j + 1] == Ground::Wall && map[i][1].contains(&Direction::Left)) {
                dirs.push(Direction::Left);
            }
            new_map[i][j] = Ground::Blizzard(dirs);
        }
    }
    new_map
}

fn bfs(map: &Vec<Vec<Ground>>, start: (usize, usize), end: (usize, usize)) -> (usize, Vec<Vec<Ground>>) {
    let m = map.len();
    let n = map[0].len();  
    let mut next_round = HashSet::from([start.clone()]);
    let mut minute = 0;
    let mut current_map = map.clone();
    loop {
        minute += 1;
        let current_round = next_round.clone();
        current_map = next_map(&current_map);
        next_round = HashSet::new();
        for (i, j) in current_round {
            if current_map[i][j].is_clear() {
                next_round.insert((i, j));
            }
            if i > 0 && current_map[i - 1][j].is_clear() {
                next_round.insert((i - 1, j));
            }
            if i + 1 < m && current_map[i + 1][j].is_clear() {
                next_round.insert((i + 1, j));
            }
            if j > 0 && current_map[i][j - 1].is_clear() {
                next_round.insert((i, j - 1));
            }
            if j + 1 < n && current_map[i][j + 1].is_clear() {
                next_round.insert((i, j + 1));
            }
        }
        if next_round.iter().any(|(i, j)| (*i, *j) == end) {
            break;
        }
    }
    (minute, current_map)
}

fn part1(map: &Vec<Vec<Ground>>) -> usize {
    bfs(map, (0, 1), (map.len() - 1, map[0].len() - 2)).0
}

fn part2(map: &Vec<Vec<Ground>>) -> usize {
    let (time1, map1) = bfs(map, (0, 1), (map.len() - 1, map[0].len() - 2));
    let (time2, map2) = bfs(&map1, (map1.len() - 1, map1[0].len() - 2), (0, 1));
    let (time3, _) = bfs(&map2, (0, 1), (map2.len() - 1, map2[0].len() - 2));
    time1 + time2 + time3
}

// cargo test --bin day24 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 18);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(&INPUT));
        assert_eq!(result, 54);
    }

    const INPUT: &str ="#.######\n\
                        #>>.<^<#\n\
                        #.<..<<#\n\
                        #>v.><>#\n\
                        #<^v^^>#\n\
                        ######.#";                
}