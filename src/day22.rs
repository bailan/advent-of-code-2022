use anyhow::Result;
use std::fs;


struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn width_include_boundary(&self) -> usize {
        self.map[0].len()
    }

    fn height_include_boundary(&self) -> usize {
        self.map.len()
    }

    fn first_tile_on_row(&self, row: usize) -> usize {
        for y in 0..self.width_include_boundary() {
            if self.map[row][y] != Tile::Outside {
                return y;
            }
        }
        panic!();
    }

    fn last_tile_on_row(&self, row: usize) -> usize {
        for y in (0..self.width_include_boundary()).rev() {
            if self.map[row][y] != Tile::Outside {
                return y;
            }
        }
        panic!();
    }

    fn first_tile_on_column(&self, column: usize) -> usize {
        for x in 0..self.height_include_boundary() {
            if self.map[x][column] != Tile::Outside {
                return x;
            }
        }
        panic!();
    }

    fn last_tile_on_column(&self, column: usize) -> usize {
        for x in (0..self.height_include_boundary()).rev() {
            if self.map[x][column] != Tile::Outside {
                return x;
            }
        }
        panic!();
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Open,
    Wall,
    Outside,
}

#[derive(Debug)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Rotation {
    fn rotate(&self, dir: &Direction) -> Direction {
        match self {
            Rotation::Clockwise => 
                match dir {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,  
                },
            Rotation::CounterClockwise => 
                match dir {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,  
                },
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Rotate(Rotation),
    Forward(usize),
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn move_from(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
            Direction::Up => (position.0 - 1, position.1),
            Direction::Down => (position.0 + 1, position.1),  
        }
    }

    fn number(&self) -> usize {
        match self {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,  
        }  
    }
}

fn main() -> Result<()> {
    let (map, path): (Map, Vec<Instruction>) = parse_input(fs::read_to_string("day22.input")?.as_str());
    println!("{}", part1(&map, &path));
    println!("{}", part1(&map, &path));
    Ok(())
}

fn parse_input(input: &str) -> (Map, Vec<Instruction>) {
    input.split_once("\n\n")
        .map(|(map_str, path_str)| (parse_map(map_str), parse_instruction(path_str)))
        .unwrap()
}

fn parse_map(s: &str) -> Map {
    let raw_map: Vec<Vec<char>> = s.split("\n").map(|row| row.chars().collect::<Vec<char>>()).collect();
    let n = raw_map.iter().map(|v| v.len()).max().unwrap() + 2;
    let m = raw_map.len() + 2;
    let mut map = vec![vec![Tile::Outside; n]; m];
    for i in 1..(m - 1) {
        for j in 1..(raw_map[i - 1].len() + 1) {
            match raw_map[i - 1][j - 1] {
                '.' => map[i][j] = Tile::Open,
                '#' => map[i][j] = Tile::Wall,
                ' ' => map[i][j] = Tile::Outside,
                _ => panic!(),
            }
        }
    }
    Map { map }
}

fn parse_instruction(s: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut number: usize = 0;
    for ch in s.chars() {
        match ch {
            'R' => {
                instructions.push(Instruction::Forward(number)); 
                instructions.push(Instruction::Rotate(Rotation::Clockwise));
                number = 0;
            },
            'L' => {
                instructions.push(Instruction::Forward(number)); 
                instructions.push(Instruction::Rotate(Rotation::CounterClockwise));
                number = 0;
            },
            d => number = number * 10 + d as usize - '0' as usize,
        }
    }
    instructions.push(Instruction::Forward(number));
    instructions
}

fn print_map(map: &Map) -> () {
    let m = map.map.len(); 
    let n = map.map[0].len();
    for i in 0..m {
        for j in 0..n {
            match map.map[i][j] {
                Tile::Open => print!("."),
                Tile::Wall => print!("#"),
                Tile::Outside => print!(" "),
            }
        }
        println!();
    }  
}

fn part1(map: &Map, instructions: &Vec<Instruction>) -> usize {
    print_map(map);
    let (start_x, start_y, start_dir) = (1, map.first_tile_on_row(1), Direction::Right);
    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut current_dir = start_dir;
    for instruction in instructions {
        match instruction {
            Instruction::Forward(steps) => {
                for _ in 0..*steps {
                    let (mut next_x, mut next_y) = current_dir.move_from((current_x, current_y));
                    if map.map[next_x][next_y] == Tile::Outside {
                        match current_dir {
                            Direction::Left => {next_y = map.last_tile_on_row(next_x);},
                            Direction::Right => {next_y = map.first_tile_on_row(next_x);},
                            Direction::Up => {next_x = map.last_tile_on_column(next_y);},
                            Direction::Down => {next_x = map.first_tile_on_column(next_y);},  
                        }
                    }
                    if map.map[next_x][next_y] == Tile::Wall {
                        break;
                    } else {
                        current_x = next_x;
                        current_y = next_y;
                    }
                };
            },
            Instruction::Rotate(rotation) => {
                current_dir = rotation.rotate(&current_dir);
            }
        }
    }
    1000 * current_x + 4 * current_y + current_dir.number()
}

fn part2(map: &Map, instructions: &Vec<Instruction>) -> usize {
    0
}

// cargo test --bin 22 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test1() {
        let (map, path) = parse_input(&INPUT);
        let result = part1(&map, &path);
        assert_eq!(result, 6032);
    }

    #[ignore]
    #[test]
    fn test2() {
        let (map, path) = parse_input(&INPUT);
        let result = part2(&map, &path);
        assert_eq!(result, 0);
    }

    const INPUT: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
}