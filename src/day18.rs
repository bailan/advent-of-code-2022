use anyhow::Result;
use std::fs;
use num::abs;
use std::collections::VecDeque;

#[derive(Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn adjacent(&self, other: &Self) -> bool {
        self.distance(other) == 1
    }

    fn distance(&self, other: &Self) -> usize {
        (abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)) as usize
    }

    fn adjacent_cubes(&self) -> Vec<Cube> {
        vec![
            Cube { x: self.x + 1, y: self.y, z: self.z },
            Cube { x: self.x - 1, y: self.y, z: self.z },
            Cube { x: self.x, y: self.y + 1, z: self.z },
            Cube { x: self.x, y: self.y - 1, z: self.z },
            Cube { x: self.x, y: self.y, z: self.z + 1 },
            Cube { x: self.x, y: self.y, z: self.z - 1 },
        ]
    }

    fn translate(&self, offset: (i32, i32, i32)) -> Cube {
        Cube { x: self.x - offset.0, y: self.y - offset.1, z: self.z - offset.2 }
    }
}

fn main() -> Result<()> {
    let cubes: Vec<Cube> = parse_input(fs::read_to_string("day18.input")?.as_str());
    println!("{}", part1(&cubes));
    println!("{}", part2(&cubes));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Cube> {
    input.split("\n").map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Cube {
    let values: Vec<i32> = line.split(",").flat_map(|s| s.parse::<i32>()).collect();
    Cube { x: values[0], y: values[1], z: values[2] }
}

fn part1(cubes: &Vec<Cube>) -> usize {
    let n = cubes.len();
    let mut surface_area = 0;
    for i in 0..n {
        surface_area += 6;
        for j in 0..n {
            if i != j {
                if cubes[i].adjacent(&cubes[j]) {
                    surface_area -= 1;
                }
            }
        }
    }
    surface_area
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Lava,
    External,
    Internal,
}

fn part2(cubes: &Vec<Cube>) -> usize {
    let max_x = cubes.iter().map(|cube| cube.x).max().unwrap();
    let min_x = cubes.iter().map(|cube| cube.x).min().unwrap();
    let max_y = cubes.iter().map(|cube| cube.y).max().unwrap();
    let min_y = cubes.iter().map(|cube| cube.y).min().unwrap();
    let max_z = cubes.iter().map(|cube| cube.z).max().unwrap();
    let min_z = cubes.iter().map(|cube| cube.z).min().unwrap();
    let x_size = (max_x - min_x + 1) as usize;
    let y_size = (max_y - min_y + 1) as usize;
    let z_size = (max_z - min_z + 1) as usize;
    let offset = (min_x, min_y, min_z);
    let mut states: Vec<Vec<Vec<State>>> = vec![vec![vec![State::Internal; z_size]; y_size]; x_size];
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; z_size]; y_size]; x_size];
    for cube in cubes {
        let translated_cube = cube.translate(offset);
        let x = translated_cube.x as usize;
        let y = translated_cube.y as usize;
        let z = translated_cube.z as usize;
        states[x][y][z] = State::Lava;
    }
    let mut queue = VecDeque::new();

    for x in 0..x_size {
        for y in 0..y_size {
            for z in 0..z_size {
                if (x == 0 || y == 0 || z == 0 || x == x_size - 1 || y == y_size - 1 || z == z_size - 1)
                    && states[x][y][z] != State::Lava {
                    states[x][y][z] = State::External;
                    visited[x][y][z] = true;
                    queue.push_back(Cube { x: x as i32, y: y as i32, z: z as i32});
                }
            }
        }
    }
    
    while let Some(current) = queue.pop_front() {
        for cube in current.adjacent_cubes() {
            if cube.x >= 0 && cube.x < x_size as i32 
            && cube.y >= 0 && cube.y < y_size as i32 
            && cube.z >= 0 && cube.z < z_size as i32 {
                let x = cube.x as usize;
                let y = cube.y as usize;
                let z = cube.z as usize;
                if !visited[x][y][z] && states[x][y][z] != State::Lava {
                    states[x][y][z] = State::External;
                    visited[x][y][z] = true;
                    queue.push_back(cube);
                }
            }
        }
    }

    let mut all_cubes: Vec<Cube> = vec![];
    for x in 0..x_size {
        for y in 0..y_size {
            for z in 0..z_size {
                if states[x][y][z] != State::External {
                    all_cubes.push(Cube { x: x as i32, y: y as i32, z: z as i32 })
                }
            }
        }
    }
    part1(&all_cubes)
}

// cargo test --bin day17 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let result = part1(&parse_input("1,1,1\n2,1,1"));
        assert_eq!(result, 10);
    }

    #[test]
    fn test01() {
        let result = part1(&parse_input("1,1,1\n2,1,1\n1,2,1"));
        assert_eq!(result, 14);
    }

    #[test]
    fn test02() {
        let result = part1(&parse_input("2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2"));
        assert_eq!(result, 22);
    }


    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 64);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(&INPUT));
        assert_eq!(result, 58);
    }

    const INPUT: &str ="2,2,2\n\
                        1,2,2\n\
                        3,2,2\n\
                        2,1,2\n\
                        2,3,2\n\
                        2,2,1\n\
                        2,2,3\n\
                        2,2,4\n\
                        2,2,6\n\
                        1,2,5\n\
                        3,2,5\n\
                        2,1,5\n\
                        2,3,5";
}