use anyhow::Result;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
    Entry,
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Material>>,
    entry: (usize, usize),
}

fn main() -> Result<()> {
    let lines: Vec<Vec<(usize, usize)>> = parse_input(fs::read_to_string("day14.input")?.as_str());
    println!("{}", part1(build_map_part1(&lines)));
    println!("{}", part2(build_map_part2(&lines)));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
    input.split("\n")
      .map(|line| parse_line(line))
      .collect()
}

fn parse_line(line: &str) -> Vec<(usize, usize)> {
    line.split(" -> ")
      .filter_map(|pair| pair.split_once(",").map(|(left, right)| (left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap())))
      .collect()
}

fn build_map_part1(lines: &Vec<Vec<(usize, usize)>>) -> Map {
    let bottom: usize = *lines.iter().flatten().map(|(_, y)| y).max().unwrap();
    let left: usize = *lines.iter().flatten().map(|(x, _)| x).min().unwrap();
    let right: usize = *lines.iter().flatten().map(|(x, _)| x).max().unwrap();

    let width = right - left + 3;
    let height = bottom + 2;
    let offset = left - 1;
    let entry_x = 0;
    let entry_y = 500 - offset;
    let mut map = vec![vec![Material::Air; width]; height];
    map[entry_x][entry_y] = Material::Entry;
    for line in lines {
        for points in line.windows(2) {
            if points[0].0 == points[1].0 {
                let y = points[0].0;
                let x_start = std::cmp::min(points[0].1, points[1].1);
                let x_end = std::cmp::max(points[0].1, points[1].1);
                for x in x_start..(x_end + 1) {
                    map[x][y - offset] = Material::Rock;
                }
            } else {
                let x = points[0].1;
                let y_start = std::cmp::min(points[0].0, points[1].0);
                let y_end = std::cmp::max(points[0].0, points[1].0);
                for y in y_start..(y_end + 1) {
                    map[x][y - offset] = Material::Rock;
                }
            }
        }
    }
    Map { map: map, entry: (entry_x, entry_y) }
}

fn print_map(map: &Vec<Vec<Material>>) -> () {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            match map[x][y] {
                Material::Air => print!("."),
                Material::Rock => print!("#"),
                Material::Sand => print!("o"),
                Material::Entry => print!("+"),
            };
        }
        println!("");
    }
}

fn next_sand(map: &Map) -> (usize, usize) {
    let (mut x, mut y) = map.entry.clone();
    loop {
        if x + 1 == map.map.len() {
            break;
        } else if map.map[x + 1][y] == Material::Air {
            x = x + 1;
        } else if map.map[x + 1][y - 1] == Material::Air {
            x = x + 1;
            y = y - 1;
        } else if map.map[x + 1][y + 1] == Material::Air {
            x = x + 1;
            y = y + 1;
        } else {
            break;
        }
    }
    (x, y)
}

fn build_map_part2(lines: &Vec<Vec<(usize, usize)>>) -> Map {
    let bottom: usize = *lines.iter().flatten().map(|(_, y)| y).max().unwrap();
    let left: usize = std::cmp::min(*lines.iter().flatten().map(|(x, _)| x).min().unwrap(), 500 - bottom - 2);
    let right: usize = std::cmp::max(*lines.iter().flatten().map(|(x, _)| x).max().unwrap(), 500 + bottom + 2);

    let width = right - left + 3;
    let height = bottom + 3;
    let offset = left - 1;
    let entry_x = 0;
    let entry_y = 500 - offset;
    let mut map = vec![vec![Material::Air; width]; height];
    for y in 0..width {
        map[height - 1][y] = Material::Rock;
    }
    map[entry_x][entry_y] = Material::Entry;
    for line in lines {
        for points in line.windows(2) {
            if points[0].0 == points[1].0 {
                let y = points[0].0;
                let x_start = std::cmp::min(points[0].1, points[1].1);
                let x_end = std::cmp::max(points[0].1, points[1].1);
                for x in x_start..(x_end + 1) {
                    map[x][y - offset] = Material::Rock;
                }
            } else {
                let x = points[0].1;
                let y_start = std::cmp::min(points[0].0, points[1].0);
                let y_end = std::cmp::max(points[0].0, points[1].0);
                for y in y_start..(y_end + 1) {
                    map[x][y - offset] = Material::Rock;
                }
            }
        }
    }
    Map { map: map, entry: (entry_x, entry_y) }
}

fn part1(input_map: Map) -> usize {
    let mut map = input_map;
    let mut count = 0;
    loop {
        let (next_x, next_y) = next_sand(&map);
        if next_x + 1 == map.map.len() {
            break
        }
        map.map[next_x][next_y] = Material::Sand;
        count += 1;
    }
    count
}

fn part2(input_map: Map) -> usize {
    let mut map = input_map;
    let mut count = 0;
    loop {
        let (next_x, next_y) = next_sand(&map);
        map.map[next_x][next_y] = Material::Sand;
        count += 1;
        if (next_x, next_y) == map.entry {
            break
        }
    }
    count
}

// cargo test --bin day14 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let map = build_map_part1(&parse_input(&INPUT));
        print_map(&map.map);
        let result = part1(map);
        assert_eq!(result, 24);
    }


    #[test]
    fn test2() {
        let map = build_map_part2(&parse_input(&INPUT));
        print_map(&map.map);
        let result = part2(map);
        assert_eq!(result, 93);
    }

    const INPUT: &str ="498,4 -> 498,6 -> 496,6\n\
                        503,4 -> 502,4 -> 502,9 -> 494,9";
}