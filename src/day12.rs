use anyhow::Result;
use std::fs;


const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() -> Result<()> {
    let map: Vec<Vec<char>> = parse_input(fs::read_to_string("day12.input")?.as_str());
    println!("{}", part1(&map));
    println!("{}", part2(&map));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.split("\n")
      .map(|line| line.chars().collect::<Vec<char>>())
      .collect()
}

fn part1(map: &Vec<Vec<char>>) -> u32 {
    helper(map, vec!['S'])
}

fn part2(map: &Vec<Vec<char>>) -> u32 {
    helper(map, vec!['S', 'a'])
}

fn next_steps(position: (usize, usize), size: (usize, usize)) -> Vec<(usize, usize)> {
    DIRECTIONS.iter()
        .map(|(x_delta, y_delta)| (position.0 as i32 + x_delta, position.1 as i32 + y_delta))
        .filter(|(x, y)| *x >= 0 && *x < size.0 as i32 && *y >= 0 && *y < size.1 as i32)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn elevation(from: char, to: char) -> i8 {
    let updated_from = if from == 'S' { 'a' } else { from };
    let updated_to = if to == 'E' { 'z' } else { to };
    updated_to as i8 - updated_from as i8
}

fn helper(map: &Vec<Vec<char>>, start_characters: Vec<char>) -> u32 {
    let m = map.len();
    let n = map[0].len();
    let mut steps: Vec<Vec<u32>> = vec![vec![0; n]; m];

    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for x in 0..m {
        for y in 0..n {
            if start_characters.contains(&map[x][y]) {
                candidates.push((x, y));
                steps[x][y] = 1;
            }
        }
    }
    let mut step = 0;
    while !candidates.is_empty() {
        step += 1;
        let mut next: Vec<(usize, usize)> = Vec::new();
        for (x, y) in candidates {
            for (new_x, new_y) in next_steps((x, y), (m, n)) {
                if steps[new_x][new_y] == 0 && elevation(map[x][y], map[new_x][new_y]) <= 1 {
                    steps[new_x][new_y] = steps[x][y] + 1;
                    next.push((new_x, new_y));
                }
            }
        }
        if next.iter().any(|(x, y)| map[*x][*y] == 'E') {
            break;
        }
        candidates = next.clone();
    }
    step
}

// cargo test --bin day10 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 31);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(&INPUT));
        assert_eq!(result, 29);
    }
    
    const INPUT: &str ="Sabqponm\n\
                        abcryxxl\n\
                        accszExk\n\
                        acctuvwj\n\
                        abdefghi";
}